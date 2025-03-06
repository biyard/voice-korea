use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::{
    prelude::{Group, GroupResponse, MemberSummary, UpdateMemberRequest},
    QueryResponse,
};

use crate::{models::role_field::RoleField, service::popup_service::PopupService};

use super::i18n::MemberDetailTranslate;

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
pub struct MemberDetail {
    pub email: String,
    pub profile_image: Option<String>,
    pub profile_name: Option<String>,
    pub group: Group,
    pub role: String,
    pub register_date: String,
    pub project_history: Vec<ProjectHistory>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ProjectHistory {
    pub history_id: String,
    pub project_type: ProjectType,
    pub project_subject: String,
    pub role: String,
    pub panel: Vec<String>,
    pub period: String, //FIXME: fix start date, end date form
    pub project_status: ProjectStatus,
}

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    pub member: Signal<MemberDetail>,
    pub groups: Signal<Vec<GroupResponse>>,
    pub roles: Signal<Vec<RoleField>>,
    pub member_resource: Resource<MemberSummary>,

    pub popup_service: Signal<PopupService>,
    pub group_resource: Resource<QueryResponse<GroupResponse>>,
}

impl Controller {
    pub fn init(
        lang: dioxus_translate::Language,
        popup_service: PopupService,
        member_id: String,
    ) -> Self {
        let translates: MemberDetailTranslate = translate(&lang);
        let member_resource: Resource<MemberSummary> = use_resource(move || {
            let _member_id = member_id.clone();
            async move { MemberSummary::default() }
        });

        let group_resource: Resource<QueryResponse<GroupResponse>> =
            use_resource(move || async move { Default::default() });

        let ctrl = Self {
            member: use_signal(|| MemberDetail::default()),
            groups: use_signal(|| vec![]),
            roles: use_signal(|| {
                vec![
                    RoleField {
                        db_name: "super_admin".to_string(),
                        field: translates.manager.to_string(),
                    },
                    RoleField {
                        db_name: "public_admin".to_string(),
                        field: translates.opinion_manager.to_string(),
                    },
                    RoleField {
                        db_name: "analyst".to_string(),
                        field: translates.analyst.to_string(),
                    },
                    RoleField {
                        db_name: "mediator".to_string(),
                        field: translates.mediator.to_string(),
                    },
                    RoleField {
                        db_name: "speaker".to_string(),
                        field: translates.speaker.to_string(),
                    },
                ]
            }),
            group_resource,
            member_resource,
            popup_service: use_signal(|| popup_service),
        };

        // let member = if let Some(v) = member_resource.value()() {
        //     match v {
        //         Ok(d) => {
        //             let seconds = d.member.created_at / 1000;
        //             let nanoseconds = (d.member.created_at % 1000) * 1_000_000;
        //             let datetime = Local.timestamp_opt(seconds, nanoseconds as u32).unwrap();

        //             let formatted_date = datetime.format("%Y년 %m월 %d일").to_string();

        //             let data = MemberDetail {
        //                 email: d.email.clone(),
        //                 profile_image: None,
        //                 profile_name: Some(d.member.name),
        //                 //FIXME: fix to group
        //                 group: if d.groups.len() == 0 {
        //                     Group::default()
        //                 } else {
        //                     d.groups[0].clone()
        //                 },
        //                 role: if d.member.role.is_none() {
        //                     "".to_string()
        //                 } else {
        //                     d.member.role.clone().unwrap().to_string()
        //                 },
        //                 register_date: formatted_date,
        //                 project_history: vec![],
        //             };

        //             tracing::debug!("member data: {:?}", data);
        //             data
        //         }
        //         Err(_) => MemberDetail::default(),
        //     }
        // } else {
        //     MemberDetail::default()
        // };

        // let groups = if let Some(v) = group_resource.value()() {
        //     match v {
        //         Ok(v) => v.items,
        //         Err(e) => {
        //             tracing::error!("list groups failed: {:?}", e);
        //             vec![]
        //         }
        //     }
        // } else {
        //     vec![]
        // };

        // ctrl.groups.set(groups);
        // ctrl.member.set(member);

        ctrl
    }

    pub fn get_member(&self) -> MemberDetail {
        (self.member)()
    }

    pub fn get_groups(&self) -> Vec<GroupResponse> {
        (self.groups)()
    }

    pub fn get_roles(&self) -> Vec<RoleField> {
        (self.roles)()
    }

    pub async fn update_member(&mut self, _member_id: String, _req: UpdateMemberRequest) {
        // let api: MemberApi = use_context();
        // let _ = match api.update_member(member_id, req).await {
        //     Ok(_) => self.member_resource.restart(),
        //     Err(v) => {
        //         tracing::error!("update failed: {v}");
        //     }
        // };
    }

    pub async fn remove_member(&mut self, _user_id: String) {
        // let api: MemberApi = use_context();
        // let _ = match api.remove_member(user_id).await {
        //     Ok(_) => self.member_resource.restart(),
        //     Err(v) => {
        //         tracing::error!("remove failed: {v}");
        //     }
        // };
    }

    pub async fn open_remove_project_modal(
        &self,
        _lang: Language,
        _member_id: String,
        _history_id: String,
    ) {
        // let mut popup_service = (self.popup_service)().clone();
        // let translates: MemberDetailTranslate = translate(&lang);
        // let _api: MemberApi = self.member_api;

        // let _member_resource = self.member_resource;

        // popup_service
        //     .open(rsx! {
        //         RemoveProjectModal {
        //             lang,
        //             remove_project: move |_e: MouseEvent| {
        //                 let member_id = member_id.clone();
        //                 let history_id = history_id.clone();
        //                 async move {
        //                     tracing::debug!("remove project clicked id: {} {}", member_id, history_id);
        //                 }
        //             },
        //             onclose: move |_e: MouseEvent| {
        //                 popup_service.close();
        //             },
        //         }
        //     })
        //     .with_id("remove_project_title")
        //     .with_title(translates.remove_project_title);
    }

    pub async fn open_remove_member_modal(&self, _lang: Language, _member_id: String) {
        // let mut popup_service = (self.popup_service)().clone();
        // let translates: MemberDetailTranslate = translate(&lang);
        // let api: MemberApi = self.member_api;

        // let mut member_resource = self.member_resource;

        // popup_service
        //     .open(rsx! {
        //         RemoveMemberModal {
        //             lang,
        //             remove_member: move |_e: MouseEvent| {
        //                 let member_id = member_id.clone();
        //                 async move {
        //                     let _ = api.remove_member(member_id.clone()).await;
        //                     member_resource.restart();
        //                     popup_service.close();
        //                 }
        //             },
        //             onclose: move |_e: MouseEvent| {
        //                 popup_service.close();
        //             },
        //         }
        //     })
        //     .with_id("remove_team_member_title")
        //     .with_title(translates.remove_team_member_title);
    }
}
