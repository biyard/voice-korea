use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

use crate::pages::project::i18n::ProjectTranslate;

#[component]
pub fn NotCompleteSurveyModal(
    lang: Language,
    description: String,
    onclose: EventHandler<MouseEvent>,
    onsave: EventHandler<MouseEvent>,
) -> Element {
    let tr: ProjectTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col min-w-[600px] max-[600px]:min-w-[350px] justify-start items-start gap-[40px]",
            div { class: "font-medium text-[14px] text-[#222222] whitespace-pre-line",
                "{description}"
            }
            div { class: "flex flex-row w-full justify-start items-center gap-[20px]",
                div {
                    class: "cursor-pointer flex flex-row bg-[#8095EA] rounded-[8px] px-[14px] py-[8px] font-semibold text-white text-[16px]",
                    onclick: move |e: Event<MouseData>| {
                        onclose.call(e);
                    },
                    "{tr.continue_answer}"
                }
                div {
                    class: "cursor-pointer flex flex-row bg-white px-[14px] py-[8px] font-semibold text-[#222222] text-[16px]",
                    onclick: move |e: Event<MouseData>| {
                        onsave.call(e);
                    },
                    "{tr.save_and_exit}"
                }
            }
        }
    }
}
