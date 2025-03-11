#![allow(non_snake_case, dead_code, unused_variables)]
use by_macros::*;
use dioxus::prelude::*;
use dioxus_translate::*;
use models::Tab;

use crate::components::icons::{
    download::DownloadIcon,
    triangle::{TriangleDown, TriangleUp},
};
#[component]
pub fn BasicInfo(lang: Language) -> Element {
    let tr: BasicInfoTranslate = translate(&lang);
    let mut clicked1 = use_signal(|| false);
    let mut clicked2 = use_signal(|| false);
    let tab_title: &str = Tab::BasicInfo.translate(&lang);

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

                // public opinion committee section
                div { class: "w-full flex flex-col rounded-[8px] bg-[#ffffff] justify-start items-center py-[14px] px-[20px]",
                    div {
                        class: "w-full flex justify-start items-center text-[16px] font-bold cursor-pointer",
                        onclick: move |_| {
                            clicked2.set(!(*clicked2)());
                            clicked1.set(false);
                        },
                        div { class: "w-full flex flex-row justify-between items-center",
                            span { "{tr.public_opinion_committee_title}" }
                            if (*clicked2)() {
                                TriangleUp {}
                            } else {
                                TriangleDown {}
                            }
                        }
                    }
                    if (*clicked2)() {
                        // 운영위원회
                        //line
                        hr { class: "w-full h-[1px] mt-[12px] mb-[12px] border-[#eee]" }
                        //Title
                        div { class: "w-full flex flex-row justify-start items-center font-bold text-[18px]",
                            span { "운영 위원회" }
                        }
                        //user information
                        div { class: "w-full mt-[20px] flex flex-row justify-start gap-[40px] mb-[10px]",
                            div { class: "min-w-[180px] w-[40px] flex flex-row justify-start gap-[8px]",
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
                            div { class: "min-w-[180px] w-[40px] flex flex-row justify-start gap-[8px]",
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
                            div { class: "min-w-[180px] w-[40px] flex flex-row justify-start gap-[8px]",
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
                            div { class: "min-w-[180px] w-[40px] flex flex-row justify-start gap-[8px]",
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
                            div { class: "min-w-[180px] w-[40px] flex flex-row justify-start gap-[8px]",
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
                        //검증 위원회
                        //line
                        hr { class: "w-full h-[1px] mt-[12px] mb-[12px] border-[#eee]" }
                        //Title
                        div { class: "w-full flex flex-row justify-start items-center font-bold text-[18px]",
                            span { "검증 위원회" }
                        }
                        //user information
                        div { class: "w-full mt-[20px] flex flex-row justify-start gap-[40px] mb-[10px]",
                            div { class: "min-w-[180px] w-[40px] flex flex-row justify-start gap-[8px]",
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
                            div { class: "min-w-[180px] w-[40px] flex flex-row justify-start gap-[8px]",
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
                            div { class: "min-w-[180px] w-[40px] flex flex-row justify-start gap-[8px]",
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
                            div { class: "min-w-[180px] w-[40px] flex flex-row justify-start gap-[8px]",
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
                            div { class: "min-w-[180px] w-[40px] flex flex-row justify-start gap-[8px]",
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
                        //자문 위원회
                        //line
                        hr { class: "w-full h-[1px] mt-[12px] mb-[12px] border-[#eee]" }
                        //Title
                        div { class: "w-full flex flex-row justify-start items-center font-bold text-[18px]",
                            span { "자문 위원회" }
                        }
                        //user information
                        div { class: "w-full mt-[20px] flex flex-row justify-start gap-[40px] mb-[10px]",
                            div { class: "min-w-[180px] w-[40px] flex flex-row justify-start gap-[8px]",
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
                            div { class: "min-w-[180px] w-[40px] flex flex-row justify-start gap-[8px]",
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
                            div { class: "min-w-[180px] w-[40px] flex flex-row justify-start gap-[8px]",
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
                            div { class: "min-w-[180px] w-[40px] flex flex-row justify-start gap-[8px]",
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
                            div { class: "min-w-[180px] w-[40px] flex flex-row justify-start gap-[8px]",
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
                //Related Data
                div { class: "w-full flex flex-col rounded-[8px] mb-[40px] bg-[#ffffff] justify-start items-center py-[14px] px-[20px]",
                    // title and button
                    div { class: "w-full flex justify-start items-center gap-[13px]",
                        div { class: "w-[180px] flex flex-row items-center text-[16px] font-bold",
                            span { "{tr.related_materials_title}" }
                        }
                        //file
                        div { class: "min-w-[195px] min-h-[26px] flex flex- row justify-center items-center rounded-[100px] bg-[#7C8292] gap-[4px] px-[12px] py-[4px]",
                            // TODO: should be change DownloadIcon
                            DownloadIcon {}
                            div { class: "text-[14px] text-white",
                                "지역사회 교통 개선 프로젝트"
                            }
                        }
                    }
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
    BasicInfoTranslate;

    main_title: {
        ko: "소개글",
        en: "Introduction"
    }

    public_opinion_committee_title: {
        ko: "공론 위원회",
        en: "Public opinion committee"
    }

    related_materials_title: {
        ko: "관련 자료",
        en: "Related materials"
    }
}
