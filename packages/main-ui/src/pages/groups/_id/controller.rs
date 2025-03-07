use by_macros::DioxusController;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::{Group, GroupMemberV2, Role};

use crate::{
    routes::Route,
    service::{login_service::LoginService, popup_service::PopupService},
};

use super::{components::*, i18n::GroupDetailTranslate};

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    pub group: Resource<Group>,
    pub id: ReadOnlySignal<i64>,
    pub lang: Language,

    popup_service: PopupService,
    user_service: LoginService,
    nav: Navigator,
}

impl Controller {
    pub fn new(
        lang: dioxus_translate::Language,
        id: ReadOnlySignal<i64>,
    ) -> std::result::Result<Self, RenderError> {
        let user_service: LoginService = use_context();

        let group = use_server_future(move || {
            let id = id();
            let org_id = user_service.org_id();

            async move {
                let endpoint = crate::config::get().api_url;
                match Group::get_client(endpoint).get(org_id, id).await {
                    Ok(res) => res,
                    Err(e) => {
                        btracing::error!("{}", e.translate(&lang));
                        Default::default()
                    }
                }
            }
        })?;

        let ctrl = Self {
            group,
            id,
            lang,
            nav: use_navigator(),
            user_service,
            // roles: use_signal(|| {
            //     vec![
            //         RoleField {
            //             db_name: "super_admin".to_string(),
            //             field: translates.manager.to_string(),
            //         },
            //         RoleField {
            //             db_name: "public_admin".to_string(),
            //             field: translates.public_opinion_manager.to_string(),
            //         },
            //         RoleField {
            //             db_name: "analyst".to_string(),
            //             field: translates.analyst.to_string(),
            //         },
            //         RoleField {
            //             db_name: "mediator".to_string(),
            //             field: translates.repeater.to_string(),
            //         },
            //         RoleField {
            //             db_name: "speaker".to_string(),
            //             field: translates.lecturer.to_string(),
            //         },
            //     ]
            // }),
            popup_service: use_context(),
        };

        Ok(ctrl)
    }

    pub async fn remove_group(&mut self) {
        let endpoint = crate::config::get().api_url;
        let org_id = self.user_service.org_id();

        match Group::get_client(endpoint).delete(org_id, self.id()).await {
            Ok(_) => {
                if self.nav.can_go_back() {
                    self.nav.go_back();
                } else {
                    self.nav.replace(Route::GroupPage { lang: self.lang });
                }
            }
            Err(e) => {
                btracing::error!("{}", e.translate(&self.lang));
            }
        };
    }

    pub async fn update_group_name(&mut self, name: String) {
        let endpoint = crate::config::get().api_url;
        let org_id = self.user_service.org_id();

        match Group::get_client(endpoint)
            .update(org_id, self.id(), name)
            .await
        {
            Ok(_) => {
                self.group.restart();
            }
            Err(e) => {
                btracing::error!("{}", e.translate(&self.lang));
            }
        };
    }

    pub async fn update_role(&mut self, _member_id: i64, _role: Role) {
        // TODO: implement it
        btracing::error!("update_role is not implemented yet");
    }

    pub async fn open_update_group_name_modal(&mut self) {
        let translates: GroupDetailTranslate = translate(&self.lang);
        let mut ctrl = *self;

        self.popup_service
            .open(rsx! {
                UpdateGroupNameModal {
                    lang: self.lang,
                    onclose: move |_e: MouseEvent| {
                        ctrl.popup_service.close();
                    },
                    initialize_group_name: self.group().unwrap_or_default().name.clone(),
                    update_group_name: move |group_name: String| async move {
                        ctrl.update_group_name(group_name).await;
                        ctrl.popup_service.close();
                    },
                }
            })
            .with_id("update_group_name")
            .with_title(translates.update_group_name);
    }

    pub async fn open_remove_group_modal(&mut self) {
        let translates: GroupDetailTranslate = translate(&self.lang);
        let mut ctrl = *self;

        self.popup_service
            .open(rsx! {
                RemoveGroupModal {
                    lang: self.lang,
                    remove_group: move |_e: MouseEvent| async move {
                        ctrl.remove_group().await;
                    },
                    onclose: move |_e: MouseEvent| {
                        ctrl.popup_service.close();
                    },
                }
            })
            .with_id("remove_group")
            .with_title(translates.remove_group);
    }

    pub async fn open_remove_member_modal(&mut self, user_id: i64) {
        let translates: GroupDetailTranslate = translate(&self.lang);
        let mut ctrl = *self;
        let org_id = self.user_service.org_id();

        self.popup_service
            .open(rsx! {
                RemoveMemberModal {
                    lang: self.lang,
                    onremove: move |_e: MouseEvent| async move {
                        let endpoint = crate::config::get().api_url;
                        match GroupMemberV2::get_client(endpoint)
                            .remove_member(org_id, ctrl.id(), user_id)
                            .await
                        {
                            Ok(_) => {
                                ctrl.group.restart();
                                ctrl.popup_service.close();
                            }
                            Err(e) => {
                                btracing::error!("{}", e.translate(& ctrl.lang));
                            }
                        };
                    },
                    onclose: move |_| {
                        ctrl.popup_service.close();
                    },
                }
            })
            .with_id("remove_team_member")
            .with_title(translates.remove_team_member);
    }

    pub async fn open_add_member_modal(&mut self) {
        let translates: GroupDetailTranslate = translate(&self.lang);
        let group = self.group().unwrap_or_default();
        let mut ctrl = *self;
        let org_id = self.user_service.org_id();

        self.popup_service
            .open(rsx! {
                AddMemberModal {
                    lang: self.lang,
                    group_id: group.id,
                    group_name: group.name,
                    onclose: move |_| {
                        ctrl.popup_service.close();
                    },
                    onadd: move |user_id| async move {
                        let endpoint = crate::config::get().api_url;
                        match GroupMemberV2::get_client(endpoint)
                            .create(org_id, ctrl.id(), user_id)
                            .await
                        {
                            Ok(_) => {
                                ctrl.group.restart();
                                ctrl.popup_service.close();
                            }
                            Err(e) => {
                                btracing::error!("{}", e.translate(& ctrl.lang));
                            }
                        };
                        ctrl.popup_service.close();
                    },
                }
            })
            .with_id("add_team_member")
            .with_title(translates.add_team_member);
    }

    pub async fn open_remove_project_modal(&self, _project_id: i64) {
        // TODO: implement
        btracing::error!("open_remove_project_modal is not implemented yet");
        // let translates: GroupDetailTranslate = translate(&self.lang);
        // let group = self.group().unwrap_or_default();
        // let mut ctrl = *self;
        // let org_id = self.user_service.org_id();

        // popup_service
        //     .open(rsx! {
        //         RemoveProjectModal {
        //             lang,
        //             onremove: move |_e: MouseEvent| {
        //                 let group_id = group_id.clone();
        //                 let project_id = project_id.clone();
        //                 async move {
        //                     tracing::debug!("on remove clicked: {} {}", group_id, project_id);
        //                     popup_service.close();
        //                 }
        //             },
        //             onclose: move |_e: MouseEvent| {
        //                 popup_service.close();
        //             },
        //         }
        //     })
        //     .with_id("remove_project")
        //     .with_title(translates.remove_project);
    }
}
