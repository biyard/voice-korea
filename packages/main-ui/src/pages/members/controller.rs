use std::str::FromStr;

use by_macros::DioxusController;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::{
    OrganizationMember, OrganizationMemberQuery, OrganizationMemberSummary, QueryResponse, Role,
};

use crate::{
    models::role_field::RoleField,
    pages::members::components::add_member_modal::AddMemberModal,
    service::{login_service::LoginService, popup_service::PopupService},
};

use super::{
    components::add_member_modal::InviteMemberRequest, i18n::MemberTranslate,
    page::RemoveMemberModal,
};

// #[derive(Debug, Clone, PartialEq, Default)]
// pub struct MemberSummary {
//     pub role_counts: Vec<u64>, // [전체 팀원, 관리자 수, 공론 관리자 수, 분석가 수, 중개자 수, 강연자 수],
//     pub members: Vec<Member>,
// }
// #[derive(Debug, Clone, PartialEq, Default)]
// pub struct Member {
//     pub member_id: String,
//     pub profile: Option<String>,
//     pub profile_name: Option<String>,
//     pub email: String,
//     pub group: String,
//     pub role: String,
//     pub projects: Vec<String>, //유저가 속해있는 프로젝트
// }

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    pub members: Resource<QueryResponse<OrganizationMemberSummary>>,
    pub roles: Signal<Vec<RoleField>>,
    pub popup_service: PopupService,
    pub user_service: LoginService,
    pub page: Signal<usize>,
    pub selected_member: Signal<Option<i64>>,
    pub lang: Language,
}

impl Controller {
    pub fn new(lang: dioxus_translate::Language) -> std::result::Result<Self, RenderError> {
        let translates: MemberTranslate = translate(&lang);
        let page = use_signal(|| 1);
        let user_service: LoginService = use_context();

        let members = use_server_future(move || {
            let page = page();
            let org_id = user_service.org_id();
            let size = 10;
            async move {
                let endpoint = crate::config::get().api_url;
                let res = OrganizationMember::get_client(endpoint)
                    .query(org_id, OrganizationMemberQuery::new(size).with_page(page))
                    .await;

                res.unwrap_or_default()
            }
        })?;

        let ctrl = Self {
            lang,
            members,
            page,
            user_service,
            selected_member: use_signal(|| None),
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
            popup_service: use_context(),
        };

        Ok(ctrl)
    }

    pub async fn invite_member(
        &mut self,
        InviteMemberRequest {
            email, name, role, ..
        }: InviteMemberRequest,
    ) {
        if let Err(e) = OrganizationMember::get_client(crate::config::get().api_url)
            .create(self.user_service.org_id(), name, role, None, email)
            .await
        {
            btracing::error!("{}", e.translate(&self.lang));
        };

        self.members.restart();
    }

    pub async fn update_group(&mut self, _id: i64, _group_index: usize) {
        // TODO: implement update group
        btracing::error!("update_group not implemented");
    }

    pub async fn update_role(&mut self, id: i64, role_name: String) {
        let role = Role::from_str(&role_name).unwrap_or_default();
        let endpoint = crate::config::get().api_url;
        OrganizationMember::get_client(endpoint)
            .update_role(self.user_service.org_id(), id, Some(role))
            .await
            .unwrap_or_default();

        self.members.restart();
    }

    pub async fn remove_member(&mut self, member_id: i64) {
        let endpoint = crate::config::get().api_url;
        if let Err(e) = OrganizationMember::get_client(endpoint)
            .delete(self.user_service.org_id(), member_id)
            .await
        {
            btracing::error!("{}", e.translate(&self.lang));
            return;
        }

        self.members.restart();
    }

    pub async fn open_remove_member_modal(&mut self, lang: Language, member_id: i64) {
        let translates: MemberTranslate = translate(&lang);
        let mut ctrl = *self;

        self.popup_service
            .open(rsx! {
                RemoveMemberModal {
                    lang,
                    remove_member: move |_e: MouseEvent| async move {
                        ctrl.remove_member(member_id).await;
                        ctrl.popup_service.close();
                    },
                    onclose: move |_e: MouseEvent| {
                        ctrl.selected_member.set(None);
                        ctrl.popup_service.close();
                    },
                }
            })
            .with_id("remove_team_member")
            .with_title(translates.remove_team_member);
    }

    pub async fn open_add_member_modal(&mut self, lang: Language) {
        let translates: MemberTranslate = translate(&lang);
        let mut ctrl = *self;

        self.popup_service
            .open(rsx! {
                AddMemberModal {
                    lang,
                    onsubmit: move |req: InviteMemberRequest| async move {
                        ctrl.invite_member(req).await;
                        ctrl.popup_service.close();
                    },
                    onclose: move |_e: MouseEvent| {
                        ctrl.popup_service.close();
                    },
                }
            })
            .with_id("add_team_member")
            .with_title(translates.add_team_member);
    }
}
