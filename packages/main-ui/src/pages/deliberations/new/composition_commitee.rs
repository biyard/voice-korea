use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::{deliberation_user::DeliberationUserCreateRequest, OrganizationMemberSummary, Role};

use crate::pages::deliberations::new::{
    components::role_dropdown::RoleDropdown, controller::CurrentStep,
};

#[component]
pub fn CompositionCommitee(
    lang: Language,
    members: Vec<OrganizationMemberSummary>,
    committees: Vec<DeliberationUserCreateRequest>,

    add_committee: EventHandler<DeliberationUserCreateRequest>,
    remove_committee: EventHandler<(i64, Role)>,
    clear_committee: EventHandler<Role>,

    onstep: EventHandler<CurrentStep>,
) -> Element {
    let tr: CompositionCommitteeTranslate = translate(&lang);

    let admins = get_role_list(members.clone(), committees.clone(), Role::Admin);
    let deliberation_admins =
        get_role_list(members.clone(), committees.clone(), Role::DeliberationAdmin);
    let analysts = get_role_list(members.clone(), committees.clone(), Role::Analyst);
    let moderators = get_role_list(members.clone(), committees.clone(), Role::Moderator);
    let speakers = get_role_list(members.clone(), committees.clone(), Role::Speaker);

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
                    RoleDropdown {
                        id: "admin_dropdown",
                        label: tr.opinion_designer_label.to_string(),
                        hint: tr.opinion_designer_hint.to_string(),
                        total_committees: committees.clone(),
                        members: members.clone(),
                        committees: admins.clone(),
                        add_committee: move |user_id: i64| {
                            add_committee
                                .call(DeliberationUserCreateRequest {
                                    user_id,
                                    role: Role::Admin,
                                });
                        },
                        remove_committee: move |user_id: i64| {
                            remove_committee.call((user_id, Role::Admin));
                        },
                        clear_committee: move |_| {
                            clear_committee.call(Role::Admin);
                        },
                    }
                    RoleDropdown {
                        id: "deliberation_admin_dropdown",
                        label: tr.specific_opinion_designer_label.to_string(),
                        hint: tr.specific_opinion_designer_hint.to_string(),
                        total_committees: committees.clone(),
                        members: members.clone(),
                        committees: deliberation_admins.clone(),
                        add_committee: move |user_id: i64| {
                            add_committee
                                .call(DeliberationUserCreateRequest {
                                    user_id,
                                    role: Role::DeliberationAdmin,
                                });
                        },
                        remove_committee: move |user_id: i64| {
                            remove_committee.call((user_id, Role::DeliberationAdmin));
                        },
                        clear_committee: move |_| {
                            clear_committee.call(Role::DeliberationAdmin);
                        },
                    }
                    RoleDropdown {
                        id: "analyst_admin_dropdown",
                        label: tr.analyst_label.to_string(),
                        hint: tr.analyst_hint.to_string(),
                        total_committees: committees.clone(),
                        members: members.clone(),
                        committees: analysts.clone(),
                        add_committee: move |user_id: i64| {
                            add_committee
                                .call(DeliberationUserCreateRequest {
                                    user_id,
                                    role: Role::Analyst,
                                });
                        },
                        remove_committee: move |user_id: i64| {
                            remove_committee.call((user_id, Role::Analyst));
                        },
                        clear_committee: move |_| {
                            clear_committee.call(Role::Analyst);
                        },
                    }
                    RoleDropdown {
                        id: "moderator_admin_dropdown",
                        label: tr.intermediary_label.to_string(),
                        hint: tr.intermediary_hint.to_string(),
                        total_committees: committees.clone(),
                        members: members.clone(),
                        committees: moderators.clone(),
                        add_committee: move |user_id: i64| {
                            add_committee
                                .call(DeliberationUserCreateRequest {
                                    user_id,
                                    role: Role::Moderator,
                                });
                        },
                        remove_committee: move |user_id: i64| {
                            remove_committee.call((user_id, Role::Moderator));
                        },
                        clear_committee: move |_| {
                            clear_committee.call(Role::Moderator);
                        },
                    }
                    RoleDropdown {
                        id: "speaker_admin_dropdown",
                        label: tr.lecturer_label.to_string(),
                        hint: tr.lecturer_hint.to_string(),
                        total_committees: committees.clone(),
                        members: members.clone(),
                        committees: speakers.clone(),
                        add_committee: move |user_id: i64| {
                            add_committee
                                .call(DeliberationUserCreateRequest {
                                    user_id,
                                    role: Role::Speaker,
                                });
                        },
                        remove_committee: move |user_id: i64| {
                            remove_committee.call((user_id, Role::Speaker));
                        },
                        clear_committee: move |_| {
                            clear_committee.call(Role::Speaker);
                        },
                    }
                }
            }

            div { class: "flex flex-row w-full justify-end items-center font-normal text-text-gray text-sm mt-5",
                {
                    format!(
                        "총 {}명 / 공론 설계자 {}명, 특정 공론 설계자 {}명, 분석가 {}명, 중개자 {}명, 강연자 {}명",
                        admins.len() + deliberation_admins.len() + analysts.len() + moderators.len()
                            + speakers.len(),
                        admins.len(),
                        deliberation_admins.len(),
                        analysts.len(),
                        moderators.len(),
                        speakers.len(),
                    )
                }
            }

            div { class: "flex flex-row w-full justify-end items-end mt-40 mb-50",
                div {
                    class: "flex flex-row w-70 h-55 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                    onclick: move |_| {
                        onstep.call(CurrentStep::SettingInfo);
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
                    onclick: move |_| {
                        onstep.call(CurrentStep::CompositionPanel);
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

translate! {
    CompositionCommitteeTranslate;

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
