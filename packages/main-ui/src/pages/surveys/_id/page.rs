#![allow(non_snake_case)]
use by_components::charts::{
    bar_chart::{BarChart, BarChartData},
    horizontal_bar::HorizontalBar,
    pie_chart::*,
    StackBarChart, StackBarData,
};
use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::SurveyV2;
use num_format::{Locale, ToFormattedString};

use crate::{
    components::icons::ArrowLeft,
    pages::surveys::_id::{controller::Controller, i18n::SurveyResultTranslate},
    routes::Route,
    utils::time::{convert_timestamp_to_date, format_remaining_time},
};

#[component]
pub fn SurveyResultPage(lang: Language, survey_id: i64) -> Element {
    let ctrl = Controller::new(lang, survey_id);
    let tr: SurveyResultTranslate = translate(&lang);

    let survey = ctrl.get_survey();
    if survey.is_none() {
        return rsx! {};
    }

    let survey = survey.unwrap();

    rsx! {
        document::Script { src: "https://cdn.jsdelivr.net/npm/d3@7" }
        div { class: "w-full flex flex-col gap-[40px] items-start justify-start",
            Nav {
                lang,
                name: "{survey.name}",
                menu: "{tr.survey_management} / {tr.update_survey}",
            }

            div { class: "w-full flex flex-col gap-[10px]",

                div { class: "w-full flex flex-row items-center justify-end gap-[20px]",
                    PrimaryButton {
                        onclick: move |_| async move {
                            ctrl.simulate_response().await;
                        },

                        "{tr.simulate_response}"
                    }
                    PrimaryButton {
                        onclick: move |_| async move {
                            ctrl.download_excel().await;
                        },
                        "{tr.download_excel}"
                    }
                }

                div { class: "flex flex-col gap-[20px] items-start justify-center",
                    SurveySummaryReport { lang, survey }
                    SurveyPanelReport {}
                }
            }
        }
    }
}

#[component]
pub fn PrimaryButton(children: Element, onclick: EventHandler<()>) -> Element {
    rsx! {
        button {
            class: "flex items-center justify-center px-[20px] py-[14px] text-[16px] text-white bg-[#2A60D3] rounded-[4px]",
            onclick: move |_| onclick(()),
            {children}
        }
    }
}

// FIXME: breadcrumb should be placed in layout.
#[component]
pub fn Nav(lang: Language, menu: String, name: String) -> Element {
    rsx! {
        div { class: "flex flex-col gap-[10px]",
            div { class: "text-[#b4b4b4] font-medium text-[14px] mb-[10px]", "{menu}" }
            div { class: "flex flex-row w-full justify-start items-center",
                Link { class: "mr-[6px]", to: Route::SurveyPage { lang: lang },
                    ArrowLeft { width: "24", height: "24", color: "#555462" }
                }
                div { class: "text-[#222222] font-semibold text-[28px]", "{name}" }
            }
        }
    }
}

#[component]
pub fn SurveyPanelReport() -> Element {
    // FIXME: testing code for charts
    rsx! {
        div { class: "w-full flex flex-col bg-white gap-[10px] p-[20px]",
            StackBarChart {
                class: "w-full flex flex-col gap-[10px] rounded-[8px] overflow-hidden",
                height: "54px",
                data: vec![
                    StackBarData::new("패널1".to_string(), 700),
                    StackBarData::new("패널2".to_string(), 300),
                    StackBarData::new("패널3".to_string(), 200),
                    StackBarData::new("패널4".to_string(), 300),
                    StackBarData::new("패널5".to_string(), 500),
                ],
            }
            HorizontalBar {
                width: "500px",
                width: "500px",
                height: "23px",
                value: 300,
                max_value: 1000,
                class: "flex flex-row bg-[#EEEEEE] rounded-[6px] overflow-hidden",
            }
            PieChart {
                width: "500px",
                height: "500px",
                class: "w-[500px]",
                data: vec![
                    PieChartData::new("A".to_string(), 10),
                    PieChartData::new("B".to_string(), 20),
                    PieChartData::new("c".to_string(), 100),
                    PieChartData::new("d".to_string(), 50),
                    PieChartData::new("i".to_string(), 10),
                    PieChartData::new("k".to_string(), 10),
                ],
            }
            BarChart {
                width: "1000px",
                height: "500px",
                data: vec![
                    BarChartData::new("A".to_string(), 500),
                    BarChartData::new("B".to_string(), 0),
                    BarChartData::new("C".to_string(), 0),
                    BarChartData::new("D".to_string(), 600),
                    BarChartData::new("E".to_string(), 1000),
                ],
            }
        }
    }
}

#[component]
pub fn SurveySummaryReport(lang: Language, survey: SurveyV2) -> Element {
    let tr: SurveyResultTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-row w-full justify-start items-start gap-[10px]",
            SurveyResponseBox {
                title: "{tr.total_survey_target}",
                value: survey.quotes.to_formatted_string(&Locale::en),
            }
            SurveyResponseBox {
                title: "{tr.number_of_responses}",
                value: survey.response_count.to_formatted_string(&Locale::en),
            }
            SurveyResponseBox {
                title: "{tr.rate_of_responses}",
                value: if survey.quotes == 0 { "0%" } else { "{survey.response_count * 100 / survey.quotes}%" },
            }
            SurveyResponseBox {
                title: "{tr.remaining_period}",
                value: "{format_remaining_time(survey.ended_at)}",
            }
            SurveyResponseBox {
                title: "{tr.survey_period}",
                value: "{convert_timestamp_to_date(survey.started_at)} - {convert_timestamp_to_date(survey.ended_at)}",
            }
        }
    }
}

#[component]
pub fn SurveyResponseBox(title: String, value: String) -> Element {
    rsx! {
        div { class: "flex flex-col justify-center items-center py-[18px] px-[24px] gap-[20px] rounded-[8px] border border-[#ebeff5] bg-[#ffffff]",
            div { class: "font-semibold text-[#35343f] text-[15px]", "{title}" }
            div { class: "font-bold text-[#435393] text-[24px]", "{value}" }
        }
    }
}
