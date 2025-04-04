use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

use crate::{
    components::icons::{self},
    routes::Route,
};

#[component]
pub fn MainFooter(lang: Language) -> Element {
    let tr: MainFooterTranslate = translate(&lang);

    rsx! {
        footer {
            id: "footer",
            class: "flex flex-col w-full justify-center items-center bg-footer",
            // div { class: "flex flex-row w-full h-1 bg-white opacity-5" }
            div { class: "flex flex-col w-full gap-5 tablet:gap-120 px-20 desktop:px-0 py-40 justify-center items-center text-sm font-semibold text-white/50 tablet:flex-row",
                div { "© 2025 Biyard. All Rights Reserved." }
                div { class: "font-extrabold text-base flex gap-4",
                    icons::Logo { class: "fill-white/50" }
                    "VOICE KOREA"
                }
                //TODO: Add more menus
                div { class: "flex gap-20",
                    Link {
                        //TODO: Change Target
                        to: Route::MainPage {
                            lang: lang.clone(),
                        },
                        "{tr.policy}"
                    }
                    Link {
                        //TODO: Change Target
                        to: Route::MainPage {
                            lang: lang.clone(),
                        },
                        "{tr.terms}"
                    }
                }
            }
        }
    }
}

translate! {
    MainFooterTranslate;

    policy: {
        ko: "개인정보 보호정책",
        en: "Privacy Policy"
    },

    terms: {
        ko: "이용 약관",
        en: "Terms of Use"
    },


}
