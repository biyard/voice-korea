use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::{
    deliberation_user::DeliberationUserCreateRequest, step::StepCreateRequest,
    OrganizationMemberSummary, PanelV2Summary, Role,
};

use crate::{
    components::icons::ArrowRight,
    pages::deliberations::new::{
        components::preview_component::PreviewComponent,
        i18n::{
            CompositionCommitteeSummaryTranslate, CompositionOpinionSummaryTranslate,
            CompositionPanelSummaryTranslate, InputOpinionSummaryTranslate, PreviewTranslate,
        },
    },
    utils::time::convert_timestamp_to_date,
};

use super::controller::{CurrentStep, DeliberationInformation};

#[component]
pub fn Preview(
    lang: Language,
    sequences: Vec<StepCreateRequest>,
    informations: DeliberationInformation,
    committees: Vec<DeliberationUserCreateRequest>,
    members: Vec<OrganizationMemberSummary>,
    selected_panels: Vec<PanelV2Summary>,

    onstep: EventHandler<CurrentStep>,
    onsend: EventHandler<MouseEvent>,
) -> Element {
    let translate: PreviewTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-[40px]",
            CompositionOpinionSummary { lang, sequences, onstep }
            InputOpinionSummary { lang, informations, onstep }
            CompositionCommitteeSummary {
                lang,
                committees,
                members,
                onstep,
            }
            CompositionPanelSummary { lang, selected_panels, onstep }

            div { class: "flex flex-row w-full justify-end items-end mb-[50px]",
                div {
                    class: "flex flex-row w-[70px] h-[55px] rounded-[4px] justify-center items-center bg-white border border-[#bfc8d9] font-semibold text-[16px] text-[#555462] mr-[20px]",
                    onclick: move |_| {
                        onstep.call(CurrentStep::DiscussionSetting);
                    },
                    "{translate.backward}"
                }
                div {
                    class: "cursor-pointer flex flex-row w-[130px] h-[55px] rounded-[4px] justify-center items-center bg-[#2a60d3] font-semibold text-[16px] text-white",
                    onclick: {
                        move |e: Event<MouseData>| {
                            onsend.call(e);
                        }
                    },
                    "{translate.start_public_opinion}"
                }
            }
        }
    }
}

// #[component]
// pub fn SendAlertModal(
//     onclose: EventHandler<MouseEvent>,
//     onclick: EventHandler<MouseEvent>,
//     lang: Language,
// ) -> Element {
//     let translate: SendAlertTranslate = translate(&lang);
//     rsx! {
//         div { class: "flex flex-col w-full justify-center items-center",
//             div { class: "font-normal text-[#222222] text-[14px] mb-[20px]",
//                 "{translate.send_alert_description}"
//             }
//             Message { width: "100", height: "100" }
//             div { class: "flex flex-row w-full justify-center items-center font-normal text-[#6d6d6d] text-[14px] mt-[10px] mb-[20px]",
//                 "총 50명 선택 / 패널 4개 선택"
//             }
//             div { class: "flex flex-row w-full justify-center items-center gap-[20px]",
//                 div {
//                     class: "flex flex-row w-[75px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-[4px] font-semibold text-[16px] text-white",
//                     onclick,
//                     "{translate.send}"
//                 }
//                 button {
//                     class: "flex flex-row w-[60px] h-[40px] justify-center items-center bg-white font-semibold text-[#222222] text-[16px]",
//                     onclick: move |e: MouseEvent| {
//                         onclose.call(e);
//                     },
//                     "{translate.cancel}"
//                 }
//             }
//         }
//     }
// }

#[component]
pub fn CompositionPanelSummary(
    lang: Language,
    selected_panels: Vec<PanelV2Summary>,
    onstep: EventHandler<CurrentStep>,
) -> Element {
    let translate: CompositionPanelSummaryTranslate = translate(&lang);

    rsx! {
        PreviewComponent {
            lang,
            label: translate.participant_panel_composition,
            onstep: move |step: CurrentStep| {
                onstep.call(step);
            },
            step: CurrentStep::PanelComposition,
            title: translate.full_panel_settings,
            div { class: "flex flex-col w-full justify-start items-start gap-[40px]",
                div { class: "flex flex-row w-full min-h-[55px] justify-start items-center",
                    div { class: "flex flex-row w-[230px] h-full justify-start items-center",
                        div { class: "font-medium text-[#222222] text-[15px]",
                            "{translate.select_panel}"
                        }
                    }
                    div { class: "flex flex-row w-full h-full justify-start items-center p-[15px]",
                        div { class: "flex flex-wrap flex-1 gap-[30px]",
                            for panel in selected_panels.clone() {
                                div { class: "flex flex-row w-fit justify-start items-center gap-[5px]",
                                    SummaryLabel { label: panel.name.clone() }
                                    div { class: "font-medium text-[15px] text-black",
                                        "{panel.user_count} {translate.unit}"
                                    }
                                }
                            }
                        }
                    }
                }

                div { class: "flex flex-col w-full justify-start items-start gap-[20px]",
                    div { class: "font-bold text-[18px] text-[#222222]",
                        "{translate.setting_properties_for_each_panel}"
                    }

                    div { class: "flex flex-col w-full justify-start items-start gap-[20px]",
                        for panel in selected_panels.clone() {
                            div { class: "flex flex-row w-full min-h-[55px] justify-start items-center",
                                div { class: "flex flex-row w-[230px] h-full justify-start items-center",
                                    div { class: "font-medium text-[15px] text-black",
                                        "{panel.name}"
                                    }
                                }

                                div { class: "flex flex-wrap flex-1 gap-[5px]",
                                    for attribute in panel.attributes.clone() {
                                        match attribute {
                                            models::response::Attribute::Age(age) => {
                                                rsx! {
                                                    SummaryLabel { label: age.translate(&lang) }
                                                }
                                            }
                                            models::response::Attribute::Gender(gender) => {
                                                rsx! {
                                                    SummaryLabel { label: gender.translate(&lang) }
                                                }
                                            }
                                            models::response::Attribute::Region(region) => {
                                                rsx! {
                                                    SummaryLabel { label: region.translate(&lang) }
                                                }
                                            }
                                            models::response::Attribute::Salary(salary) => {
                                                rsx! {
                                                    SummaryLabel { label: salary.translate(&lang) }
                                                }
                                            }
                                            models::response::Attribute::None => {
                                                rsx! {}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn CompositionCommitteeSummary(
    lang: Language,
    committees: Vec<DeliberationUserCreateRequest>,
    members: Vec<OrganizationMemberSummary>,
    onstep: EventHandler<CurrentStep>,
) -> Element {
    let translate: CompositionCommitteeSummaryTranslate = translate(&lang);

    let admin = get_role_list(committees.clone(), members.clone(), Role::Admin);
    let deliberation_admin =
        get_role_list(committees.clone(), members.clone(), Role::DeliberationAdmin);
    let analyst = get_role_list(committees.clone(), members.clone(), Role::Analyst);
    let moderator = get_role_list(committees.clone(), members.clone(), Role::Moderator);
    let speaker = get_role_list(committees.clone(), members.clone(), Role::Speaker);

    rsx! {
        PreviewComponent {
            lang,
            label: translate.composition_public_opinion_committee,
            onstep: move |step: CurrentStep| {
                onstep.call(step);
            },
            step: CurrentStep::CommitteeComposition,
            title: translate.division_of_roles,
            div { class: "flex flex-col w-full justify-start items-start gap-[10px]",
                CommitteeSummary { label: translate.admin, members: admin }
                CommitteeSummary {
                    label: translate.deliberation_admin,
                    members: deliberation_admin,
                }
                CommitteeSummary { label: translate.analyst, members: analyst }
                CommitteeSummary { label: translate.moderator, members: moderator }
                CommitteeSummary { label: translate.speaker, members: speaker }
            }
        }
    }
}

#[component]
pub fn InputOpinionSummary(
    lang: Language,
    informations: DeliberationInformation,
    onstep: EventHandler<CurrentStep>,
) -> Element {
    let translate: InputOpinionSummaryTranslate = translate(&lang);

    rsx! {
        PreviewComponent {
            lang,
            label: translate.input_necessary_information,
            onstep: move |step: CurrentStep| {
                onstep.call(step);
            },
            step: CurrentStep::InputInformation,
            title: translate.introduction,
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "flex flex-row px-[15px] py-[8px] justify-start items-start font-medium text-[18px] text-[#222222]",
                    "{informations.title.clone().unwrap_or_default()}"
                }
                div { class: "flex flex-row w-full h-[1px] bg-[#EBEFF5] mb-[10px]" }
                div { class: "flex flex-col w-full gap-[40px]",
                    div { class: "flex flex-row px-[15px] py-[8px] justify-start items-start font-medium text-[15px] text-[#222222]",
                        "{informations.description.clone().unwrap_or_default()}"
                    }

                    div { class: "flex flex-col w-full justify-start items-start gap-[10px]",
                        div { class: "font-bold text-[18px] text-[#222222]",
                            "{translate.upload_document}"
                        }
                        div { class: "flex flex-wrap flex-1 gap-[5px] p-[15px]",
                            for document in informations.clone().documents {
                                SummaryLabel { label: document.title.clone() }
                            }
                        }
                    }

                    div { class: "flex flex-col w-full justify-start items-start gap-[10px]",
                        div { class: "font-bold text-[18px] text-[#222222]",
                            "{translate.upload_survey_project}"
                        }
                        div { class: "flex flex-wrap flex-1 gap-[5px] p-[15px]",
                            for project in informations.clone().projects {
                                SummaryLabel { label: project.name.clone() }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn CompositionOpinionSummary(
    lang: Language,
    sequences: Vec<StepCreateRequest>,
    onstep: EventHandler<CurrentStep>,
) -> Element {
    let translate: CompositionOpinionSummaryTranslate = translate(&lang);
    rsx! {
        PreviewComponent {
            lang,
            label: translate.public_opinion_composition_and_period,
            onstep: move |step: CurrentStep| {
                onstep.call(step);
            },
            step: CurrentStep::PublicOpinionComposition,
            title: translate.public_opinion_composition_and_period,
            div { class: "flex flex-wrap w-full justify-start items-center gap-[10px]",
                for (i , sequence) in sequences.iter().enumerate() {
                    CompositionOpinionSummaryCard {
                        title: sequence.name.clone(),
                        date: "{convert_timestamp_to_date(sequence.started_at)} ~ {convert_timestamp_to_date(sequence.ended_at)}",
                    }

                    if i != sequences.len() - 1 {
                        ArrowRight { width: "12", height: "12" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn SummaryLabel(label: String) -> Element {
    rsx! {
        div { class: "flex flex-row h-[25px] justify-center items-center px-[8px] py-[3px] bg-[#35343f] rounded-[4px] font-semibold text-[14px] text-white",
            {label}
        }
    }
}

#[component]
pub fn CompositionOpinionSummaryCard(title: String, date: String) -> Element {
    rsx! {
        div { class: "flex flex-col w-[200px] justify-center items-center bg-white border border-[#bfc8d9] px-[15px] py-[10px]",
            div { class: "font-medium text-[#222222] text-[15px] mb-[5px]", {title} }
            div { class: "font-normal text-[#6d6d6d] text-[14px]", {date} }
        }
    }
}

#[component]
pub fn CommitteeSummary(label: String, members: Vec<OrganizationMemberSummary>) -> Element {
    rsx! {
        div { class: "flex flex-row w-full min-h-[55px] justify-start items-center",
            div { class: "flex flex-row w-[230px] h-full justify-start items-center",
                div { class: "font-medium text-[#222222] text-[15px]", "{label}" }
            }

            div { class: "flex flex-row w-full h-full justify-start items-center p-[15px]",
                div { class: "flex flex-wrap flex-1 gap-[5px]",
                    for member in members {
                        SummaryLabel { label: member.name.clone() }
                    }
                }
            }
        }
    }
}

pub fn get_role_list(
    committees: Vec<DeliberationUserCreateRequest>,
    members: Vec<OrganizationMemberSummary>,
    role: Role,
) -> Vec<OrganizationMemberSummary> {
    let user_ids: Vec<i64> = committees
        .iter()
        .filter(|committee| committee.role == role)
        .map(|committee| committee.user_id)
        .collect();

    let users: Vec<OrganizationMemberSummary> = members
        .into_iter()
        .filter(|member| user_ids.contains(&member.user_id))
        .collect();

    users
}
