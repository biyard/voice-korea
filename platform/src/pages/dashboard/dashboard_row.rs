#![allow(non_snake_case)]
use crate::{
    pages::dashboard::StatusButton, routes::Route, utils::time::convert_timestamp_to_fmt_string,
};
use dioxus::prelude::*;
use models::prelude::{SurveyDraftStatus, SurveyStatus};

use super::Language;

#[component]
pub fn DashboardRow(
    lang: Language,
    survey_id: String,
    survey_status: SurveyStatus,
    survey_draft_status: Option<SurveyDraftStatus>,
    title: String,
    updated_at: i64,
    response_count: u64,
    total_response_count: u64,
    draft_label: String,
    in_progress_label: String,
    complete_label: String,
    add_question_description: String,
    edit_survey: String,
    analysis_result: String,
) -> Element {
    let navigator = use_navigator();
    let updated_at_string = convert_timestamp_to_fmt_string(updated_at, "%Y.%m.%d");
    rsx! {
        div {
            class: "flex flex-row w-full h-[110px] mb-[10px] justify-between items-center rounded-[5px] bg-white px-[30px]",
            div {
                class: "flex flex-row min-w-[500px]",
                div {
                    class: "text-[#4c4c4c] font-semibold text-[30px] mr-[50px]",
                    {title}
                }
                if survey_status == SurveyStatus::Draft {
                    div {
                        class: "flex flex-row items-center",
                        div {
                            class: "w-[20px] h-[20px] mr-[10px]",
                            img {
                                src: asset!("/public/images/info.png"),
                                width: 20,
                                height: 20,
                                alt: "Info"
                            }
                        }
                        div {
                            class: "text-[#1e5eaf] font-normal text-[20px] mt-[1px]",
                            "{add_question_description}"
                        }
                    }
                }

            }
            div {
                class: "flex flex-row justify-start items-center",
                div {
                    class: "flex flex-row items-center justify-center w-[80px] text-[#696969] font-normal text-[20px] mr-[50px]",
                    "{response_count}/{total_response_count}"
                }
                div {
                    class: "flex flex-row items-center justify-center w-[210px] text-[#696969] font-normal text-[20px] mr-[50px]",
                    "{updated_at_string}"
                }
                div {
                    class: "flex flex-col items-center justify-center w-[110px] text-[#696969] font-normal text-[20px] mr-[50px]",
                    div {
                        class: "flex flex-col w-full justify-center items-center mt-[25px]",
                        StatusButton {
                            survey_status: survey_status.clone(),
                            draft_label,
                            in_progress_label,
                            complete_label,
                        }
                    }
                }
                div {
                    class: "flex flex-row w-[265px] items-center justify-center",
                    if survey_status == SurveyStatus::Draft {
                        div {
                            class: "flex flex-row w-[200px] h-[55px] rounded-[8px] border-solid border border-[#b0b0b0] bg-white items-center justify-center",
                            onclick: move |_| {
                                if survey_draft_status == Some(SurveyDraftStatus::Title) {
                                    navigator.push(
                                        Route::WriteTitlePage {
                                            lang, survey_id: survey_id.clone(),
                                        }
                                    );
                                } else if survey_draft_status == Some(SurveyDraftStatus::Question) {
                                    navigator.push(
                                        Route::WriteQuestionPage { lang, survey_id: survey_id.clone() }
                                    );
                                } else if survey_draft_status == Some(SurveyDraftStatus::Quotas) {
                                    navigator.push(
                                        Route::SelectResponsePage { lang, survey_id: survey_id.clone() }
                                    );
                                } else {
                                    navigator.push(Route::SurveySummaryPage { lang, survey_id: survey_id.clone()});
                                }
                            },
                            div {
                                class: "text-[20px] font-medium text-[#1e5eaf]",
                                "{edit_survey}"
                            }
                        }
                    } else {
                        div {
                            class: "flex flex-row w-[200px] h-[55px] rounded-[8px] border-solid border border-[#b0b0b0] bg-white items-center justify-center",
                            onclick: move |_| {
                                navigator.push(Route::SurveySummaryPage { lang, survey_id: survey_id.clone()});
                            },
                            div {
                                class: "text-[20px] font-medium text-[#1e5eaf]",
                                "{analysis_result}"
                            }
                        }
                    }
                }
            }
        }
    }
}
