#![allow(dead_code, unused)]
use by_macros::DioxusController;
use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::{
    deliberation::DeliberationCreateRequest, deliberation_user::DeliberationUserCreateRequest,
    OrganizationMember, OrganizationMemberQuery, OrganizationMemberSummary, Role,
};

use crate::{
    pages::deliberations::new::{components::role_dropdown::RoleDropdown, controller::CurrentStep},
    service::login_service::LoginService,
};

#[component]
pub fn CompositionCommitee(
    lang: Language,

    roles: Vec<Role>,
    req: DeliberationCreateRequest,

    onprev: EventHandler<(DeliberationCreateRequest, CurrentStep)>,
    onnext: EventHandler<(DeliberationCreateRequest, CurrentStep)>,
) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: CompositionCommitteeTranslate = translate(&lang);

    let members = ctrl.members()?;

    let mut committee_roles: Signal<Vec<Vec<OrganizationMemberSummary>>> = use_signal(|| vec![]);

    use_effect({
        let roles = roles.clone();
        let members = members.clone();

        let committees = req.roles.clone();

        move || {
            for role in roles.clone() {
                let members = get_role_list(members.clone(), committees.clone(), role);

                committee_roles.push(members);
            }

            ctrl.committees.set(committees.clone());
        }
    });

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "font-medium text-base text-text-black mb-10",
                "{tr.composition_committee_title}"
            }

            div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white px-40 py-24",
                div { class: "font-bold text-text-black text-lg mb-3", "{tr.division_roles}" }
                div { class: "font-normal text-text-gray text-sm mb-20",
                    "{tr.composition_committee_description}"
                }
                // selection box
                div { class: "flex flex-col w-full justify-start items-center mb-40",
                    for (i , committee_role) in committee_roles().iter().enumerate() {
                        RoleDropdown {
                            id: format!("{}_dropdown", roles[i].to_string()),
                            label: roles[i].translate(&lang),
                            hint: tr.enter_charge_person,
                            total_committees: ctrl.get_committees(),
                            members: members.clone(),
                            committees: committee_role.clone(),
                            add_committee: {
                                let role = roles[i].clone();
                                let members = members.clone();
                                move |user_id: i64| {
                                    ctrl.add_committee(DeliberationUserCreateRequest {
                                        user_id,
                                        role: role.clone(),
                                    });
                                    let mut list = committee_roles();
                                    if let Some(role_list) = list.get_mut(i) {
                                        let user = members.iter().find(|m| m.user_id == user_id);
                                        if let Some(user) = user {
                                            if !role_list.iter().any(|m| m.user_id == user_id) {
                                                role_list.push(user.clone());
                                            }
                                        }
                                    }
                                    committee_roles.set(list);
                                }
                            },
                            remove_committee: {
                                let role = roles[i].clone();
                                move |user_id: i64| {
                                    ctrl.remove_committee(user_id, role.clone());
                                    let mut list = committee_roles();
                                    if let Some(role_list) = list.get_mut(i) {
                                        role_list.retain(|m| m.user_id != user_id);
                                    }
                                    committee_roles.set(list);
                                }
                            },
                            clear_committee: {
                                let role = roles[i].clone();
                                move |_| {
                                    ctrl.clear_committee(role.clone());
                                    let mut list = committee_roles();
                                    if let Some(role_list) = list.get_mut(i) {
                                        role_list.clear();
                                    }
                                    committee_roles.set(list);
                                }
                            },
                        }
                    }
                }
            }

            div { class: "flex flex-row w-full justify-end items-end mt-40 mb-50",
                div {
                    class: "flex flex-row w-70 h-55 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                    onclick: {
                        let new_req = {
                            let mut r = req.clone();
                            r.roles = ctrl.get_committees();
                            r
                        };
                        move |_| {
                            onprev.call((new_req.clone(), CurrentStep::SettingInfo));
                        }
                    },
                    "{tr.backward}"
                }
                div {
                    class: "flex flex-row w-105 h-55 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                    onclick: move |_| {},
                    "{tr.temporary_save}"
                }
                div {
                    class: "cursor-pointer flex flex-row w-110 h-55 rounded-sm justify-center items-center bg-hover font-semibold text-base text-white",
                    onclick: {
                        let new_req = {
                            let mut r = req.clone();
                            r.roles = ctrl.get_committees();
                            r
                        };
                        move |_| {
                            onnext.call((new_req.clone(), CurrentStep::CompositionPanel));
                        }
                    },
                    "{tr.next}"
                }
            }
        }
    }
}

pub fn get_role_list(
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

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,

    pub members: Resource<Vec<OrganizationMemberSummary>>,
    pub committees: Signal<Vec<DeliberationUserCreateRequest>>,
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

        let ctrl = Self {
            lang,
            members,
            committees: use_signal(move || vec![]),
        };

        Ok(ctrl)
    }

    pub fn set_committee(&mut self, committee: Vec<DeliberationUserCreateRequest>) {
        self.committees.set(committee);
    }

    pub fn get_committees(&self) -> Vec<DeliberationUserCreateRequest> {
        (self.committees)()
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
}

translate! {
    CompositionCommitteeTranslate;

    enter_charge_person: {
        ko: "담당자명 입력",
        en: "Enter the person in charge"
    }
    composition_committee_title: {
        ko: "공론 위원회 구성",
        en: "Composition of a public opinion committee"
    }
    composition_committee_description: {
        ko: "공론위원회는 다양한 의견을 수렴하고 합의된 결정을 도출하는 역할을 합니다. 각 역할의 담당자를 설정해주세요.",
        en: "The Public Opinion Committee's role is to collect diverse opinions and arrive at a consensus decision. Please set a person in charge of each role."
    }
    opinion_designer_label: {
        ko: "공론 설계자",
        en: "Public Opinion Designer"
    }
    opinion_designer_hint: {
        ko: "공론 설계자 선택",
        en: "Select Public Opinion Designer"
    }
    specific_opinion_designer_label: {
        ko: "특정 공론 설계자",
        en: "Specific Public Opinion Designer"
    }
    specific_opinion_designer_hint: {
        ko: "특정 공론 설계자 선택",
        en: "Select Specific Public Opinion Designer"
    }
    analyst_label: {
        ko: "분석가",
        en: "Analyst"
    }
    analyst_hint: {
        ko: "분석가 선택",
        en: "Select Analyst"
    }
    intermediary_label: {
        ko: "중개자",
        en: "Intermediary"
    }
    intermediary_hint: {
        ko: "중개자 선택",
        en: "Select Intermediary"
    }
    lecturer_label: {
        ko: "강연자",
        en: "Lecturer"
    }
    lecturer_hint: {
        ko: "강연자 선택",
        en: "Select Lecturer"
    }
    division_roles: {
        ko: "역할 분담",
        en: "Division of Roles"
    }
    backward: {
        ko: "뒤로",
        en: "Backward"
    }
    temporary_save: {
        ko: "임시저장",
        en: "Temporary Save"
    }
    next: {
        ko: "다음으로",
        en: "Next"
    }
}
