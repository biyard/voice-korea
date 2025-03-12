use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::{step::StepCreateRequest, OrganizationMemberSummary, PanelV2Summary};

use crate::{
    components::icons::{ArrowRight, Message},
    pages::deliberations::new::{
        components::preview_component::PreviewComponent,
        controller::Controller,
        i18n::{
            CompositionOpinionSummaryTranslate, InputOpinionSummaryTranslate, PreviewTranslate,
            SendAlertTranslate,
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
    committee_users: Vec<OrganizationMemberSummary>,
    selected_panels: Vec<PanelV2Summary>,

    onstep: EventHandler<CurrentStep>,
) -> Element {
    let translate: PreviewTranslate = translate(&lang);
    let mut ctrl: Controller = use_context();

    rsx! {
        //FIXME: fix to real data
        div { class: "flex flex-col w-full justify-start items-start gap-[40px]",
            CompositionOpinionSummary { lang, sequences, onstep }
            InputOpinionSummary { lang, informations, onstep }
            // CompositionCommitteeSummary { lang }
            // CompositionPanelSummary { lang }

            div { class: "flex flex-row w-full justify-end items-end mb-[50px]",
                div {
                    class: "flex flex-row w-[70px] h-[55px] rounded-[4px] justify-center items-center bg-white border border-[#bfc8d9] font-semibold text-[16px] text-[#555462] mr-[20px]",
                    onclick: move |_| {
                        ctrl.change_step(CurrentStep::DiscussionSetting);
                    },
                    "{translate.backward}"
                }
                div {
                    class: "cursor-pointer flex flex-row w-[130px] h-[55px] rounded-[4px] justify-center items-center bg-[#2a60d3] font-semibold text-[16px] text-white",
                    onclick: {
                        move |_| {
                            ctrl.open_send_alerm_modal(lang);
                        }
                    },
                    "{translate.start_public_opinion}"
                }
            }
        }
    }
}

#[component]
pub fn SendAlertModal(
    onclose: EventHandler<MouseEvent>,
    onclick: EventHandler<MouseEvent>,
    lang: Language,
) -> Element {
    let translate: SendAlertTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-center items-center",
            div { class: "font-normal text-[#222222] text-[14px] mb-[20px]",
                "{translate.send_alert_description}"
            }
            Message { width: "100", height: "100" }
            div { class: "flex flex-row w-full justify-center items-center font-normal text-[#6d6d6d] text-[14px] mt-[10px] mb-[20px]",
                "총 50명 선택 / 패널 4개 선택"
            }
            div { class: "flex flex-row w-full justify-center items-center gap-[20px]",
                div {
                    class: "flex flex-row w-[75px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-[4px] font-semibold text-[16px] text-white",
                    onclick,
                    "{translate.send}"
                }
                button {
                    class: "flex flex-row w-[60px] h-[40px] justify-center items-center bg-white font-semibold text-[#222222] text-[16px]",
                    onclick: move |e: MouseEvent| {
                        onclose.call(e);
                    },
                    "{translate.cancel}"
                }
            }
        }
    }
}

// #[component]
// pub fn CompositionPanelSummary(lang: Language) -> Element {
//     let translate: CompositionPanelSummaryTranslate = translate(&lang);
//     //FIXME: fix to real data
//     rsx! {
//         div { class: "flex flex-col w-full justify-start items-start mt-[40px]",
//             div { class: "font-medium text-[16px] text-black mb-[10px]",
//                 "{translate.participant_panel_composition}"
//             }
//             div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white px-[40px] py-[24px]",
//                 div { class: "flex flex-col w-full justify-start items-start mb-[40px]",
//                     div { class: "font-bold text-[#222222] text-lg mb-[40px]",
//                         "{translate.full_panel_settings}"
//                     }
//                     div { class: "flex flex-row w-full h-[55px] justify-start items-start",
//                         div { class: "flex flex-row w-[180px] justify-start items-start mr-[50px]",
//                             "{translate.select_panel}"
//                         }
//                         div { class: "flex flex-wrap w-full justify-start items-center gap-[30px]",
//                             div { class: "flex flex-row gap-[5px]",
//                                 SummaryLabel { label: "패널1" }
//                                 div { class: "font-normal text-black text-[15px]", "15명" }
//                             }
//                             div { class: "flex flex-row gap-[5px]",
//                                 SummaryLabel { label: "패널2" }
//                                 div { class: "font-normal text-black text-[15px]", "15명" }
//                             }
//                             div { class: "flex flex-row gap-[5px]",
//                                 SummaryLabel { label: "패널3" }
//                                 div { class: "font-normal text-black text-[15px]", "15명" }
//                             }
//                         }
//                     }
//                 }

//                 div { class: "flex flex-col w-full justify-start items-start",
//                     div { class: "font-bold text-[#222222] text-lg mb-[40px]",
//                         "{translate.setting_properties_for_each_panel}"
//                     }
//                     div { class: "flex flex-row w-full h-[55px] justify-start items-start mb-[20px]",
//                         div { class: "flex flex-row w-[180px] justify-start items-start mr-[50px]",
//                             "패널1"
//                         }
//                         div { class: "flex flex-wrap w-full justify-start items-center gap-[5px]",
//                             SummaryLabel { label: "속성1" }
//                             SummaryLabel { label: "속성2" }
//                             SummaryLabel { label: "속성3" }
//                         }
//                     }
//                     div { class: "flex flex-row w-full h-[55px] justify-start items-start mb-[20px]",
//                         div { class: "flex flex-row w-[180px] justify-start items-start mr-[50px]",
//                             "패널2"
//                         }
//                         div { class: "flex flex-wrap w-full justify-start items-center gap-[5px]",
//                             SummaryLabel { label: "속성1" }
//                             SummaryLabel { label: "속성2" }
//                             SummaryLabel { label: "속성3" }
//                         }
//                     }
//                     div { class: "flex flex-row w-full h-[55px] justify-start items-start",
//                         div { class: "flex flex-row w-[180px] justify-start items-start mr-[50px]",
//                             "패널3"
//                         }
//                         div { class: "flex flex-wrap w-full justify-start items-center gap-[5px]",
//                             SummaryLabel { label: "속성1" }
//                             SummaryLabel { label: "속성2" }
//                             SummaryLabel { label: "속성3" }
//                         }
//                     }
//                 }

//                 div { class: "flex flex-row w-full justify-end items-end font-light text-[#6d6d6d] text-[14px]",
//                     "총 45명 / 공평한 인원수 배정 / 패널1 15명, 패널2 15명, 패널3 15명"
//                 }
//             }
//         }
//     }
// }

// #[component]
// pub fn CompositionCommitteeSummary(lang: Language) -> Element {
//     let translate: CompositionCommitteeSummaryTranslate = translate(&lang);
//     let opinion_designers = use_signal(|| {
//         vec![
//             "보이스".to_string(),
//             "보이스".to_string(),
//             "보이스".to_string(),
//         ]
//     });
//     let specific_opinion_designers = use_signal(|| {
//         vec![
//             "보이스".to_string(),
//             "보이스".to_string(),
//             "보이스".to_string(),
//         ]
//     });
//     let analysts = use_signal(|| {
//         vec![
//             "보이스".to_string(),
//             "보이스".to_string(),
//             "보이스".to_string(),
//         ]
//     });
//     let intermediaries = use_signal(|| {
//         vec![
//             "보이스".to_string(),
//             "보이스".to_string(),
//             "보이스".to_string(),
//         ]
//     });
//     let lecturers = use_signal(|| {
//         vec![
//             "보이스".to_string(),
//             "보이스".to_string(),
//             "보이스".to_string(),
//         ]
//     });

//     rsx! {
//         div { class: "flex flex-col w-full justify-start items-start mt-[40px]" }
//         div { class: "font-medium text-black text-[16px] mb-[10px]",
//             "{translate.composition_public_opinion_committee}"
//         }
//         div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white px-[40px] py-[24px]",
//             div { class: "font-bold text-[#222222] text-lg mb-[20px]", "{translate.division_of_roles}" }

//             div { class: "flex flex-row w-full h-[55px] justify-start items-center mb-[10px]",
//                 div { class: "flex flex-row w-[180px] justify-start items-start mr-[50px]",
//                     "공론 설계자"
//                 }

//                 div { class: "flex flex-wrap w-full justify-start items-center p-[15px] gap-[5px]",
//                     for role in opinion_designers() {
//                         SummaryLabel { label: role }
//                     }
//                 }
//             }

//             div { class: "flex flex-row w-full h-[55px] justify-start items-center mb-[10px]",
//                 div { class: "flex flex-row w-[180px] justify-start items-start mr-[50px]",
//                     "특정 공론 설계자"
//                 }

//                 div { class: "flex flex-wrap w-full justify-start items-center p-[15px] gap-[5px]",
//                     for role in specific_opinion_designers() {
//                         SummaryLabel { label: role }
//                     }
//                 }
//             }

//             div { class: "flex flex-row w-full h-[55px] justify-start items-center mb-[10px]",
//                 div { class: "flex flex-row w-[180px] justify-start items-start mr-[50px]",
//                     "분석가"
//                 }

//                 div { class: "flex flex-wrap w-full justify-start items-center p-[15px] gap-[5px]",
//                     for role in analysts() {
//                         SummaryLabel { label: role }
//                     }
//                 }
//             }
//             div { class: "flex flex-row w-full h-[55px] justify-start items-center mb-[10px]",
//                 div { class: "flex flex-row w-[180px] justify-start items-start mr-[50px]",
//                     "중개자"
//                 }

//                 div { class: "flex flex-wrap w-full justify-start items-center p-[15px] gap-[5px]",
//                     for role in intermediaries() {
//                         SummaryLabel { label: role }
//                     }
//                 }
//             }

//             div { class: "flex flex-row w-full h-[55px] justify-start items-center mb-[10px]",
//                 div { class: "flex flex-row w-[180px] justify-start items-start mr-[50px]",
//                     "강연자"
//                 }

//                 div { class: "flex flex-wrap w-full justify-start items-center p-[15px] gap-[5px]",
//                     for role in lecturers() {
//                         SummaryLabel { label: role }
//                     }
//                 }
//             }

//             div { class: "flex flex-row w-full justify-end items-end font-light text-[#6d6d6d] text-[14px]",
//                 "총 15명 / 공론 설계자 3명, 특정 공론 설계자 3명, 분석가 3명, 중계자 3명, 강연자 3명"
//             }
//         }
//     }
// }

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
