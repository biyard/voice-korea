use crate::{
    components::icons::{
        person::Person,
        right_arrow::RightArrow,
        triangle::{TriangleDown, TriangleUp},
    },
    pages::project::{
        components::sample::sample::{get_survey_status, SurveyStatus},
        i18n::SampleTranslate,
    },
};
use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::{deliberation_user::DeliberationUser, SurveyV2};

use super::sample::SurveyStep;

#[component]
pub fn SurveyInfo(
    lang: Language,
    survey: SurveyV2,
    members: Vec<DeliberationUser>,

    check_edit: bool,
    onchange: EventHandler<SurveyStep>,
) -> Element {
    let editor = 4; //FIXME: fix to connect model data
    let tr: SampleTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full justify-center items-center gap-[20px]",
            div { class: "flex flex-row w-full justify-between items-center",
                div { class: "font-semibold text-[#222222] text-[20px]", "{tr.sample_survey}" }
                div { class: "flex flex-row justify-start items-center gap-[80px]",
                    div { class: "font-medium text-black text-[15px]", {survey.period(lang)} }
                    div { class: "flex flex-row justify-center items-center gap-[20px]",
                        // FIXME: Remove comment after checking exact usage in figma
                        // div { class: "relative flex items-center",
                        //     img { class: "w-[32px] h-[32px] bg-[#CFCFCF] rounded-full z-10" }
                        //     img { class: "w-[32px] h-[32px] bg-[#8C8C8C] rounded-full -ml-2 z-20" }
                        //     img { class: "w-[32px] h-[32px] bg-[#CFCFCF] rounded-full -ml-2 z-30" }
                        //     img { class: "w-[32px] h-[32px] bg-[#8C8C8C] rounded-full -ml-2 z-40" }
                        // }
                        div { class: "flex flex-row items-center gap-[4px]",
                            //count
                            span { "{editor}" }
                            Person {}
                        }
                    }
                }
            }

            div {
                class: "flex flex-col w-full gap-[10px]",
                display: if check_edit { "flex" } else { "none" },
                SampleLinkComponent {
                    lang,
                    title: tr.written_by_me,
                    onclick: move |_| {
                        onchange.call(SurveyStep::MySurvey);
                    },
                }
                SampleLinkComponent {
                    lang,
                    title: tr.responses,
                    onclick: move |_| {
                        onchange.call(SurveyStep::Statistics);
                    },
                }
            }

            SurveyDescriptionInfo {
                lang,
                survey,
                members,

                check_edit,
                onchange,
            }
        }
    }
}

#[component]
pub fn SurveyDescriptionInfo(
    lang: Language,
    survey: SurveyV2,
    members: Vec<DeliberationUser>,

    check_edit: bool,
    onchange: EventHandler<SurveyStep>,
) -> Element {
    let mut clicked = use_signal(|| false);
    let status = get_survey_status(survey.started_at, survey.ended_at);

    rsx! {
        div {
            class: "flex flex-col w-full gap-[20px]",
            display: if check_edit { "none" } else { "flex" },
            div { class: "flex flex-col w-full rounded-[8px] bg-[#ffffff] justify-start items-start py-[14px] px-[20px] gap-[10px]",
                div {
                    class: "flex flex-col w-full  justify-start items-center text-[16px] font-bold cursor-pointer",
                    onclick: move |_| clicked.set(!clicked()),
                    div { class: "w-full flex flex-row justify-between items-center",
                        div { class: "font-bold text-[#222222] text-[16px]", "{survey.name}" }
                        if clicked() {
                            TriangleUp {}
                        } else {
                            TriangleDown {}
                        }
                    }

                    div {
                        class: "flex flex-col w-full",
                        display: if clicked() { "flex" } else { "none" },
                        div { class: "w-full h-[1px] bg-[#eeeeee] my-[12px]" }
                        div { class: "flex flex-col w-full gap-[20px]",
                            div { class: "font-bold text-[18px] text-black", "{survey.description}" }
                            div { class: "flex flex-col w-full",
                                for (i , question) in survey.questions.iter().enumerate() {
                                    div { class: "font-normal text-[15px] text-black",
                                        "{i + 1}. {question.title()}"
                                    }
                                }
                            }
                            div { class: "flex flex-wrap w-full gap-[40px]",
                                for member in members {
                                    div { class: "flex flex-row gap-[8px] justify-start items-center",
                                        div { class: "w-[40px] h-[40px] rounded-[100px] bg-[#d9d9d9]" }
                                        div { class: "font-semibold text-[12px] text-[#222222]",
                                            {member.role.translate(&lang)}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div { class: "flex flex-row w-full justify-center",
                div {
                    class: format!(
                        "flex flex-row px-[15px] py-[13px] {} rounded-[8px] font-bold text-white text-[16px]",
                        if status == SurveyStatus::InProgress {
                            "bg-[#8095EA] cursor-pointer"
                        } else {
                            "bg-[#B4B4B4]"
                        },
                    ),
                    onclick: move |_| {
                        if status == SurveyStatus::InProgress {
                            onchange.call(SurveyStep::WriteSurvey);
                        }
                    },
                    {status.translate(&lang)}
                }
            }
        }
    }
}

#[component]
pub fn SampleLinkComponent(
    lang: Language,
    title: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let tr: SampleTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-row w-full justify-between items-center px-[20px] py-[9px] bg-white rounded-[8px]",
            div { class: "font-bold text-[16px] text-[#222222]", "{title}" }
            div { class: "flex flex-row justify-start items-center gap-[5px]",
                div {
                    class: "cursor-pointer font-semibold text-[#2A60D3] text-[14px] underline",
                    onclick: move |e: Event<MouseData>| {
                        onclick.call(e);
                    },
                    "{tr.see_detail}"
                }
                RightArrow { color: "#555462" }
            }
        }
    }
}
