use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

mod i18n;
use crate::{components::icons, routes::Route};
use i18n::Translate;

#[component]
pub fn Header(lang: Language) -> Element {
    let translates: Translate = translate(&lang);

    rsx! {
        header { class: "flex justify-between my-6.5 h-[30px]",
            Link {
                class: "flex flex-row items-center justify-around gap-1 h-full",
                to: Route::MainPage {
                    lang: lang.clone(),
                },
                icons::Logo {}
                div { class: "font-extrabold text-base text-[#34333e]", "VOICE KOREA" }
            }
            //TODO: Add more menus
            div { class: "flex font-bold justify-center items-center text-[#35343f] text-[15px] leading-[18.75px] gap-[45px]",
                Link {
                    //TODO: Change Target
                    to: Route::MainPage {
                        lang: lang.clone(),
                    },
                    "{translates.service}"
                }
                Link {
                    //TODO: Change Target
                    to: Route::MainPage {
                        lang: lang.clone(),
                    },
                    "{translates.project}"
                }
                Link {
                    //TODO: Change Target
                    to: Route::MainPage {
                        lang: lang.clone(),
                    },
                    "{translates.organization}"
                }
                Link {
                    //TODO: Change Target
                    to: Route::MainPage {
                        lang: lang.clone(),
                    },
                    "{translates.plan}"
                }
                Link {
                    //TODO: Change Target
                    to: Route::MainPage {
                        lang: lang.clone(),
                    },
                    "{translates.contact}"
                }
                Link {
                    //TODO: Change Target
                    to: Route::MainPage {
                        lang: lang.clone(),
                    },
                    "{translates.guide}"
                }
                Link {
                    to: Route::UserLoginPage {
                        lang: lang.clone(),
                    },
                    "{translates.login}"
                }
                div { class: "flex flex-row w-[105px] h-[30px] justify-center items-center rounded-lg px-[5px] py-[10px] bg-white border border-[#35343f]",
                    "공론 조사 설계"
                }
            }
        }
    }
}
