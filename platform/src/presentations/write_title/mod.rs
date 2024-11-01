#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;

mod controller;
mod i18n;

#[derive(PartialEq, Props, Clone)]
pub struct WriteTitleProps {
    lang: Language,
    id: String,
}

#[component]
pub fn WriteTitlePage(props: WriteTitleProps) -> Element {
    let mut ctrl = controller::Controller::init();
    let translates = i18n::translate(props.lang.clone());
    let navigator = use_navigator();

    rsx! {
        div {
            class: "flex flex-col w-full h-full justify-start items-center",
            div {
                class: "flex flex-col max-w-[1200px] min-w-[600px] w-full justify-start items-end mt-[45px] px-[50px]",
                div {
                    class: "flex flex-row w-[250px] h-[55px] rounded-[8px] bg-[#2168c3] justify-center items-center text-[21px] font-semibold text-white",
                    {translates.temporary_storage}
                }
            }
            div {
                class: "flex flex-col max-w-[1200px] min-w-[600px] w-full h-full justify-start items-start mt-[25px] px-[50px]",
                div {
                    class: "flex flex-col w-full justify-start items-start max-w-[1200px] h-[300px] rounded-[10px] border-solid border border-[#9f9f9f] bg-white px-[33px] py-[55px]",
                    div {
                        class: "text-[#2168c3] font-semibold text-[30px] mb-[20px]",
                        {translates.write_title}
                    },
                    div {
                        class: "flex flex-row w-full h-[55px] justify-start items-start mb-[20px]",
                        input {
                            class: "flex flex-1 text-[21px] text-[#8a8a8a] font-normal",
                            "type": "text",
                            style: "border:0px; padding: 5px; border-color: transparent; outline-style: none; box-shadow: none; border-bottom: 1px solid #9f9f9f;",
                            placeholder: translates.write_title_hint,
                            value: ctrl.get_survey_title(),
                            onchange: move |e| {
                                ctrl.set_survey_title(e.value());
                            },
                        }
                    },
                    div {
                        class: "flex flex-row w-full justify-end items-start mb-[20px]",
                        Link {
                            to: Route::DashboardPage {
                                lang: props.lang.clone(),
                            },
                            div {
                                class: "flex flex-row w-[85px] h-[45px] justify-center items-center rounded-[5px] bg-[#424242] text-[20px] font-normal text-white mr-[7px]",
                                {translates.cancel}
                            }
                        }
                        div {
                            class: "flex flex-row w-[85px] h-[45px] justify-center items-center rounded-[5px] bg-[#2168c3] text-[20px] font-normal text-white mr-[7px]",
                            onclick: move |_| async move {
                                let title = ctrl.get_survey_title();
                                ctrl.write_survey_title(title).await;
                                navigator
                                    .push(Route::WriteQuestionPage {
                                        lang: props.lang.clone(),
                                        title: ctrl.get_survey_title(),
                                    });
                            },
                            {translates.store}
                        }
                    }
                }
            }
        }
    }
}
