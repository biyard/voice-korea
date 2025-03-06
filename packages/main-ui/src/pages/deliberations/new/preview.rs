use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

use crate::{
    components::icons::{ArrowRight, Message},
    pages::deliberations::new::{
        controller::Controller,
        i18n::{
            CompositionCommitteeSummaryTranslate, CompositionOpinionSummaryTranslate,
            CompositionPanelSummaryTranslate, InputOpinionSummaryTranslate, PreviewTranslate,
            SendAlertTranslate,
        },
    },
};

use super::controller::CurrentStep;

#[derive(Props, Clone, PartialEq)]
pub struct PreviewProps {
    lang: Language,
}

#[component]
pub fn Preview(props: PreviewProps) -> Element {
    let translate: PreviewTranslate = translate(&props.lang);
    let mut ctrl: Controller = use_context();

    rsx! {
        //FIXME: fix to real data
        div { class: "flex flex-col w-full justify-start items-start",
            CompositionOpinionSummary { lang: props.lang }
            InputOpinionSummary { lang: props.lang }
            CompositionCommitteeSummary { lang: props.lang }
            CompositionPanelSummary { lang: props.lang }

            div { class: "flex flex-row w-full justify-end items-end mt-[40px] mb-[50px]",
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
                            ctrl.open_send_alerm_modal(props.lang);
                        }
                    },
                    "{translate.start_public_opinion}"
                }
            }
        }
    }
}

#[component]
pub fn SendAlertModal(onclose: EventHandler<MouseEvent>, lang: Language) -> Element {
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
                    onclick: move |e: MouseEvent| {
                        onclose.call(e);
                    },
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

#[component]
pub fn CompositionPanelSummary(lang: Language) -> Element {
    let translate: CompositionPanelSummaryTranslate = translate(&lang);
    //FIXME: fix to real data
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start mt-[40px]",
            div { class: "font-medium text-[16px] text-black mb-[10px]",
                "{translate.participant_panel_composition}"
            }
            div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white px-[40px] py-[24px]",
                div { class: "flex flex-col w-full justify-start items-start mb-[40px]",
                    div { class: "font-bold text-[#222222] text-lg mb-[40px]",
                        "{translate.full_panel_settings}"
                    }
                    div { class: "flex flex-row w-full h-[55px] justify-start items-start",
                        div { class: "flex flex-row w-[180px] justify-start items-start mr-[50px]",
                            "{translate.select_panel}"
                        }
                        div { class: "flex flex-wrap w-full justify-start items-center gap-[30px]",
                            div { class: "flex flex-row gap-[5px]",
                                SummaryLabel { label: "패널1" }
                                div { class: "font-normal text-black text-[15px]", "15명" }
                            }
                            div { class: "flex flex-row gap-[5px]",
                                SummaryLabel { label: "패널2" }
                                div { class: "font-normal text-black text-[15px]", "15명" }
                            }
                            div { class: "flex flex-row gap-[5px]",
                                SummaryLabel { label: "패널3" }
                                div { class: "font-normal text-black text-[15px]", "15명" }
                            }
                        }
                    }
                }

                div { class: "flex flex-col w-full justify-start items-start",
                    div { class: "font-bold text-[#222222] text-lg mb-[40px]",
                        "{translate.setting_properties_for_each_panel}"
                    }
                    div { class: "flex flex-row w-full h-[55px] justify-start items-start mb-[20px]",
                        div { class: "flex flex-row w-[180px] justify-start items-start mr-[50px]",
                            "패널1"
                        }
                        div { class: "flex flex-wrap w-full justify-start items-center gap-[5px]",
                            SummaryLabel { label: "속성1" }
                            SummaryLabel { label: "속성2" }
                            SummaryLabel { label: "속성3" }
                        }
                    }
                    div { class: "flex flex-row w-full h-[55px] justify-start items-start mb-[20px]",
                        div { class: "flex flex-row w-[180px] justify-start items-start mr-[50px]",
                            "패널2"
                        }
                        div { class: "flex flex-wrap w-full justify-start items-center gap-[5px]",
                            SummaryLabel { label: "속성1" }
                            SummaryLabel { label: "속성2" }
                            SummaryLabel { label: "속성3" }
                        }
                    }
                    div { class: "flex flex-row w-full h-[55px] justify-start items-start",
                        div { class: "flex flex-row w-[180px] justify-start items-start mr-[50px]",
                            "패널3"
                        }
                        div { class: "flex flex-wrap w-full justify-start items-center gap-[5px]",
                            SummaryLabel { label: "속성1" }
                            SummaryLabel { label: "속성2" }
                            SummaryLabel { label: "속성3" }
                        }
                    }
                }

                div { class: "flex flex-row w-full justify-end items-end font-light text-[#6d6d6d] text-[14px]",
                    "총 45명 / 공평한 인원수 배정 / 패널1 15명, 패널2 15명, 패널3 15명"
                }
            }
        }
    }
}

#[component]
pub fn CompositionCommitteeSummary(lang: Language) -> Element {
    let translate: CompositionCommitteeSummaryTranslate = translate(&lang);
    let opinion_designers = use_signal(|| {
        vec![
            "보이스".to_string(),
            "보이스".to_string(),
            "보이스".to_string(),
        ]
    });
    let specific_opinion_designers = use_signal(|| {
        vec![
            "보이스".to_string(),
            "보이스".to_string(),
            "보이스".to_string(),
        ]
    });
    let analysts = use_signal(|| {
        vec![
            "보이스".to_string(),
            "보이스".to_string(),
            "보이스".to_string(),
        ]
    });
    let intermediaries = use_signal(|| {
        vec![
            "보이스".to_string(),
            "보이스".to_string(),
            "보이스".to_string(),
        ]
    });
    let lecturers = use_signal(|| {
        vec![
            "보이스".to_string(),
            "보이스".to_string(),
            "보이스".to_string(),
        ]
    });

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start mt-[40px]" }
        div { class: "font-medium text-black text-[16px] mb-[10px]",
            "{translate.composition_public_opinion_committee}"
        }
        div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white px-[40px] py-[24px]",
            div { class: "font-bold text-[#222222] text-lg mb-[20px]", "{translate.division_of_roles}" }

            div { class: "flex flex-row w-full h-[55px] justify-start items-center mb-[10px]",
                div { class: "flex flex-row w-[180px] justify-start items-start mr-[50px]",
                    "공론 설계자"
                }

                div { class: "flex flex-wrap w-full justify-start items-center p-[15px] gap-[5px]",
                    for role in opinion_designers() {
                        SummaryLabel { label: role }
                    }
                }
            }

            div { class: "flex flex-row w-full h-[55px] justify-start items-center mb-[10px]",
                div { class: "flex flex-row w-[180px] justify-start items-start mr-[50px]",
                    "특정 공론 설계자"
                }

                div { class: "flex flex-wrap w-full justify-start items-center p-[15px] gap-[5px]",
                    for role in specific_opinion_designers() {
                        SummaryLabel { label: role }
                    }
                }
            }

            div { class: "flex flex-row w-full h-[55px] justify-start items-center mb-[10px]",
                div { class: "flex flex-row w-[180px] justify-start items-start mr-[50px]",
                    "분석가"
                }

                div { class: "flex flex-wrap w-full justify-start items-center p-[15px] gap-[5px]",
                    for role in analysts() {
                        SummaryLabel { label: role }
                    }
                }
            }
            div { class: "flex flex-row w-full h-[55px] justify-start items-center mb-[10px]",
                div { class: "flex flex-row w-[180px] justify-start items-start mr-[50px]",
                    "중개자"
                }

                div { class: "flex flex-wrap w-full justify-start items-center p-[15px] gap-[5px]",
                    for role in intermediaries() {
                        SummaryLabel { label: role }
                    }
                }
            }

            div { class: "flex flex-row w-full h-[55px] justify-start items-center mb-[10px]",
                div { class: "flex flex-row w-[180px] justify-start items-start mr-[50px]",
                    "강연자"
                }

                div { class: "flex flex-wrap w-full justify-start items-center p-[15px] gap-[5px]",
                    for role in lecturers() {
                        SummaryLabel { label: role }
                    }
                }
            }

            div { class: "flex flex-row w-full justify-end items-end font-light text-[#6d6d6d] text-[14px]",
                "총 15명 / 공론 설계자 3명, 특정 공론 설계자 3명, 분석가 3명, 중계자 3명, 강연자 3명"
            }
        }
    }
}

#[component]
pub fn InputOpinionSummary(lang: Language) -> Element {
    let translate: InputOpinionSummaryTranslate = translate(&lang);
    let documents = use_signal(|| {
        vec![
            "assets.doc".to_string(),
            "assets.pdf".to_string(),
            "assets.pdf".to_string(),
            "assets.pdf".to_string(),
            "assets.pdf".to_string(),
        ]
    });

    let projects = use_signal(|| {
        vec![
            "조사주제".to_string(),
            "조사주제".to_string(),
            "조사주제".to_string(),
        ]
    });
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start mt-[40px]",
            div { class: "font-medium text-black text-[#16px] mb-[10px]",
                "{translate.input_necessary_information}"
            }
            div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white px-[40px] py-[24px]",
                div { class: "flex flex-col w-full justify-start items-start  mb-[50px]",
                    div { class: "font-bold text-[#222222] text-lg mb-[20px]",
                        "{translate.introduction}"
                    }
                    div { class: "flex flex-row w-full h-[45px] justify-start items-center px-[15px] font-medium text-[#222222] text-[18px]",
                        "공론주제입니다."
                    }
                    div { class: "flex flex-row w-full justify-start items-start border border-[#ebeff5] mt-[5px] mb-[20px]" }
                    div { class: "flex flex-row w-full justify-start items-start px-[15px] font-medium text-[#222222] text-[15px] whitespace-pre-line",
                        "공론주제의 소개내용입니다.\n공론주제의 소개내용입니다.\n공론주제의 소개내용입니다.\n공론주제의 소개내용입니다.\n공론주제의 소개내용입니다."
                    }
                }

                div { class: "flex flex-col w-full justify-start items-start mb-[55px]",
                    div { class: "font-bold text-[#222222] text-lg mb-[25px]",
                        "{translate.upload_document}"
                    }
                    div { class: "flex flex-wrap w-full justify-start items-start px-[15px] gap-[5px]",
                        for document in documents() {
                            SummaryLabel { label: document }
                        }
                    }
                }

                div { class: "flex flex-col w-full justify-start items-start mb-[25px]",
                    div { class: "font-bold text-[#222222] text-lg mb-[25px]",
                        "{translate.upload_survey_project}"
                    }
                    div { class: "flex flex-wrap w-full justify-start items-start px-[15px] gap-[5px]",
                        for project in projects() {
                            SummaryLabel { label: project }
                        }
                    }
                }

                div { class: "flex flex-row w-full justify-end items-end font-light text-[#6d6d6d] text-[14px]",
                    "총 8개 자료, 업로드된 자료 5개, 조사 프로젝트 3개"
                }
            }
        }
    }
}

#[component]
pub fn CompositionOpinionSummary(lang: Language) -> Element {
    let translate: CompositionOpinionSummaryTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "font-medium text-black text-[#16px] mb-[10px]",
                "{translate.public_opinion_composition_and_period}"
            }
            div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white px-[40px] py-[24px]",
                div { class: "font-bold text-[#222222] text-lg mb-[20px]",
                    "{translate.public_opinion_composition_and_period}"
                }
                div { class: "flex flex-wrap w-full justify-start items-center mb-[10px]",
                    CompositionOpinionSummaryCard { title: "정보 제공", date: "2.12.2025 ~ 3.12.2025" }
                    div { class: "flex justify-center items-center mx-[10px]",
                        ArrowRight { width: "12", height: "12" }
                    }
                    CompositionOpinionSummaryCard {
                        title: "토론 및 숙의",
                        date: "2.12.2025 ~ 3.12.2025",
                    }
                    div { class: "flex justify-center items-center mx-[10px]",
                        ArrowRight { width: "12", height: "12" }
                    }
                    CompositionOpinionSummaryCard { title: "의견 도출", date: "2.12.2025 ~ 3.12.2025" }
                    div { class: "flex justify-center items-center mx-[10px]",
                        ArrowRight { width: "12", height: "12" }
                    }
                    CompositionOpinionSummaryCard { title: "합의 도출", date: "2.12.2025 ~ 3.12.2025" }
                    div { class: "flex justify-center items-center mx-[10px]",
                        ArrowRight { width: "12", height: "12" }
                    }
                    CompositionOpinionSummaryCard { title: "결과 분석", date: "2.12.2025 ~ 3.12.2025" }
                }
                div { class: "flex flex-row w-full justify-end items-end font-light text-[#6d6d6d] text-[14px]",
                    "{translate.total_period} 2월 12일 2025년 - 3월 12일 2025년"
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
        div { class: "flex flex-col w-[185px] justify-center items-center bg-white border border-[#bfc8d9] px-[15px] py-[10px]",
            div { class: "font-medium text-[#222222] text-[15px] mb-[5px]", {title} }
            div { class: "font-normal text-[#6d6d6d] text-[14px]", {date} }
        }
    }
}
