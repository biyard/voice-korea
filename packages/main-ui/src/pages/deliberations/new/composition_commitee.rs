use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::{deliberation_user::DeliberationUserCreateRequest, OrganizationMemberSummary, Role};

use crate::pages::deliberations::new::{
    components::role_dropdown::RoleDropdown, controller::CurrentStep,
    i18n::CompositionCommitteeTranslate,
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
    let translate: CompositionCommitteeTranslate = translate(&lang);

    let admins = get_role_list(members.clone(), committees.clone(), Role::Admin);
    let deliberation_admins =
        get_role_list(members.clone(), committees.clone(), Role::DeliberationAdmin);
    let analysts = get_role_list(members.clone(), committees.clone(), Role::Analyst);
    let moderators = get_role_list(members.clone(), committees.clone(), Role::Moderator);
    let speakers = get_role_list(members.clone(), committees.clone(), Role::Speaker);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "font-medium text-[16px] text-[#222222] mb-[10px]",
                "{translate.composition_committee_title}"
            }

            div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white px-[40px] py-[24px]",
                div { class: "font-bold text-[#222222] text-lg mb-[3px]", "{translate.division_roles}" }
                div { class: "font-normal text-[#6d6d6d] text-sm mb-[20px]",
                    "{translate.composition_committee_description}"
                }
                // selection box
                div { class: "flex flex-col w-full justify-start items-center mb-[40px]",
                    RoleDropdown {
                        id: "admin_dropdown",
                        label: translate.opinion_designer_label.to_string(),
                        hint: translate.opinion_designer_hint.to_string(),
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
                        label: translate.specific_opinion_designer_label.to_string(),
                        hint: translate.specific_opinion_designer_hint.to_string(),
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
                        label: translate.analyst_label.to_string(),
                        hint: translate.analyst_hint.to_string(),
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
                        label: translate.intermediary_label.to_string(),
                        hint: translate.intermediary_hint.to_string(),
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
                        label: translate.lecturer_label.to_string(),
                        hint: translate.lecturer_hint.to_string(),
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

            div { class: "flex flex-row w-full justify-end items-center font-normal text-[#6d6d6d] text-[14px] mt-[5px]",
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

            div { class: "flex flex-row w-full justify-end items-end mt-[40px] mb-[50px]",
                div {
                    class: "flex flex-row w-[70px] h-[55px] rounded-[4px] justify-center items-center bg-white border border-[#bfc8d9] font-semibold text-[16px] text-[#555462] mr-[20px]",
                    onclick: move |_| {
                        onstep.call(CurrentStep::InputInformation);
                    },
                    "{translate.backward}"
                }
                div {
                    class: "flex flex-row w-[105px] h-[55px] rounded-[4px] justify-center items-center bg-white border border-[#bfc8d9] font-semibold text-[16px] text-[#555462] mr-[20px]",
                    onclick: move |_| {},
                    "{translate.temporary_save}"
                }
                div {
                    class: "cursor-pointer flex flex-row w-[110px] h-[55px] rounded-[4px] justify-center items-center bg-[#2a60d3] font-semibold text-[16px] text-white",
                    onclick: move |_| {
                        onstep.call(CurrentStep::PanelComposition);
                    },
                    "{translate.next}"
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
