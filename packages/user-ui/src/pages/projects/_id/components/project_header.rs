use chrono::{NaiveDateTime, NaiveTime, Utc};
use dioxus::prelude::*;
use dioxus_translate::{translate, Language, Translate};
use models::{deliberation_project::DeliberationProject, Tab};

use crate::{
    components::icons::{
        adopted::Adopted, in_progress::InProgress, right_arrow::RightArrow, waiting::Waiting,
    },
    pages::projects::_id::components::i18n::HeaderTranslate,
    utils::time::formatted_timestamp,
};

#[derive(Debug, Translate, PartialEq, Eq, Default)]
pub enum ProjectStatus {
    #[default]
    #[translate(ko = "준비", en = "Ready")]
    Ready,
    #[translate(ko = "진행중", en = "InProgress")]
    InProgress,
    #[translate(ko = "완료", en = "Finish")]
    Finish,
}

#[component]
pub fn ProjectHeader(
    lang: Language,
    deliberation: DeliberationProject,
    active_tab: Signal<Tab>,
) -> Element {
    let tr: HeaderTranslate = translate(&lang);
    let mut set_active_tab = move |value: Tab| active_tab.set(value);
    let active_tab_value = active_tab.read();

    let started_at = formatted_timestamp(deliberation.started_at);
    let ended_at = formatted_timestamp(deliberation.ended_at);

    rsx! {
        div { class: "max-w-[1300px] h-[300px] mb-[40px] flex flex-row w-full justify-center items-center gap-[40px]",
            // TODO: connect to data and UI
            //data section
            div { class: "w-full max-w-[720px] h-[260px] flex flex-col justify-center",
                div { class: "flex flex-col justify-start",
                    div { class: "w-full h-[24px] flex justify-start items-center font-semibold text-[18px] gap-[8px]",
                        match deliberation_status(deliberation.started_at, deliberation.ended_at) {
                            ProjectStatus::Ready => rsx! {
                                Waiting {}
                            },
                            ProjectStatus::InProgress => rsx! {
                                InProgress {}
                            },
                            ProjectStatus::Finish => rsx! {
                                Adopted {}
                            },
                        }
                        div { "{started_at} ~ {ended_at}" }
                    }
                    div { class: "w-full flex justify-start items-center font-semibold text-[32px] leading-[60px]",
                        "{deliberation.title}"
                    }
                    div { class: "w-full flex justify-start items-center font-md text-[14px] gap-[4px]",
                        div { class: "py-[2px] px-[12px] leading-[22px] flex justify-center items-center border border-[#222] rounded-[100px]",
                            div { {deliberation.project_area.translate(&lang)} }
                        }
                    }
                    div { class: "w-full my-[20px] flex flex-row justify-start items-center gap-[8px]",
                        img {
                            class: "w-[50px] h-[50px]",
                            src: asset!("/public/images/organization.png"),
                        }
                        div {
                            div { class: "flex justify-start items-center font-normal text-[15px]",
                                "{tr.organization}"
                            }
                        }
                    }
                    div { class: "flex flex-row justify-start items-center gap-[60px]",
                        div { class: "w-hug h-[59px] flex flex-col justify-center items-center",
                            div { class: "justify-center items-center font-semibold text-[24px]",
                                "{deliberation.participants}"
                            }
                            div { class: "justify-center items-center font-md text-[14px]",
                                "{tr.participant}"
                            }
                        }
                        div { class: "w-hug h-[59px] flex flex-col justify-center items-center",
                            div { class: "justify-center items-center font-semibold text-[24px]",
                                "{deliberation.votes}"
                            }
                            div { class: "justify-center items-center font-md text-[14px]",
                                "{tr.vote}"
                            }
                        }
                    }
                }
            }
            //img section
            div { class: "flex justify-center items-center",
                img {
                    class: "w-[540px] h-[300px] rounded-[12px]",
                    src: asset!("/public/images/section_image.png"),
                    alt: "Header Section Image",
                }
            }
        }
        //menu
        div { class: "flex flex-col w-full justify-center items-center bg-[#f7f7f7]",
            div { class: "flex flex-col max-w-[1300px] w-full",
                // Tab menu
                div { class: "w-full h-[42px] flex flex-row justify-between items-center",
                    for tab in Tab::all() {
                        div { class: "flex flex-col items-center w-[160px]",
                            div {
                                class: "w-[160px] h-[30px] flex justify-center items-center font-md text-[15px] cursor-pointer",
                                class: if *active_tab_value == tab { " font-semibold" } else { "text-[#222]" },
                                onclick: move |_| set_active_tab(tab),
                                p { {tab.translate(&lang)} }
                            }
                            div { class: if *active_tab_value == tab { "w-full h-[2px] bg-[#8095EA]" } else { "w-full h-[2px] bg-transparent" } }
                        }
                        if tab != Tab::FinalDraft {
                            RightArrow { color: "#B4B4B4" }
                        }
                    }
                }
                // line
                div { class: "w-full h-[1px] bg-[#eee]" }
            }
        }
    }
}

pub fn deliberation_status(started_at: i64, ended_at: i64) -> ProjectStatus {
    let today = Utc::now().date_naive();
    let naive_time = NaiveTime::from_hms_opt(0, 0, 0).expect("Invalid time");
    let timestamp = NaiveDateTime::new(today, naive_time).and_utc().timestamp();

    if timestamp < started_at {
        ProjectStatus::Ready
    } else if timestamp > ended_at {
        ProjectStatus::Finish
    } else {
        ProjectStatus::InProgress
    }
}
