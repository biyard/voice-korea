use bdk::prelude::*;
use models::{
    deliberation_user::DeliberationUserCreateRequest, OrganizationMember, OrganizationMemberQuery,
    OrganizationMemberSummary, Role,
};

use super::*;
use crate::service::login_service::LoginService;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,
    #[allow(dead_code)]
    pub parent_ctrl: ParentController,
    pub roles: Signal<Vec<Role>>,

    pub members: Resource<Vec<OrganizationMemberSummary>>,
    pub committees: Signal<Vec<DeliberationUserCreateRequest>>,

    pub committee_roles: Signal<Vec<Vec<OrganizationMemberSummary>>>,
    pub nav: Navigator,
}

impl Controller {
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let user: LoginService = use_context();

        let members = use_server_future(move || {
            let page = 1;
            let size = 20;
            async move {
                let org_id = user.get_selected_org();
                if org_id.is_none() {
                    tracing::error!("Organization ID is missing");
                    return vec![];
                }
                let endpoint = crate::config::get().api_url;
                let res = OrganizationMember::get_client(endpoint)
                    .query(
                        org_id.unwrap().id,
                        OrganizationMemberQuery::new(size).with_page(page),
                    )
                    .await;

                res.unwrap_or_default().items
            }
        })?;

        let mut ctrl = Self {
            lang,
            members,
            parent_ctrl: use_context(),
            nav: use_navigator(),

            committees: use_signal(|| vec![]),
            committee_roles: use_signal(|| vec![]),
            roles: use_signal(|| {
                vec![
                    Role::Admin,
                    Role::DeliberationAdmin,
                    Role::Analyst,
                    Role::Moderator,
                    Role::Speaker,
                ]
            }),
        };

        use_effect({
            let req = ctrl.parent_ctrl.deliberation_requests();
            let roles = ctrl.roles();
            let members = members().unwrap_or_default();

            let committees = req.roles.clone();

            move || {
                ctrl.committees.set(committees.clone());

                for role in roles.clone() {
                    let members = ctrl.get_role_list(members.clone(), committees.clone(), role);

                    ctrl.committee_roles.push(members);
                }
            }
        });

        Ok(ctrl)
    }

    pub fn back(&self) {
        self.nav.go_back();
    }

    pub fn next(&self) {
        self.nav
            .push(crate::routes::Route::CompositionPanel { lang: self.lang });
    }

    pub fn save_deliberation(&mut self) {
        let mut parent_ctrl = self.parent_ctrl;
        let roles = self.committees().iter().map(|v| v.clone()).collect();
        parent_ctrl.save_committees(roles);
    }

    pub fn add_committee(&mut self, committee: DeliberationUserCreateRequest) {
        self.committees.push(committee);
    }

    pub fn remove_committee(&mut self, user_id: i64, role: Role) {
        self.committees
            .retain(|committee| !(committee.user_id == user_id && committee.role == role));
    }

    pub fn clear_committee(&mut self, role: Role) {
        self.committees
            .retain(|committee| !(committee.role == role));
    }

    pub fn add_committee_roles(&mut self, index: usize, user_id: i64) {
        let mut list = self.committee_roles();
        let members = self.members().unwrap_or_default();

        if let Some(role_list) = list.get_mut(index) {
            let user = members.iter().find(|m| m.user_id == user_id);
            if let Some(user) = user {
                if !role_list.iter().any(|m| m.user_id == user_id) {
                    role_list.push(user.clone());
                }
            }
        }
        self.committee_roles.set(list);
    }

    pub fn remove_committee_roles(&mut self, index: usize, user_id: i64) {
        let mut list = self.committee_roles();
        if let Some(role_list) = list.get_mut(index) {
            role_list.retain(|m| m.user_id != user_id);
        }
        self.committee_roles.set(list);
    }

    pub fn clear_committee_roles(&mut self, index: usize) {
        let mut list = self.committee_roles();
        if let Some(role_list) = list.get_mut(index) {
            role_list.clear();
        }
        self.committee_roles.set(list);
    }

    pub fn get_role_list(
        &mut self,
        members: Vec<OrganizationMemberSummary>,
        committees: Vec<DeliberationUserCreateRequest>,
        role: Role,
    ) -> Vec<OrganizationMemberSummary> {
        let user_ids: Vec<i64> = committees
            .iter()
            .filter(|committee| committee.role == role)
            .map(|committee| committee.user_id)
            .collect();

        let members = members
            .into_iter()
            .filter(|member| user_ids.contains(&member.user_id))
            .collect();

        members
    }
}
