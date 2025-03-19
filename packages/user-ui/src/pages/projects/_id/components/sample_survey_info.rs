#![allow(non_snake_case, dead_code, unused_variables)]
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::{deliberation_survey::DeliberationSurvey, Tab};

use crate::{
    components::icons::{
        right_arrow::RightArrow,
        triangle::{TriangleDown, TriangleUp},
    },
    pages::projects::_id::components::sample_survey::{SampleSurveyTranslate, SurveyStatus},
    utils::time::current_timestamp,
};

use super::sample_survey::SurveyStep;

#[component]
pub fn SampleSurveyInfo(
    lang: Language,
    survey: DeliberationSurvey,
    survey_completed: bool,
    onchange: EventHandler<SurveyStep>,
) -> Element {
    let tab_title: &str = Tab::SampleSurvey.translate(&lang);
    let mut clicked1 = use_signal(|| true);
    let status = get_survey_status(survey.started_at, survey.ended_at);
    let tr: SampleSurveyTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-[10px]",
            div { class: "flex flex-col w-full h-fit bg-[#F7F7F7] gap-[20px]",

                // header
                div { class: "w-full flex flex-row justify-between items-center",
                    p { class: "mt-[28px] font-semibold text-[20px]", "{tab_title}" }
                }

                // information section
                div {
                    style: if survey_completed { "display: none;" } else { "" },
                    class: "flex flex-col gap-[10px]",

                    // introduction section
                    div { class: "w-full flex flex-col rounded-[8px] bg-[#ffffff] justify-start items-center py-[14px] px-[20px]",
                        div {
                            class: "w-full flex justify-start items-center text-[16px] font-bold cursor-pointer",
                            onclick: move |_| {
                                clicked1.set(!clicked1());
                            },
                            div { class: "w-full flex flex-row justify-between items-center",
                                span { "{tr.title}" }
                                if clicked1() {
                                    TriangleUp {}
                                } else {
                                    TriangleDown {}
                                }
                            }
                        }
                        if clicked1() {
                            //line
                            hr { class: "w-full h-[1px] mt-[12px] mb-[12px] border-[#eee]" }
                            div { class: "w-full justify-start mt-[15px] mb-[20px] font-bold text-[18px]",
                                "{survey.title}"
                            }
                            div { class: "w-full flex justify-start text-[15px]",
                                "{survey.description}"
                            }
                            div { class: "w-full mt-[20px] flex flex-row justify-start gap-[40px]",
                                for member in survey.members {
                                    div { class: "flex flex-row justify-start gap-[8px]",
                                        img { class: "w-[40px] h-[40px] bg-[#D9D9D9] rounded-full" }
                                        div { class: "flex flex-col justify-start",
                                            p { class: "font-semibold text-[15px] justify-start",
                                                {member.role.translate(&lang)}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // information section when survey completed
                div {
                    class: "flex flex-col w-full gap-[10px]",
                    style: if survey_completed { "" } else { "display: none;" },
                    SampleLinkComponent {
                        lang,
                        title: tr.my_answer,
                        onclick: move |_| {
                            onchange.call(SurveyStep::MySurvey);
                        },
                    }
                    SampleLinkComponent {
                        lang,
                        title: tr.response_per_question,
                        onclick: move |_| {
                            onchange.call(SurveyStep::Statistics);
                        },
                    }
                }
            }

            div { class: "flex flex-row w-full justify-center mb-[40px]",
                div {
                    style: if survey.surveys.is_empty() || survey_completed { "display: none;" } else { "" },
                    class: format!(
                        "flex flex-row px-[15px] py-[13px] {} rounded-[8px] font-bold text-white text-[16px]",
                        if status == SurveyStatus::InProgress {
                            "bg-[#8095EA] cursor-pointer"
                        } else {
                            "bg-[#B4B4B4] cursor-not-allowed"
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
    let tr: SampleSurveyTranslate = translate(&lang);
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

pub fn get_survey_status(started_at: i64, ended_at: i64) -> SurveyStatus {
    let current = current_timestamp();

    if started_at > 10000000000 {
        tracing::error!("time parsing failed");
        return SurveyStatus::default();
    }

    if started_at > current {
        SurveyStatus::Ready
    } else if ended_at < current {
        SurveyStatus::Finish
    } else {
        SurveyStatus::InProgress
    }
}
