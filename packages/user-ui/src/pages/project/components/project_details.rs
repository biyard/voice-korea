use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::Tab;

use crate::{
    components::icons::{
        download::DownloadIcon,
        triangle::{TriangleDown, TriangleUp},
    },
    pages::project::components::i18n::{DeliberationTranslate, DetailsTranslate},
};

#[component]
pub fn ProjectDetails(lang: Language, active_tab: Signal<Tab>) -> Element {
    rsx! {
        div { class: "w-full bg-[#F7F7F7]",
            match *active_tab.read() {
                Tab::Details => rsx! {
                    Details { lang }
                },
                Tab::SampleSurvey => rsx! {
                    SampleSurvey {}
                },
                Tab::Deliberation => rsx! {
                    Deliberation { lang }
                },
                Tab::Discussion => rsx! {
                    Discussion {}
                },
                Tab::Vote => rsx! {
                    Vote {}
                },
                Tab::FinalRecommendation => rsx! {
                    FinalRecommendation {}
                },
            }
        }
    }
}

#[component]
pub fn Details(lang: Language) -> Element {
    let mut clicked1 = use_signal(|| false);
    let mut clicked2 = use_signal(|| false);
    let tab_title: &str = Tab::Details.translate(&lang);
    let tr: DetailsTranslate = translate(&lang);

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

#[component]
pub fn SampleSurvey() -> Element {
    rsx! {
        div { "여기에 표본 조사 페이지 내용을 넣으세요." }
    }
}

#[component]
pub fn Deliberation(lang: Language) -> Element {
    let tab_title: &str = Tab::Deliberation.translate(&lang);
    let tr: DeliberationTranslate = translate(&lang);
    let mut clicked1 = use_signal(|| false);
    let mut clicked2 = use_signal(|| false);
    let watched_seconds = use_signal(|| 120);
    let total_seconds = use_signal(|| 300);

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
                            // TODO: have to import image data
                            img { class: "w-[240px] h-[150px] rounded-[8px] border-none",
                                "image data"
                            }
                            div { class: "w-full h-[150px] flex flex-col justify-between",
                                p { class: "w-full flex flex-col justify-start",
                                    p { class: "text-[14px]", "e-Book" }
                                    // TODO: have to connect data
                                    p { class: "font-bold text-[18px]",
                                        "지역사회 교통 개선 기초 수업"
                                    }
                                    div { class: "max-w-[500px] flex flex-row gap-[20px]",
                                        div { class: "w-full flex items-center",
                                            ProgressBar {
                                                watched_seconds: watched_seconds(),
                                                total_seconds: total_seconds(),
                                            }
                                        }
                                        div { class: "w-full text-[14px]", "87% (21/30 페이지)" }
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
            //Related Data
            div { class: "w-full flex flex-col rounded-[8px] mb-[40px] bg-[#ffffff] justify-start items-center py-[14px] px-[20px]",
                // title and button
                div { class: "w-full flex justify-start items-center gap-[13px]",
                    div { class: "w-[180px] flex flex-row items-center text-[16px] font-bold",
                        span { "{tr.deliberation_materials_title}" }
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

#[component]
pub fn Discussion() -> Element {
    rsx! {
        div { "여기에 토론 페이지 내용을 넣으세요." }
    }
}

#[component]
pub fn Vote() -> Element {
    rsx! {
        div { "여기에 투표 페이지 내용을 넣으세요." }
    }
}
#[component]
pub fn FinalRecommendation() -> Element {
    rsx! {
        div { "여기에 최종 권고안 페이지 내용을 넣으세요." }
    }
}

#[component]
pub fn ProgressBar(watched_seconds: u32, total_seconds: u32) -> Element {
    let progress = if total_seconds == 0 {
        0
    } else {
        (watched_seconds * 100 / total_seconds).min(100)
    };
    //TODO: have to connect video watching time data

    rsx!(
        div { class: "w-full flex justify-start items-center bg-gray-300 h-1 rounded-[100px]",
            div {
                class: "bg-green-500 h-2 transition-all duration-500 rounded-[100px]",
                style: format!("width: {}%;", progress),
            }
        }
    )
}
