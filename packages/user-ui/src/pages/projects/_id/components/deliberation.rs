#![allow(non_snake_case, dead_code, unused_variables)]
use by_components::icons::upload_download::Download2;
use by_macros::*;
use dioxus::prelude::*;
use dioxus_translate::*;
use models::Tab;

use crate::components::icons::triangle::{TriangleDown, TriangleUp};

#[component]
pub fn Deliberation(
    lang: Language,
    project_id: ReadOnlySignal<i64>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctrl = Controller::new(lang, project_id)?;
    let tr: DeliberationTranslate = translate(&lang);
    let tab_title: &str = Tab::Deliberation.translate(&lang);
    let mut clicked1 = use_signal(|| false);
    let mut clicked2 = use_signal(|| false);
    let watched_seconds = use_signal(|| 120);
    let total_seconds = use_signal(|| 300);
    let watched_pages = use_signal(|| 5);
    let total_pages = use_signal(|| 10);

    rsx! {
        div { class: "w-full h-auto  bg-[#F7F7F7]",
            // header
            div { class: "w-full h-[32px] mb-[20px] flex flex-row justify-between items-center",
                p { class: "mt-[28px] font-semibold text-[20px]", "{tab_title}" }
            }
            // information section
            div { class: "flex flex-col gap-[10px]",

                // introduction section
                div { class: "w-full flex flex-col rounded-[8px] bg-[#ffffff] justify-start items-center py-[14px] px-[20px]",
                    div {
                        class: "w-full flex justify-start items-center text-[16px] font-bold cursor-pointer",
                        onclick: move |_| {
                            clicked1.set(!(*clicked1)());
                            clicked2.set(false);
                        },
                        div { class: "w-full flex flex-row justify-between items-center",
                            span { "{tr.main_title}" }
                            if (*clicked1)() {
                                TriangleUp {}
                            } else {
                                TriangleDown {}
                            }
                        }
                    }
                    if (*clicked1)() {
                        //line
                        hr { class: "w-full h-[1px] mt-[12px] mb-[12px] border-[#eee]" }
                        div { class: "w-full justify-start mt-[15px] mb-[20px] font-bold text-[18px]",
                            "제목 구간입니다(Title)."
                        }
                        div { class: "w-full flex justify-start text-[15px]",
                            "내용 구간입니다(details)."
                        }
                        div { class: "w-full mt-[20px] flex flex-row justify-start gap-[40px]",
                            //user information
                            div { class: "flex flex-row justify-start gap-[8px]",
                                img { class: "w-[40px] h-[40px] bg-[#D9D9D9] rounded-full" }
                                div { class: "flex flex-col justify-start",
                                    //user name
                                    p { class: "font-semibold text-[15px] justify-start",
                                        "id"
                                    }
                                    // Affiliated DAO
                                    p { class: "font-semibold text-[12px] justify-start",
                                        "DAO"
                                    }
                                }
                            }
                            div { class: "flex flex-row justify-start gap-[8px]",
                                img { class: "w-[40px] h-[40px] bg-[#D9D9D9] rounded-full" }
                                div { class: "flex flex-col justify-start",
                                    //user name
                                    p { class: "font-semibold text-[15px] justify-start",
                                        "id"
                                    }
                                    // Affiliated DAO
                                    p { class: "font-semibold text-[12px] justify-start",
                                        "DAO"
                                    }
                                }
                            }
                            div { class: "flex flex-row justify-start gap-[8px]",
                                img { class: "w-[40px] h-[40px] bg-[#D9D9D9] rounded-full" }
                                div { class: "flex flex-col justify-start",
                                    //user name
                                    p { class: "font-semibold text-[15px] justify-start",
                                        "id"
                                    }
                                    // Affiliated DAO
                                    p { class: "font-semibold text-[12px] justify-start",
                                        "DAO"
                                    }
                                }
                            }
                        }
                    }
                }

                //e learning
                div { class: "w-full flex flex-col rounded-[8px] bg-[#ffffff] justify-start items-center py-[14px] px-[20px] mb-[10px]",
                    div {
                        class: "w-full flex justify-start items-center text-[16px] font-bold cursor-pointer",
                        onclick: move |_| {
                            clicked2.set(!(*clicked2)());
                            clicked1.set(false);
                        },
                        div { class: "w-full flex flex-row justify-between items-center",
                            span { "{tr.e_learning_title}" }
                            if (*clicked2)() {
                                TriangleUp {}
                            } else {
                                TriangleDown {}
                            }
                        }
                    }
                    if (*clicked2)() {
                        //e learning section
                        hr { class: "w-full h-[1px] mt-[12px] mb-[12px] border-[#eee]" }
                        div { class: "w-full max-h-[170] py-[12px] flex flex-row justify-start gap-[20px]",
                            // TODO(web): have to import image data
                            img { class: "w-[240px] h-[150px] rounded-[8px] border-none",
                                "image data"
                            }
                            div { class: "w-full h-[150px] flex flex-col justify-between",
                                p { class: "w-full flex flex-col justify-start",
                                    p { class: "text-[14px]", "e-Book" }
                                    // TODO(web): have to connect data
                                    p { class: "font-bold text-[18px]",
                                        "지역사회 교통 개선 기초 수업"
                                    }
                                    div { class: "max-w-[500px] flex flex-row gap-[20px]",
                                        div { class: "w-full flex items-center" }
                                    }
                                }
                                //user information
                                div { class: "flex flex-row justify-start gap-[8px]",
                                    img { class: "w-[40px] h-[40px] bg-[#D9D9D9] rounded-full" }
                                    div { class: "flex flex-col justify-start",
                                        //user name
                                        p { class: "font-semibold text-[15px] justify-start",
                                            "id"
                                        }
                                        // Affiliated DAO
                                        p { class: "font-semibold text-[12px] justify-start",
                                            "DAO"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        //Related Data
        div { class: "w-full flex flex-col rounded-[8px] mb-[40px] bg-[#ffffff] justify-start items-center py-[14px] px-[20px]",
            // title and button
            div { class: "w-full flex justify-start items-center gap-[13px]",
                div { class: "w-[180px] flex flex-row items-center text-[16px] font-bold",
                    span { "{tr.deliberation_materials_title}" }
                }
                //file
                div { class: "min-w-[195px] min-h-[26px] flex flex- row justify-center items-center rounded-[100px] bg-[#7C8292] gap-[4px] px-[12px] py-[4px]",
                    // TODO: should be check DownloadIcon color
                    Download2 { class: "[&>path]:stroke-[#ffffff] [&>path]:fill-[#ffffff]" }
                    div { class: "text-[14px] text-white", "지역사회 교통 개선 프로젝트" }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    project_id: ReadOnlySignal<i64>,
}

impl Controller {
    pub fn new(
        lang: Language,
        project_id: ReadOnlySignal<i64>,
    ) -> std::result::Result<Self, RenderError> {
        let ctrl = Self { lang, project_id };

        Ok(ctrl)
    }
}

translate! {
    DeliberationTranslate;

    main_title: {
        ko: "주요 내용",
        en: "Highlights"
    }

    e_learning_title: {
        ko: "e-Learning",
        en: "e-Learning"
    }

    deliberation_materials_title: {
        ko: "숙의 자료",
        en: "Deliberation materials"
    }
}
