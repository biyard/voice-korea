#![allow(non_snake_case, dead_code, unused_variables)]
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::{deliberation_survey::DeliberationSurvey, Tab};

use crate::{
    components::icons::triangle::{TriangleDown, TriangleUp},
    pages::projects::_id::components::final_survey::{FinalSurveyStatus, FinalSurveyTranslate},
    utils::time::current_timestamp,
};

use super::final_survey::FinalSurveyStep;

#[component]
pub fn FinalSurveyInfo(
    lang: Language,
    survey: DeliberationSurvey,
    onchange: EventHandler<FinalSurveyStep>,
) -> Element {
    let tab_title: &str = Tab::SampleSurvey.translate(&lang);
    let mut clicked1 = use_signal(|| true);
    let status = get_survey_status(survey.started_at, survey.ended_at);
    let tr: FinalSurveyTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-[10px]",
            div { class: "flex flex-col w-full h-fit bg-[#F7F7F7] gap-[20px]",

                // header
                div { class: "w-full flex flex-row justify-between items-center",
                    p { class: "mt-[28px] font-semibold text-[20px]", "{tab_title}" }
                }

                // information section
                div { class: "flex flex-col gap-[10px]",

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
            }

            div { class: "flex flex-row w-full justify-center mb-[40px]",
                div {
                    visibility: if survey.surveys.len() == 0 { "hidden" },
                    class: format!(
                        "flex flex-row px-[15px] py-[13px] {} rounded-[8px] font-bold text-white text-[16px]",
                        if status == FinalSurveyStatus::InProgress {
                            "bg-[#8095EA] cursor-pointer"
                        } else {
                            "bg-[#B4B4B4] cursor-not-allowed"
                        },
                    ),
                    onclick: move |_| {
                        if status == FinalSurveyStatus::InProgress {
                            onchange.call(FinalSurveyStep::WriteSurvey);
                        }
                    },
                    {status.translate(&lang)}
                }
            }
        }
    }
}

pub fn get_survey_status(started_at: i64, ended_at: i64) -> FinalSurveyStatus {
    let current = current_timestamp();

    if started_at > 10000000000 {
        tracing::error!("time parsing failed");
        return FinalSurveyStatus::default();
    }

    if started_at > current {
        FinalSurveyStatus::Ready
    } else if ended_at < current {
        FinalSurveyStatus::Finish
    } else {
        FinalSurveyStatus::InProgress
    }
}
