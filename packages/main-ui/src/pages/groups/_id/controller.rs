use chrono::{Local, TimeZone};
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::prelude::{GroupMemberResponse, TeamMemberRequest};

use crate::{
    models::role_field::RoleField,
    routes::Route,
    service::{group_api::GroupApi, popup_service::PopupService},
};

use super::{
    i18n::GroupDetailTranslate,
    page::{
        AddMemberModal, RemoveGroupModal, RemoveMemberModal, RemoveProjectModal,
        UpdateGroupNameModal,
    },
};

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ProjectType {
    #[default]
    Investigation, //조사
    PublicOpinion, //공론
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum ProjectStatus {
    #[default]
    Ready, //준비
    InProgress, //진행
    Finished,   //마감
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GroupDetail {
    pub group: String,
    pub register_date: String,
    pub group_members: Vec<GroupMemberResponse>,
    pub group_projects: Vec<GroupProject>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GroupProject {
    pub project_type: ProjectType,
    pub project_subject: String,
    pub panels: Vec<String>,
    pub periods: String, //FIXME: fix to start date, end date format
    pub project_status: ProjectStatus,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GroupMember {
    pub member_id: String,
    pub email: String,
    pub profile_image: Option<String>,
    pub profile_name: Option<String>,
    pub group: String,
    pub role: String,
    pub projects: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    pub group: Signal<GroupDetail>,
    pub groups: Signal<Vec<String>>,
    pub roles: Signal<Vec<RoleField>>,
    pub group_resource: Resource<Result<models::prelude::GroupResponse, ServerFnError>>,

    popup_service: Signal<PopupService>,
    group_api: GroupApi,
}

impl Controller {
    pub fn init(
        lang: dioxus_translate::Language,
        popup_service: PopupService,
        group_id: String,
    ) -> Self {
        let translates: GroupDetailTranslate = translate(&lang);
        let api: GroupApi = use_context();
        let group_resource: Resource<Result<models::prelude::GroupResponse, ServerFnError>> =
            use_resource(move || {
                let api = api.clone();
                let group_id = group_id.clone();
                async move { api.get_group(group_id.clone()).await }
            });
        let mut ctrl = Self {
            group: use_signal(|| GroupDetail::default()),
            groups: use_signal(|| vec![]),
            roles: use_signal(|| {
                vec![
                    RoleField {
                        db_name: "super_admin".to_string(),
                        field: translates.manager.to_string(),
                    },
                    RoleField {
                        db_name: "public_admin".to_string(),
                        field: translates.public_opinion_manager.to_string(),
                    },
                    RoleField {
                        db_name: "analyst".to_string(),
                        field: translates.analyst.to_string(),
                    },
                    RoleField {
                        db_name: "mediator".to_string(),
                        field: translates.repeater.to_string(),
                    },
                    RoleField {
                        db_name: "speaker".to_string(),
                        field: translates.lecturer.to_string(),
                    },
                ]
            }),
            group_resource,

            popup_service: use_signal(|| popup_service),
            group_api: api,
        };
        ctrl.groups.set(vec![
            "보이스코리아".to_string(),
            "보이스코리아1".to_string(),
            "보이스코리아2".to_string(),
            "보이스코리아3".to_string(),
        ]);

        let group = if let Some(v) = group_resource.value()() {
            match v {
                Ok(d) => {
                    let seconds = d.created_at / 1000;
                    let nanoseconds = (d.created_at % 1000) * 1_000_000;
                    let datetime = Local.timestamp_opt(seconds, nanoseconds as u32).unwrap();

                    let formatted_date = datetime.format("%Y년 %m월 %d일").to_string();

                    let data = GroupDetail {
                        group: d.name.clone(),
                        register_date: formatted_date.clone(),
                        group_members: d.members,
                        group_projects: vec![],
                    };

                    data
                }
                Err(_) => GroupDetail::default(),
            }
        } else {
            GroupDetail::default()
        };

        ctrl.group.set(group);

        ctrl
    }

    pub fn get_group(&self) -> GroupDetail {
        (self.group)()
    }

    pub fn get_groups(&self) -> Vec<String> {
        (self.groups)()
    }

    pub fn get_roles(&self) -> Vec<RoleField> {
        (self.roles)()
    }

    pub async fn remove_group(&mut self, group_id: String) {
        let api: GroupApi = use_context();
        let _ = api.remove_group(group_id).await;
        self.group_resource.restart();
    }

    pub async fn update_group_name(&mut self, group_id: String, group_name: String) {
        let api: GroupApi = use_context();
        let _ = api.update_group_name(group_id, group_name).await;
        self.group_resource.restart();
    }

    pub async fn update_role(&mut self, _index: usize, _role_name: String) {
        // let api: MemberApi = use_context();
        // let members = self.get_group().group_members;
        // let member = members[index].clone();
        // let _ = api
        //     .update_member(
        //         member.org_member_id,
        //         UpdateMemberRequest {
        //             name: if member.user_name != "" {
        //                 Some(member.user_name.clone())
        //             } else {
        //                 None
        //             },
        //             group: Some(GroupInfo {
        //                 id: member.group_id,
        //                 name: member.group_name,
        //             }),
        //             role: Some(role_name),
        //         },
        //     )
        //     .await;
        self.group_resource.restart();
    }

    pub async fn open_update_group_name_modal(
        &self,
        lang: Language,
        group_id: String,
        initialize_group_name: String,
    ) {
        let mut popup_service = (self.popup_service)().clone();
        let translates: GroupDetailTranslate = translate(&lang);
        let api: GroupApi = self.group_api;

        let mut group_resource = self.group_resource;

        popup_service
            .open(rsx! {
                UpdateGroupNameModal {
                    lang,
                    onclose: move |_e: MouseEvent| {
                        popup_service.close();
                    },
                    initialize_group_name,
                    update_group_name: move |group_name: String| {
                        let group_id = group_id.clone();
                        let group_name = group_name.clone();
                        async move {
                            match api.update_group_name(group_id, group_name).await {
                                Ok(_) => group_resource.restart(),
                                Err(e) => {
                                    tracing::error!("failed to update group name: {e}");
                                }
                            };
                            popup_service.close();
                        }
                    },
                }
            })
            .with_id("update_group_name")
            .with_title(translates.update_group_name);
    }

    pub async fn open_remove_group_modal(&self, lang: Language, group_id: String) {
        let navigator = use_navigator();
        let mut popup_service = (self.popup_service)().clone();
        let translates: GroupDetailTranslate = translate(&lang);
        let api: GroupApi = self.group_api;

        popup_service
            .open(rsx! {
                RemoveGroupModal {
                    lang,
                    remove_group: move |_e: MouseEvent| {
                        let group_id = group_id.clone();
                        async move {
                            match api.remove_group(group_id).await {
                                Ok(_) => {
                                    popup_service.close();
                                    navigator
                                        .push(Route::GroupPage {
                                            lang: lang.clone(),
                                        });
                                }
                                Err(e) => {
                                    tracing::error!("failed to update group name: {e}");
                                }
                            };
                        }
                    },
                    onclose: move |_e: MouseEvent| {
                        popup_service.close();
                    },
                }
            })
            .with_id("remove_group")
            .with_title(translates.remove_group);
    }

    pub async fn open_remove_member_modal(
        &self,
        lang: Language,
        group_id: String,
        member_id: String,
    ) {
        let mut popup_service = (self.popup_service)().clone();
        let translates: GroupDetailTranslate = translate(&lang);
        let api: GroupApi = self.group_api;

        let mut group_resource = self.group_resource;

        popup_service
            .open(rsx! {
                RemoveMemberModal {
                    lang,
                    onremove: move |_e: MouseEvent| {
                        let group_id = group_id.clone();
                        let member_id = member_id.clone();
                        async move {
                            match api.remove_team_member(group_id, member_id).await {
                                Ok(_) => {
                                    group_resource.restart();
                                }
                                Err(e) => {
                                    tracing::error!("failed to remove team member: {e}");
                                }
                            };
                            popup_service.close();
                        }
                    },
                    onclose: move |_e: MouseEvent| {
                        popup_service.close();
                    },
                }
            })
            .with_id("remove_team_member")
            .with_title(translates.remove_team_member);
    }

    pub async fn open_add_member_modal(
        &self,
        lang: Language,
        group_id: String,
        group_name: String,
    ) {
        let mut popup_service = (self.popup_service)().clone();
        let translates: GroupDetailTranslate = translate(&lang);
        let api: GroupApi = self.group_api;

        let mut group_resource = self.group_resource;
        let roles = self.get_roles();

        popup_service
            .open(rsx! {
                AddMemberModal {
                    lang,
                    group_id: group_id.clone(),
                    group_name,
                    onclose: move |_e: MouseEvent| {
                        popup_service.close();
                    },
                    onadd: move |req: TeamMemberRequest| {
                        let group_id = group_id.clone();
                        let req = req.clone();
                        async move {
                            match api.add_team_member(group_id, req).await {
                                Ok(_) => {
                                    group_resource.restart();
                                }
                                Err(e) => {
                                    tracing::error!("failed to add team member: {e}");
                                }
                            };
                            popup_service.close();
                        }
                    },
                    roles,
                }
            })
            .with_id("add_team_member")
            .with_title(translates.add_team_member);
    }

    pub async fn open_remove_project_modal(
        &self,
        lang: Language,
        group_id: String,
        project_id: String,
    ) {
        let mut popup_service = (self.popup_service)().clone();
        let translates: GroupDetailTranslate = translate(&lang);
        let _api: GroupApi = self.group_api;

        let _group_resource = self.group_resource;

        popup_service
            .open(rsx! {
                RemoveProjectModal {
                    lang,
                    onremove: move |_e: MouseEvent| {
                        let group_id = group_id.clone();
                        let project_id = project_id.clone();
                        async move {
                            tracing::debug!("on remove clicked: {} {}", group_id, project_id);
                            popup_service.close();
                        }
                    },
                    onclose: move |_e: MouseEvent| {
                        popup_service.close();
                    },
                }
            })
            .with_id("remove_project")
            .with_title(translates.remove_project);
    }
}
