use by_components::icons as by_components_icon;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};

mod i18n;
use crate::{components::icons, routes::Route, service::popup_service::PopupService};
use i18n::Translate;

#[component]
pub fn SignupPopup(lang: Language) -> Element {
    rsx! {
        div { class: "flex flex-col min-w-[420px]",
            div { class: "flex flex-row w-full bg-[#8095EA] rounded-[8px] p-[8px] gap-[15px]",
                div { class: "flex flex-row w-[62px] h-[62px] " }
            }
        }
    }
}

#[component]
pub fn Header(lang: Language) -> Element {
    let translates: Translate = translate(&lang);
    let mut popup_service: PopupService = use_context();

    let onclick = move |_| {
        tracing::debug!("signup button clicked");
        popup_service
            .open(rsx! {
                SignupPopup { lang: lang.clone() }
            })
            .with_id("signup")
            .with_title("로그인");
    };

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
                div { onclick, "{translates.login}" }
                div { class: "flex flex-row w-[105px] h-[30px] justify-center items-center rounded-lg px-[5px] py-[10px] bg-white border border-[#35343f]",
                    "{translates.public_opinion_design}"
                }
            }
        }
    }
}
