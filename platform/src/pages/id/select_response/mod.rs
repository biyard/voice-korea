#![allow(non_snake_case)]
use crate::{components::button::Button, prelude::*};
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct SelectResponseProps {
    lang: Language,
    survey_id: String,
}

mod controller;
mod i18n;
pub mod response_type;

#[component]
pub fn SelectResponsePage(props: SelectResponseProps) -> Element {
    let survey_id_copy = props.survey_id.clone();
    let id_copy = props.survey_id.clone();
    let mut ctrl = controller::Controller::init(props.survey_id.clone());
    let survey_response = ctrl.get_survey();
    let question_list = survey_response.questions.len() as u64;
    let translates = i18n::translate(props.lang.clone());
    let question_list_info_first = translates.question_list_info_first.clone();
    let question_list_info_second = translates.question_list_info_second.clone();
    let question_list_info = question_list_info_first
        + question_list.to_string().as_str()
        + question_list_info_second.as_str();
    let navigator = use_navigator();

    rsx! {
        div {
            class: "flex flex-col w-full h-full justify-start items-center",
            div {
                class: "flex flex-col max-w-[1200px] min-w-[600px] w-full justify-start items-start mt-[55px] px-[50px]",
                div {
                    class: "flex flex-row w-full h-[110px] rounded-[10px] bg-white mb-[10px]",
                    div {
                        class: "flex flex-row w-full h-[110px] items-center justify-start text-[#2168c3] font-semibold text-[30px] pl-[30px]",
                        {ctrl.get_title()}
                    }
                }
                div {
                    class: "flex flex-col w-full h-[110px] rounded-[10px] bg-white mb-[10px] justify-center items-start",
                    div {
                        class: "text-black font-semibold text-[22px] pl-[30px] mb-[10px]",
                        "{translates.question_list}"
                    }
                    div {
                        class: "text-[#5e5e5e] font-normal text-[22px] pl-[30px]",
                        "{question_list_info}",
                    }
                }
                div {
                    class: "flex flex-col w-full h-min rounded-[10px] bg-white mb-[10px] justify-between items-start py-[30px] px-[30px]",
                    div {
                        class: "flex flex-col w-full justify-start items-start",
                        div {
                            class: "text-black font-semibold text-[22px] mb-[10px]",
                            "{translates.collect_response_title}"
                        }
                        div {
                            class: "text-[#5e5e5e] font-normal text-[22px] mb-[20px]",
                            "{translates.collect_response_description}"
                        }
                    }
                    div {
                        class: "flex flex-col w-full h-min justify-start items-start mb-[15px]",
                        for (i, attribute) in ctrl.get_attributes().iter().enumerate() {
                            if i % 2 == 1 {
                                AttributeRow {
                                    background_color: "#ffffff".to_string(),
                                    region: attribute.region.clone(),
                                    gender: attribute.gender.clone(),
                                    age: attribute.age.clone(),
                                    payload: attribute.payload.clone(),
                                    value: attribute.value,
                                }
                            } else {
                                AttributeRow {
                                    background_color: "#f9f9f9".to_string(),
                                    region: attribute.region.clone(),
                                    gender: attribute.gender.clone(),
                                    age: attribute.age.clone(),
                                    payload: attribute.payload.clone(),
                                    value: attribute.value,
                                }
                            }
                        }
                    }
                    div {
                        class: "flex flex-row w-full justify-end items-end",
                        Button {
                            button_text: translates.add_attribute,
                            onclick: move |_| {
                                navigator.push(Route::SelectResponseDetailPage {
                                    lang: props.lang.clone(),
                                    survey_id: survey_id_copy.clone(),
                                    select_type: "attribute".to_string(),
                                });
                            },
                            class: "flex flex-row w-[160px] h-[50px] bg-[#1e5eaf]",
                        }
                    }
                    // div {
                    //     class: "flex flex-row w-full justify-start items-start",
                    //     div {
                    //         class: "flex flex-1 rounded-xl h-[240px] mr-[10px]",
                    //         style: "background-image: url('{RESPONSE_ATTRIBUTE_IMAGE}');",
                    //         onclick: move |_| {
                    //             navigator.push(Route::SelectResponseDetailPage {
                    //                 lang: props.lang.clone(),
                    //                 survey_id: survey_id_copy.clone(),
                    //                 select_type: "attribute".to_string()
                    //             });
                    //         },
                    //         div {
                    //             class: "flex flex-col w-full h-full justify-center items-center",
                    //             div {
                    //                 class: "text-[28px] font-semibold text-white mb-[20px]",
                    //                 "{translates.select_response_attribute_title}"
                    //             }
                    //             div {
                    //                 class: "flex flex-row w-full justify-center items-center text-[16px] font-normal text-white whitespace-pre-line content-center px-[10px]",
                    //                 "{translates.select_response_attribute_description}"
                    //             }
                    //         }
                    //     }
                    //     div {
                    //         class: "flex flex-1 rounded-xl h-[240px]",
                    //         style: "background-image: url('{RESPONSE_PANEL_IMAGE}');",
                    //         onclick: move |_| {
                    //             navigator.push(Route::SelectResponseDetailPage {
                    //                 lang: props.lang.clone(),
                    //                 survey_id: survey_id_copy_v2.clone(),
                    //                 select_type: "panel".to_string()
                    //             });
                    //         },
                    //         div {
                    //             class: "flex flex-col w-full h-full justify-center items-center",
                    //             div {
                    //                 class: "text-[28px] font-semibold text-white mb-[20px]",

                    //                 "{translates.select_response_panel_title}"
                    //             }
                    //             div {
                    //                 class: "flex flex-row w-full justify-center items-center text-[16px] font-normal text-white whitespace-pre-line content-center px-[10px]",

                    //                 "{translates.select_response_panel_description}"
                    //             }
                    //         }
                    //     }
                    // }
                }
                div {
                    class: "flex flex-row w-full justify-end items-end mb-[20px]",
                    Button {
                        button_text: translates.back,
                        onclick: move |_| {
                            let survey_id = props.survey_id.clone();
                            async move {
                                ctrl.back_button_clicked().await;
                                navigator.push(Route::WriteQuestionPage {
                                    lang: props.lang.clone(),
                                    survey_id: survey_id.clone(),
                                });
                            }
                        },
                        class: "flex flex-row w-[200px] h-[50px] bg-[#434343] mr-[15px]",
                    }
                    Button {
                        button_text: translates.save,
                        onclick: move |_| {
                            let survey_id = id_copy.clone();
                            async move {
                                ctrl.save_button_clicked().await;
                                navigator.push(Route::SurveySummaryPage {
                                    lang: props.lang.clone(),
                                    survey_id: survey_id.clone(),
                                });
                            }
                        },
                        class: "flex flex-row w-[200px] h-[50px] bg-[#1e5eaf]",
                    }
                }
            }
        }
    }
}

#[component]
fn AttributeRow(
    background_color: String,
    region: String,
    gender: String,
    age: String,
    payload: String,
    value: u64,
) -> Element {
    rsx! {
        div {
            class: "flex flex-row w-full h-[100px] justify-between items-start p-[30px]",
            style: format!("background-color: {}", background_color),
            div {
                class: "flex flex-row w-full justify-start items-start",
                if region != "" {
                    div {
                        class: "w-[70px] text-[#5e5e5e] font-normal text-[20px] pr-[20px]",
                        {region.clone()}
                    }
                }
                if gender != "" {
                    div {
                        class: "w-[70px] text-[#5e5e5e] font-normal text-[20px] pr-[20px]",
                        {gender.clone()}
                    }
                }
                if age != "" {
                    div {
                        class: "w-[120px] text-[#5e5e5e] font-normal text-[20px] pr-[20px]",
                        {age.clone()}
                    }
                }
                if payload != "" {
                    div {
                        class: "text-[#5e5e5e] font-normal text-[20px] pr-[20px]",
                        {payload.clone()}
                    }
                }
            }
            div {
                class: "flex flex-row w-min justify-start items-start",
                div {
                    class: "text-black font-medium text-[20px] mr-[5px]",
                    "총 "
                }
                div {
                    class: "text-[#1e5eaf] font-medium text-[20px]",
                    {value.to_string()}
                }
                div {
                    class: "text-black font-medium text-[20px]",
                    "명"
                }
            }
        }
    }
}
