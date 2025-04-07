use bdk::prelude::*;

use crate::components::icons::ArrowLeft;

use super::*;
use controller::*;
use i18n::*;

// TODO: implement discussion
#[component]
pub fn DeliberationDiscussionSettingPage(lang: Language) -> Element {
    let _ctrl = Controller::new(lang)?;
    let tr: DiscussionTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",

            div { class: "text-header-gray font-medium text-sm mb-10",
                "{tr.organization_management} / {tr.deliberation_management} / {tr.start_deliberation}"
            }
            div { class: "flex flex-row w-full justify-start items-center mb-25 gap-10",
                div { onclick: move |_| {},
                    ArrowLeft { width: "24", height: "24", color: "#3a3a3a" }
                }
                div { class: "text-header-black font-semibold text-[28px] mr-20", "{tr.discussion}" }
            }

            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "font-medium text-base text-text-black mb-10", "{tr.post_setting}" }
                div { class: "flex flex-row w-full justify-end items-end mt-40 mb-50",
                    div {
                        class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                        onclick: move |_| {},
                        "{tr.backward}"
                    }
                    div {
                        class: "flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                        onclick: move |_| {},
                        "{tr.temporary_save}"
                    }
                    div {
                        class: "cursor-pointer flex flex-row px-20 py-14 rounded-sm justify-center items-center bg-hover font-semibold text-base text-white",
                        onclick: move |_| {},
                        "{tr.next}"
                    }
                }
            }
        }
    }
}
