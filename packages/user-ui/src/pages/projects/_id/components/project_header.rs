use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::{deliberation::Deliberation, Tab};

use crate::{
    components::icons::right_arrow::RightArrow,
    pages::projects::_id::components::i18n::HeaderTranslate,
};

#[component]
pub fn ProjectHeader(
    lang: Language,
    deliberation: Deliberation,
    active_tab: Signal<Tab>,
) -> Element {
    let tr: HeaderTranslate = translate(&lang);
    let mut set_active_tab = move |value: Tab| active_tab.set(value);
    let active_tab_value = active_tab.read();

    rsx! {
        div { class: "max-w-[1300px] h-[300px] mb-[40px] flex flex- row justify-center items-center gap-[40px]",
            // TODO: connect to data and UI
            //data section
            div { class: "w-full max-w-[720px] h-[260px] flex flex-col justify-center",
                div { class: "flex flex-col justify-start",
                    div { class: "w-full h-[24px] flex justify-start items-center font-semibold text-[18px] gap-[8px]",
                        img { class: "w-[24px] h-[24px]" }
                        // TODO: need to change time type
                        div { "{deliberation.created_at}" }
                    }
                    div { class: "w-full h-[60px] flex justify-start items-center font-semibold text-[32px] ",
                        "{deliberation.title}"
                    }
                    div { class: "w-full h-[27px] flex justify-start items-center font-md text-[14px] gap-[4px]",
                        div { class: "min-w-[49px] h-[27px] px-[12px] flex justify-center items-center border border-[#222] rounded-[100px]",
                            div { "{deliberation.project_area}" }
                        }
                    }
                    div { class: "w-full h-[50px] my-[20px] flex flex-row justify-start items-center gap-[8px]",
                        img { class: "w-[50px] h-[50px]" }
                        div {
                            div { class: "flex justify-start items-center font-semibold text-[18px]",
                                "{deliberation.id}"
                            }
                            div { class: "flex justify-start items-center font-md text-[14px]",
                                "{deliberation.org_id}"
                            }
                        }
                    }
                    div { class: "flex flex-row justify-start items-center gap-[60px]",
                        div { class: "w-hug h-[59px] flex flex-col justify-center items-center",
                            div { class: "justify-center items-center font-semibold text-[24px]",
                                "{deliberation.panels.len()}"
                            }
                            div { class: "justify-center items-center font-md text-[14px]",
                                "{tr.participant}"
                            }
                        }
                        div { class: "w-hug h-[59px] flex flex-col justify-center items-center",
                            div { class: "justify-center items-center font-semibold text-[24px]",
                                "{deliberation.members.len()}"
                            }
                            div { class: "justify-center items-center font-md text-[14px]",
                                "{tr.public_opinion_committee}"
                            }
                        }
                        div { class: "w-hug h-[59px] flex flex-col justify-center items-center",
                            div { class: "justify-center items-center font-semibold text-[24px]",
                                "{deliberation.response_count}"
                            }
                            div { class: "justify-center items-center font-md text-[14px]",
                                "{tr.vote}"
                            }
                        }
                        div { class: "w-hug h-[59px] flex flex-col justify-center items-center",
                            div { class: "justify-center items-center font-semibold text-[24px]",
                                "data"
                            }
                            div { class: "justify-center items-center font-md text-[14px]",
                                "{tr.yes}"
                            }
                        }
                        div { class: "w-hug h-[59px] flex flex-col justify-center items-center",
                            div { class: "justify-center items-center font-semibold text-[24px]",
                                "data"
                            }
                            div { class: "justify-center items-center font-md text-[14px]",
                                "{tr.no}"
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
        //menu
        div { class: " w-full h-hug flex flex-col",
            // Tab menu
            div { class: " bg-[#F7F7F7] w-full h-[42px] flex flex-row justify-between items-center",
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
