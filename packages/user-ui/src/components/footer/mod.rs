use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

mod i18n;
use crate::{components::icons, routes::Route};
use i18n::Translate;

#[component]
pub fn Footer(lang: Language) -> Element {
    let translates: Translate = translate(&lang);

    rsx! {
        footer { class: "flex gap-30 py-10 justify-center items-center text-sm font-semibold text-white/50 bg-[#1f1d2c]",
            div { "© 2025 Biyard. All Rights Reserved." }
            div { class: "font-extrabold text-base flex gap-1",
                icons::Logo { class: "fill-white/50" }
                "VOICE KOREA"
            }
            //TODO: Add more menus
            div { class: "flex gap-5",
                Link {
                    //TODO: Change Target
                    to: Route::MainPage {
                        lang: lang.clone(),
                    },
                    "{translates.policy}"
                }
                Link {
                    //TODO: Change Target
                    to: Route::MainPage {
                        lang: lang.clone(),
                    },
                    "{translates.terms}"
                }
            }
        }
    }
}
