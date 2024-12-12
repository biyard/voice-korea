#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::{
    components::{bar_graph::BarGraph, pi_graph::PiGraph},
    pages::id::response_report::controller::use_controller,
};

#[derive(Props, Clone, PartialEq)]
pub struct ResponseSummaryProps {
    pub response_report: String,
    pub response_download: String,
    pub response_summary: String,
    pub total_number_of_responses: String,
    pub completion_rate: String,
    pub normal_time_required: String,
    pub most_skipped_questions: String,
    pub response_attribute: String,
    pub survey_summary: String,

    pub number_of_replies: String,
    pub number_of_skipped: String,
    pub item: String,
    pub reply: String,
    pub total: String,
}

#[component]
pub fn ResponseSummary(props: ResponseSummaryProps) -> Element {
    let ctrl = use_controller();
    rsx! {
        div {
            class: "flex flex-col w-full justify-start items-start",
            div {
                class: "text-[#2168c3] font-semibold text-[30px]",
                "{props.response_report}"
            }
            div {
                class: "flex flex-row w-full justify-end items-end mb-[20px]",
                div {
                    class: "flex flex-row w-[135px] h-[45px] justify-center items-center mt-[30px] rounded-lg bg-[#1e5eaf] text-white font-bold text-[16px]",
                    "{props.response_download}"
                }
            }
            div {
                class: "flex flex-col w-full justify-start items-start mb-[30px]",
                div {
                    class: "text-black font-semibold text-[24px] mb-[20px]",
                    "{props.response_summary}"
                }
                div {
                    class: "flex flex-wrap gap-5 justify-start items-start px-[10px]",
                    SummaryInfo {
                        label: props.total_number_of_responses,
                        value: ctrl.get_total_response()
                    }
                    SummaryInfo {
                        label: props.completion_rate,
                        //FIXME: fix to real api logic
                        value: "100%".to_string()
                    }
                    SummaryInfo {
                        label: props.normal_time_required,
                        //FIXME: fix to real api logic
                        value: "-".to_string()
                    }
                    SummaryInfo {
                        label: props.most_skipped_questions,
                        //FIXME: fix to real api logic
                        value: "없음".to_string()
                    }
                }
            }
            div {
                class: "flex flex-col w-full justify-start items-start mb-[30px]",
                div {
                    class: "text-black font-semibold text-[24px] mb-[40px]",
                    "{props.response_attribute}"
                }
                for (index, attribute) in ctrl.get_attributes().iter().enumerate() {
                    div {
                        class: "flex flex-col w-full justify-start items-start mb-[40px]",
                        div {
                            class: "text-black font-semibold text-[24px] mb-[20px]",
                            {format!("{}. {}", index + 1, attribute.label.clone())}
                        }
                        PiGraph {
                            chart_data: attribute.chart_datas.clone(),
                        }
                    }
                }
            },
            div {
                class: "flex flex-col w-full justify-start items-start mb-[30px]",
                div {
                    class: "text-black font-semibold text-[24px] mb-[40px]",
                    "{props.survey_summary}"
                }
                for (index, survey) in ctrl.get_surveys().iter().enumerate() {
                    div {
                        class: "flex flex-col w-full justify-start items-start mb-[40px]",
                        div {
                            class: "text-black font-semibold text-[20px] mb-[10px]",
                            {format!("Q{}", index + 1)}
                        }
                        div {
                            class: "px-[10px]",
                            div {
                                class: "text-[#838a90] font-semibold text-[24px] mb-[10px]",
                                "{survey.title}"
                            }
                            div {
                                class: "flex flex-row w-full justify-start items-start text-[#838a90] font-medium text-[16px] mb-[15px]",
                                div {
                                    class: "mr-[7px]",
                                    {format!("{}:", props.number_of_replies)}
                                }
                                div {
                                    class: "mr-[20px]",
                                    {format!("{}", survey.answer)}
                                }
                                div {
                                    class: "mr-[7px]",
                                    {format!("{}:", props.number_of_skipped)}
                                }
                                div {
                                    {format!("{}", survey.skipped_answer)}
                                }
                            }
                            if survey.colors.len() != 0 {
                                div {
                                    class: "flex flex-row w-full justify-start items-start mb-[20px]",
                                    BarGraph {
                                        labels: survey.labels.clone(),
                                        values: survey.value_percents.clone(),
                                        colors: survey.colors.clone(),
                                    }
                                }
                            }
                            div {
                                class: "flex flex-col w-[700px] justify-start items-start mb-[30px]",
                                if survey.colors.len() != 0 {
                                    div {
                                        class: "flex flex-row w-full h-[40px] justify-start items-center bg-[#edeeee]",
                                        div {
                                            class: "flex flex-row w-[350px] h-full justify-start items-center border border-white px-[10px]",
                                            "{props.item}"
                                        }
                                        div {
                                            class: "flex flex-row w-[350px] h-full justify-start items-center border border-white px-[10px]",
                                            "{props.reply}"
                                        }
                                    }
                                    for (i, label) in survey.labels.iter().enumerate() {
                                        div {
                                            class: "flex flex-row w-full h-[40px] justify-start items-center bg-white border border-b-[#e2e3e4] border-t-transparent border-r-transparent border-l-transparent",
                                            div {
                                                class: "flex flex-row w-[350px] h-full justify-start items-center px-[10px] overflow-hidden truncate",
                                                {label.clone()}
                                            }
                                            div {
                                                class: "flex flex-row w-[350px] h-full justify-between items-center px-[10px]",
                                                div {
                                                    "{survey.value_percents[i]}"
                                                }
                                                div {
                                                    "{survey.value_counts[i]}"
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    div {
                                        class: "flex flex-row w-full h-[40px] justify-start items-center bg-[#edeeee]",
                                        div {
                                            class: "flex flex-row w-[350px] h-full justify-start items-center border border-white px-[10px]",
                                            "{props.reply}"
                                        }
                                        div {
                                            class: "flex flex-row w-[350px] h-full justify-start items-center border border-white px-[10px]",
                                            "갯수"
                                        }
                                    }
                                    for (i, label) in survey.labels.iter().enumerate() {
                                        div {
                                            class: "flex flex-row w-full h-[40px] justify-start items-center bg-white border border-b-[#e2e3e4] border-t-transparent border-r-transparent border-l-transparent",
                                            div {
                                                class: "flex flex-row w-[350px] h-full justify-start items-center px-[10px] overflow-hidden truncate",
                                                {label.clone()}
                                            }
                                            div {
                                                class: "flex flex-row w-[350px] h-full justify-between items-center px-[10px]",
                                                div {
                                                    "{survey.value_counts[i]}"
                                                }
                                            }
                                        }
                                    }
                                }
                                div {
                                    class: "flex flex-row w-full h-[40px] justify-between items-center bg-[#edeeee] px-[10px]",
                                    div {
                                        "{props.total}"
                                    }
                                    div {
                                        {format!("{}", survey.answer + survey.skipped_answer)}
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
pub fn SummaryInfo(label: String, value: String) -> Element {
    rsx! {
        div {
            class: "flex flex-col w-[170px] h-[160px] justify-start items-start px-[10px] py-[30px] bg-[#f9f9f9] rounded-lg",
            div {
                class: "text-[16px] font-bold text-[#696969] pb-[10px]",
                {label},
            }
            div {
                class: "text-[20px] font-medium text-[#2168c3]",
                {value}
            }
        }
    }
}
