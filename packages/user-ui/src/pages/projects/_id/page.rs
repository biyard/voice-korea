use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::Language;

use super::{components::sample::Sample, controller};

use crate::{
    components::icons::{
        download::DownloadIcon,
        pencil::Pencil,
        person::Person,
        right_arrow::RightArrow,
        triangle::{TriangleDown, TriangleUp},
    },
    pages::projects::_id::components::basic_info::BasicInfo,
};

#[component]
pub fn ProjectPage(lang: Language, project_id: ReadOnlySignal<i64>) -> Element {
    let ctrl = controller::Controller::init(lang, project_id)?;
    let deliberation = ctrl.get_deliberation();
    tracing::debug!("deliberation: {:?}", deliberation);
    rsx! {
        div {
            ProjectProfile { lang }
            ProjectDetailsMenu { lang }

            // TODO: Specific view for tabs
            // Use sliding tabs when changing selected tab
            BasicInfo { lang, project_id }

        // TODO: Comments
        }
    }
}

#[component]
pub fn ProjectProfile(lang: Language) -> Element {
    rsx! {
        div { class: "w-[1300px] h-[300px] mb-[40px] flex flex- row justify-center items-center gap-[40px]",
            // TODO: connect to data and UI
            //data section
            div { class: "w-[720px] h-[260px] flex flex-col justify-center",
                div { class: "flex flex-col justify-start",
                    div { class: "w-full h-[24px] flex justify-start items-center font-semibold text-[18px] gap-[8px]",
                        img { class: "w-[24px] h-[24px]" }
                        div { "date" }
                    }
                    div { class: "w-full h-[60px] flex justify-start items-center font-semibold text-[32px] ",
                        "title"
                    }
                    div { class: "w-full h-[27px] flex justify-start items-center font-md text-[14px] gap-[4px]",
                        div { class: "min-w-[49px] h-[27px] px-[12px] flex justify-center items-center border border-[#222] rounded-[100px]",
                            "tag1"
                        }
                        div { class: "min-w-[49px] h-[27px] px-[12px] flex justify-center items-center border border-[#222] rounded-[100px]",
                            "tag2"
                        }
                    }
                    div { class: "w-full h-[50px] my-[20px] flex flex-row justify-start items-center gap-[8px]",
                        img { class: "w-[50px] h-[50px]" }
                        div {
                            div { class: "flex justify-start items-center font-semibold text-[18px]",
                                "owner"
                            }
                            div { class: "flex justify-start items-center font-md text-[14px]",
                                "sub owner"
                            }
                        }
                    }
                    div { class: "flex flex-row justify-start items-center gap-[60px]",
                        div { class: "w-hug h-[59px] flex flex-col justify-center items-center",
                            div { class: "justify-center items-center font-semibold text-[24px]",
                                "숫자"
                            }
                            div { class: "justify-center items-center font-md text-[14px]",
                                "참여자"
                            }
                        }
                        div { class: "w-hug h-[59px] flex flex-col justify-center items-center",
                            div { class: "justify-center items-center font-semibold text-[24px]",
                                "숫자"
                            }
                            div { class: "justify-center items-center font-md text-[14px]",
                                "공론위원회"
                            }
                        }
                        div { class: "w-hug h-[59px] flex flex-col justify-center items-center",
                            div { class: "justify-center items-center font-semibold text-[24px]",
                                "숫자"
                            }
                            div { class: "justify-center items-center font-md text-[14px]",
                                "투표"
                            }
                        }
                        div { class: "w-hug h-[59px] flex flex-col justify-center items-center",
                            div { class: "justify-center items-center font-semibold text-[24px]",
                                "숫자"
                            }
                            div { class: "justify-center items-center font-md text-[14px]",
                                "찬성"
                            }
                        }
                        div { class: "w-hug h-[59px] flex flex-col justify-center items-center",
                            div { class: "justify-center items-center font-semibold text-[24px]",
                                "숫자"
                            }
                            div { class: "justify-center items-center font-md text-[14px]",
                                "반대"
                            }
                        }
                    }
                }
            }

            //img section
            div { class: "flex justify-center items-center",
                img { class: "w-[540px] h-[300px] rounded-[12px]" }
            }
        }
    }
}

#[component]
pub fn ProjectDetailsMenu(lang: Language) -> Element {
    let mut active_tab = use_signal(|| "기본정보".to_string());
    let mut set_active_tab = move |value: &str| active_tab.set(value.to_string());
    let active_tab_value = active_tab.read();
    let mut clicked1 = use_signal(|| false);
    let mut clicked2 = use_signal(|| false);

    rsx! {
        div { class: "w-full h-hug flex flex-col bg-[#F7F7F7]",
            // Tap menu
            div { class: "w-full h-[42px] flex flex-row justify-between items-center",
                for tab in ["기본정보", "표본 조사", "숙의", "토론", "투표", "최종 권고안"] {
                    div { class: "flex flex-col items-center w-[160px]",
                        div {
                            class: "w-[160px] h-[30px] flex justify-center items-center font-md text-[15px] cursor-pointer",
                            class: if active_tab_value.as_str() == tab { " font-semibold" } else { "text-[#222]" },
                            onclick: move |_| set_active_tab(tab),
                            p { "{tab}" }
                        }
                        div { class: if active_tab_value.as_str() == tab { "w-full h-[2px] bg-[#8095EA]" } else { "w-full h-[2px] bg-transparent" } }
                    }
                    if tab != "최종 권고안" {
                        RightArrow { color: "#B4B4B4" }
                    }
                }
            }
            // line
            div { class: "w-full h-[1px] bg-[#eee]" }

            // Tap contents
            div { class: "w-full px-4 pt-[28px] pb-[40px]",
                match active_tab_value.as_str() {
                    "기본정보" => rsx! {
                        div { class: "w-full h-[32px] mb-[20px] flex flex-row justify-between items-center",
                            p { class: "font-semibold text-[20px]", "기본정보" }
                            div { class: "min-w-[160px] min-h-[32px] flex flex-row justify-center items-center gap-[80px]",
                                //date and write count
                                div { class: "flex flex-row justify-start items-center gap-[20px]",
                                    //date
                                    div { class: "text-[15px]", "2월 14일 2025년" }
                                    //write count
                                    div { class: "flex flex-row items-center gap-[4px]",
                                        span { "6" }
                                        Pencil {}
                                    }
                                }
                                //user count menu
                                div { class: "flex flex-row justify-center items-center gap-[20px]",
                                    div { class: "relative flex items-center",
                                        img { class: "w-[32px] h-[32px] bg-[#CFCFCF] rounded-full z-10" }
                                        img { class: "w-[32px] h-[32px] bg-[#8C8C8C] rounded-full -ml-2 z-20" }
                                        img { class: "w-[32px] h-[32px] bg-[#CFCFCF] rounded-full -ml-2 z-30" }
                                        img { class: "w-[32px] h-[32px] bg-[#8C8C8C] rounded-full -ml-2 z-40" }
                                    }
                                    //user count number and icon
                                    div { class: "flex flex-row items-center gap-[4px]",
                                        //count
                                        span { "4" }
                                        Person {}
                                    }
                                }
                            }
                        }
                        //Information section
                        div { class: "flex flex-col gap-[10px]",
                            //introduce
                            div { class: "w-full flex flex-col rounded-[8px] bg-[#ffffff] justify-start items-center py-[14px] px-[20px]",
                                // title and button
                                div {
                                    class: "w-full flex justify-start items-center text-[16px] font-bold cursor-pointer",
                                    onclick: move |_| clicked1.set(!clicked1()),
                                    div { class: "w-full flex flex-row justify-between items-center",
                                        span { "소개글" }
                                        if (*clicked1)() {
                                            TriangleUp {}
                                        } else {
                                            TriangleDown {}
                                        }
                                    }
                                }
                                // text section
                                if (*clicked1)() {
                                    div { class: "w-full mt-[15px]" }
                                    //line
                                    div { class: "w-full h-[1px] bg-[#eee]" }
                                    //title
                                    div { class: "w-full mt-[15px] mb-[20px] font-bold text-[18px]", "제목 구간입니다." }
                                    //description
                                    div { class: "w-full flex justify-start text-[15px]",
                                        "1. 공론조사의 목적 및 배경"
                                    }
                                }
                            }
                            //public opinion committee
                            // TODO: when this section open, other section have to closed
                            div { class: "w-full flex flex-col rounded-[8px] bg-[#ffffff] justify-start items-center py-[14px] px-[20px]",
                                // title and button
                                div {
                                    class: "w-full flex justify-start items-center text-[16px] font-bold cursor-pointer",
                                    onclick: move |_| clicked2.set(!clicked2()),
                                    div { class: "w-full flex flex-row justify-between items-center",
                                        span { "공론 위원회" }
                                        if (*clicked2)() {
                                            TriangleUp {}
                                        } else {
                                            TriangleDown {}
                                        }
                                    }
                                }
                                // text section
                                if (*clicked2)() {
                                    div { class: "w-full mt-[15px]" }
                                    //line
                                    div { class: "w-full h-[1px] bg-[#eee]" }
                                    //title
                                    div { class: "w-full mt-[15px] mb-[20px] font-bold text-[18px]", "제목 구간입니다." }
                                    //description
                                    div { class: "w-full flex justify-start text-[15px]",
                                        "1. 공론조사의 목적 및 배경"
                                    }
                                }
                            }
                            //Related Data
                            div { class: "w-full flex flex-col rounded-[8px] bg-[#ffffff] justify-start items-center py-[14px] px-[20px]",
                                // title and button
                                div { class: "w-full flex justify-start items-center gap-[13px]",
                                    div { class: "w-[180px] flex flex-row items-center text-[16px] font-bold",
                                        span { "관련 자료" }
                                    }
                                    //file
                                    div { class: "min-w-[195px] min-h-[26px] flex flex- row justify-center items-center rounded-[100px] bg-[#7C8292] gap-[4px] px-[12px] py-[4px]",
                                        // TODO: should be change DownloadIcon
                                        DownloadIcon {}
                                        div { class: "text-[14px] text-white", "지역사회 교통 개선 프로젝트" }
                                    }
                                }
                            }
                        }
                    },
                    "표본 조사" => rsx! {
                        Sample { lang }
                    },
                    "숙의" => rsx! {
                        div { "여기에 숙의 페이지 내용을 넣으세요." }
                    },
                    "토론" => rsx! {
                        div { "여기에 토론 페이지 내용을 넣으세요." }
                    },
                    "투표" => rsx! {
                        div { "여기에 투표 페이지 내용을 넣으세요." }
                    },
                    "최종 권고안" => rsx! {
                        div { "여기에 최종 권고안 페이지 내용을 넣으세요." }
                    },
                    _ => rsx! {
                        div { "선택된 탭이 없습니다." }
                    },
                }
            }
        }
    }
}
