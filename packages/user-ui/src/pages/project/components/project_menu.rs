use dioxus::prelude::*;
use dioxus_translate::Language;

use crate::components::icons::right_arrow::RightArrow;
use models::Tab;
#[component]
pub fn ProjectMenu(lang: Language) -> Element {
    let mut active_tab = use_signal(|| Tab::Details);
    let mut set_active_tab = move |value: Tab| active_tab.set(value);
    let active_tab_value = active_tab.read();

    rsx! {
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
                    if tab != Tab::FinalRecommendation {
                        RightArrow { color: "#B4B4B4" }
                    }
                }
            }
            // line
            div { class: "w-full h-[1px] bg-[#eee]" }
        }
    }
}
