use bdk::prelude::*;

use by_macros::DioxusController;

use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::{Group, GroupMemberV2, GroupQuery, GroupSummary, QueryResponse};

use crate::service::{login_service::LoginService, popup_service::PopupService};

use super::components::*;

use super::{
    i18n::GroupTranslate,
    page::{RemoveGroupModal, UpdateGroupNameModal},
};

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    pub groups: Resource<QueryResponse<GroupSummary>>,
    popup_service: PopupService,
    user_service: LoginService,
    pub page: Signal<usize>,
    pub lang: Language,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let page = use_signal(|| 1);
        let user_service: LoginService = use_context();

        let groups: Resource<QueryResponse<GroupSummary>> = use_server_future(move || {
            let page = page();
            let org_id = user_service.org_id();
            let size = 10;

            async move {
                let endpoint = crate::config::get().api_url;
                match Group::get_client(endpoint)
                    .query(org_id, GroupQuery::new(size).with_page(page))
                    .await
                {
                    Ok(res) => res,
                    Err(e) => {
                        btracing::error!("{}", e.translate(&lang));
                        Default::default()
                    }
                }
            }
        })?;

        let ctrl = Self {
            lang,
            page,
            groups,
            popup_service: use_context(),
            user_service,
        };

        use_context_provider(|| ctrl);

        Ok(ctrl)
    }

    pub async fn remove_group_member(&mut self, group_id: i64, member_id: i64) {
        let endpoint = crate::config::get().api_url;
        let org_id = self.user_service.org_id();

        if let Err(e) = GroupMemberV2::get_client(endpoint)
            .remove_member(org_id, group_id, member_id)
            .await
        {
            btracing::error!("{}", e.translate(&self.lang));
        }
    }

    pub async fn create_group(&mut self, name: String) {
        let endpoint = crate::config::get().api_url;

        if let Err(e) = Group::get_client(endpoint)
            .create(self.user_service.org_id(), name)
            .await
        {
            btracing::error!("{}", e.translate(&self.lang));
        } else {
            self.groups.restart();
        }
    }

    pub async fn invite_team_member(
        &mut self,
        group_id: String,
        email: String,
        name: Option<String>,
    ) {
        tracing::debug!("invite team member: {email} {group_id} {name:?}");
        // let api: GroupApi = use_context();
        // match api
        //     .add_team_member(
        //         group_id,
        //         TeamMemberRequest {
        //             email,
        //             name,
        //             group: None,
        //             role: None,
        //         },
        //     )
        //     .await
        // {
        //     Ok(_) => {
        //         self.group_resource.restart();
        //         // self.member_resource.restart()
        //     }
        //     Err(e) => {
        //         tracing::error!("failed to invite team member: {e}");
        //     }
        // };
    }

    pub async fn remove_group(&mut self, group_id: i64) {
        let endpoint = crate::config::get().api_url;
        match Group::get_client(endpoint)
            .delete(self.user_service.org_id(), group_id)
            .await
        {
            Ok(_) => {
                self.groups.restart();
            }
            Err(e) => {
                btracing::error!("{}", e.translate(&self.lang));
            }
        }
    }

    pub async fn update_group_name(&mut self, id: i64, name: String) {
        let endpoint = crate::config::get().api_url;
        match Group::get_client(endpoint)
            .update(self.user_service.org_id(), id, name)
            .await
        {
            Ok(_) => {
                self.groups.restart();
            }
            Err(e) => {
                btracing::error!("{}", e.translate(&self.lang));
            }
        }
    }

    pub async fn open_update_group_name_modal(&mut self, id: i64) {
        let lang = self.lang;
        let translates: GroupTranslate = translate(&lang);
        let mut ctrl = *self;
        let org_id = ctrl.user_service.org_id();

        // FIXME: It should be loaded in update group name modal component
        let name = ctrl
            .groups()
            .unwrap_or_default()
            .items
            .iter()
            .find(|g| g.id == id)
            .unwrap()
            .name
            .clone();

        self.popup_service
            .open(rsx! {
                UpdateGroupNameModal {
                    lang,
                    onclose: move |_e: MouseEvent| {
                        ctrl.popup_service.close();
                    },
                    initialize_group_name: name,
                    update_group_name: move |name: String| async move {
                        let endpoint = crate::config::get().api_url;
                        match Group::get_client(endpoint).update(org_id, id, name).await {
                            Ok(_) => ctrl.groups.restart(),
                            Err(e) => {
                                btracing::error!("{}", e.translate(& lang));
                            }
                        };
                        ctrl.popup_service.close();
                    },
                }
            })
            .with_id("update_group")
            .with_title(translates.update_group_name);
    }

    pub async fn open_remove_group_modal(&mut self, id: i64) {
        let lang = self.lang;
        let translates: GroupTranslate = translate(&lang);
        let mut ctrl = *self;
        let org_id = ctrl.user_service.org_id();

        self.popup_service
            .open(rsx! {
                RemoveGroupModal {
                    lang,
                    onclose: move |_e: MouseEvent| {
                        ctrl.popup_service.close();
                    },
                    remove_group: move |_e: Event<MouseData>| async move {
                        let endpoint = crate::config::get().api_url;
                        match Group::get_client(endpoint).delete(org_id, id).await {
                            Ok(_) => ctrl.groups.restart(),
                            Err(e) => {
                                btracing::error!("{}", e.translate(& lang));
                            }
                        };
                        ctrl.popup_service.close();
                    },
                }
            })
            .with_id("remove_group")
            .with_title(translates.remove_group);
    }

    pub async fn open_create_group_modal(&mut self) {
        let lang = self.lang;
        let translates: GroupTranslate = translate(&lang);
        let mut ctrl = *self;
        let org_id = ctrl.user_service.org_id();

        self.popup_service
            .open(rsx! {
                CreateGroupModal {
                    lang,
                    org_id,
                    // FIXME: use user_ids
                    oncreate: move |(name, _user_ids)| async move {
                        let endpoint = crate::config::get().api_url;
                        match Group::get_client(endpoint).create(org_id, name).await {
                            Ok(_) => ctrl.groups.restart(),
                            Err(e) => {
                                btracing::error!("{}", e.translate(& lang));
                            }
                        };
                        ctrl.popup_service.close();
                    },
                    onclose: move |_e: MouseEvent| {
                        ctrl.popup_service.close();
                    },
                }
            })
            .with_id("create_group")
            .with_title(translates.create_group);
    }
}
