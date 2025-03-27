use bdk::prelude::*;
use by_components::{charts::horizontal_bar::HorizontalBar, icons::validations::Clear};

use crate::{
    components::icons::{right_arrow::RightArrow, Logo},
    pages::education::{controller::Controller, i18n::EducationTranslate},
};

#[component]
pub fn EducationPage(lang: Language, resource_id: i64) -> Element {
    let _ctrl = Controller::new(lang, resource_id)?;

    rsx! {
        div { class: "fixed top-0 left-0 w-screen h-screen bg-key-gray",
            div { class: "flex flex-col w-full h-full justify-start items-start gap-16 px-[60px]",
                div { class: "flex flex-col w-full justify-start items-start gap-8",
                    HeaderSection { lang }
                    ProgressSection { lang }
                }

                div { class: "flex flex-row w-full h-full justify-center items-center",
                    PdfSection {}
                }
            }
        }
    }
}

#[component]
pub fn PdfSection() -> Element {
    rsx! {
        div { class: "flex flex-row max-w-[1300px] w-full h-full bg-[#1F1E2D]" }
    }
}

#[component]
pub fn ProgressSection(lang: Language) -> Element {
    //FIXME: fix to connect data
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-5",
            div { class: "flex flex-row w-full justify-between items-center font-semibold text-lg text-white leading-6",
                div { "지역 사회 교통 개선 프로젝트입니다." }
                div { "29% 남음" }
            }
            HorizontalBar {
                id: "education horizontal bar",
                value: 29,
                height: "6px",
                max_value: 100,
                class: "flex flex-row w-full h-full bg-[#000000] opacity-80 rounded-[10px] [&>rect]:bg-[#65CD99]",
            }
        }
    }
}

#[component]
pub fn HeaderSection(lang: Language) -> Element {
    let tr: EducationTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-row w-full justify-start items-start h-20 p-3",
            div { class: "flex flex-col w-full",
                div { class: "flex flex-row w-full justify-end items-end",
                    Clear {
                        class: "[&>path]:stroke-[#BFC8D9]",
                        width: "24",
                        height: "24",
                        fill: "#BFC8D9",
                    }
                }

                div { class: "flex flex-row w-full justify-start items-start",
                    div { class: "flex flex-row flex-1 justify-center items-center",
                        div { class: "flex flex-row gap-1.5 justify-start items-center w-fit",
                            Logo {
                                width: "40",
                                height: "25",
                                class: "fill-[#555462]",
                            }
                            div { class: "font-extrabold text-white text-sm", "{tr.voice_korea}" }
                        }
                    }

                    div { class: "flex flex-row px-2.5 py-1.5 rounded-lg border border-light-gray gap-1.5 mr-12",
                        div { class: "font-bold text-sm text-white leading-6", "{tr.next_education}" }
                        RightArrow {}
                    }
                }
            }
        }
    }
}
