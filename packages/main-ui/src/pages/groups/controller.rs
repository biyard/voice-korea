use dioxus::prelude::*;

use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::prelude::{CreateGroupRequest, MemberSummary, TeamMemberRequest};

use crate::service::{group_api::GroupApi, popup_service::PopupService};

use super::{
    i18n::GroupTranslate,
    page::{CreateGroupModal, RemoveGroupModal, UpdateGroupNameModal},
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GroupMemberSummary {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl From<&models::prelude::GroupMemberResponse> for GroupMemberSummary {
    fn from(response: &models::prelude::GroupMemberResponse) -> Self {
        GroupMemberSummary {
            id: response.id.clone(),
            name: response.user_name.clone(),
            email: response.user_email.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GroupSummary {
    pub group_id: String,
    pub group_name: String,
    pub member_count: u64,
    pub member_list: Vec<GroupMemberSummary>,
}

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    pub members: Signal<Vec<MemberSummary>>,
    pub groups: Signal<Vec<GroupSummary>>,
    pub group_resource: Resource<
        Result<
            crate::api::common::CommonQueryResponse<models::prelude::GroupResponse>,
            ServerFnError,
        >,
    >,
    popup_service: Signal<PopupService>,
    group_api: GroupApi,
}

impl Controller {
    pub fn init(_lang: Language, popup_service: PopupService) -> Self {
        let api: GroupApi = use_context();
        let group_resource: Resource<
            Result<
                crate::api::common::CommonQueryResponse<models::prelude::GroupResponse>,
                ServerFnError,
            >,
        > = use_resource(move || {
            let api = api.clone();
            async move { api.list_groups(Some(100), None).await }
        });

        let ctrl = Self {
            groups: use_signal(|| vec![]),
            group_resource,
            popup_service: use_signal(|| popup_service),
            group_api: api,

            members: use_signal(|| vec![]),
        };

        // let groups = if let Some(v) = group_resource.value()() {
        //     match v {
        //         Ok(d) => d
        //             .items
        //             .iter()
        //             .map(|group| GroupSummary {
        //                 group_id: group.id.clone(),
        //                 group_name: group.name.clone(),
        //                 member_count: group.members.len() as u64,
        //                 member_list: group
        //                     .members
        //                     .iter()
        //                     .map(|v: &models::prelude::GroupMemberResponse| v.into())
        //                     .collect(),
        //             })
        //             .collect(),
        //         Err(e) => {
        //             tracing::error!("Failed to fetch groups: {:?}", e);
        //             vec![]
        //         }
        //     }
        // } else {
        //     vec![]
        // };

        // ctrl.groups.set(groups);
        // ctrl.members.set(members);
        use_context_provider(|| ctrl);

        ctrl
    }

    pub fn get_members(&self) -> Vec<MemberSummary> {
        tracing::debug!("total members: {:?}", (self.members)());
        (self.members)()
    }

    pub fn get_groups(&self) -> Vec<GroupSummary> {
        tracing::debug!("total groups: {:?}", (self.groups)());
        (self.groups)()
    }

    pub async fn remove_group_member(&mut self, group_id: String, member_id: String) {
        let api: GroupApi = use_context();
        let mut group_resource = self.group_resource;

        match api.remove_team_member(group_id, member_id).await {
            Ok(_) => {
                group_resource.restart();
            }
            Err(e) => {
                tracing::error!("failed to remove team member: {e}");
            }
        };
    }

    pub async fn create_group(&mut self, req: CreateGroupRequest) {
        let api: GroupApi = use_context();
        match api.create_group(req).await {
            Ok(_) => self.group_resource.restart(),
            Err(e) => {
                tracing::error!("failed to create group: {e}");
            }
        }
    }

    pub async fn invite_team_member(
        &mut self,
        group_id: String,
        email: String,
        name: Option<String>,
    ) {
        let api: GroupApi = use_context();
        match api
            .add_team_member(
                group_id,
                TeamMemberRequest {
                    email,
                    name,
                    group: None,
                    role: None,
                },
            )
            .await
        {
            Ok(_) => {
                self.group_resource.restart();
                // self.member_resource.restart()
            }
            Err(e) => {
                tracing::error!("failed to invite team member: {e}");
            }
        };
    }

    pub async fn remove_group(&mut self, group_id: String) {
        let api: GroupApi = use_context();
        match api.remove_group(group_id).await {
            Ok(_) => self.group_resource.restart(),
            Err(e) => {
                tracing::error!("failed to remove group: {e}");
            }
        };
    }

    pub async fn update_group_name(
        &mut self,
        api: &GroupApi,
        group_id: String,
        group_name: String,
    ) {
        match api.update_group_name(group_id, group_name).await {
            Ok(_) => self.group_resource.restart(),
            Err(e) => {
                tracing::error!("failed to update group name: {e}");
            }
        };
    }

    pub async fn open_update_group_name_modal(
        &self,
        lang: Language,
        mut clicked_group_id: Signal<String>,
        mut clicked_group_name: Signal<String>,
    ) {
        let mut popup_service = (self.popup_service)().clone();
        let translates: GroupTranslate = translate(&lang);
        let api: GroupApi = self.group_api;

        let mut group_resource = self.group_resource;

        popup_service
            .open(rsx! {
                UpdateGroupNameModal {
                    lang,
                    onclose: move |_e: MouseEvent| {
                        clicked_group_id.set("".to_string());
                        clicked_group_name.set("".to_string());
                        popup_service.close();
                    },
                    initialize_group_name: clicked_group_name(),
                    update_group_name: move |group_name: String| {
                        async move {
                            match api.update_group_name(clicked_group_id(), group_name).await {
                                Ok(_) => group_resource.restart(),
                                Err(e) => {
                                    tracing::error!("failed to update group name: {e}");
                                }
                            };
                            clicked_group_id.set("".to_string());
                            clicked_group_name.set("".to_string());
                            popup_service.close();
                        }
                    },
                }
            })
            .with_id("update_group")
            .with_title(translates.update_group_name);
    }

    pub async fn open_remove_group_modal(
        &self,
        lang: Language,
        mut clicked_group_id: Signal<String>,
        mut clicked_group_name: Signal<String>,
    ) {
        let mut popup_service = (self.popup_service)().clone();
        let translates: GroupTranslate = translate(&lang);
        let api: GroupApi = self.group_api;

        let mut group_resource = self.group_resource;

        popup_service
            .open(rsx! {
                RemoveGroupModal {
                    lang,
                    onclose: move |_e: MouseEvent| {
                        clicked_group_id.set("".to_string());
                        clicked_group_name.set("".to_string());
                        popup_service.close();
                    },
                    remove_group: move |_e: Event<MouseData>| {
                        async move {
                            match api.remove_group(clicked_group_id()).await {
                                Ok(_) => group_resource.restart(),
                                Err(e) => {
                                    tracing::error!("failed to remove group: {e}");
                                }
                            };
                            clicked_group_id.set("".to_string());
                            clicked_group_name.set("".to_string());
                            popup_service.close();
                        }
                    },
                }
            })
            .with_id("remove_group")
            .with_title(translates.remove_group);
    }

    pub async fn open_create_group_modal(
        &self,
        lang: Language,
        mut clicked_group_id: Signal<String>,
        mut clicked_group_name: Signal<String>,
    ) {
        let mut popup_service = (self.popup_service)().clone();
        let translates: GroupTranslate = translate(&lang);
        let api: GroupApi = self.group_api;

        let mut group_resource = self.group_resource;
        let members = self.get_members();

        popup_service
            .open(rsx! {
                CreateGroupModal {
                    lang,
                    members,
                    oncreate: move |req: CreateGroupRequest| async move {
                        match api.create_group(req).await {
                            Ok(_) => group_resource.restart(),
                            Err(e) => {
                                tracing::error!("failed to create group: {e}");
                            }
                        }
                        popup_service.close();
                    },
                    onclose: move |_e: MouseEvent| {
                        clicked_group_id.set("".to_string());
                        clicked_group_name.set("".to_string());
                        popup_service.close();
                    },
                }
            })
            .with_id("create_group")
            .with_title(translates.create_group);
    }
}
