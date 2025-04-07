#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::routes::Route;

use dioxus_translate::Language;

#[derive(PartialEq, Props, Clone)]
pub struct HeaderProps {
    logout: String,
    lang: Language,
}

#[component]
pub fn Header(props: HeaderProps) -> Element {
    let logo_path = asset!("/public/images/logo.png");
    let navigator = use_navigator();
    rsx! {
        div { class: "flex flex-row justify-between items-center w-full min-h-[70px] max-h-[70px]",
            div {
                class: "flex min-h-[70px] justify-center items-center w-[250px]",
                onclick: move |_| {
                    navigator
                        .push(Route::SurveyPage {
                            lang: props.lang,
                        });
                },
                div { class: "mr-[7px]",
                    img { src: logo_path, width: 42, height: 42 }
                }
                div { class: "text-[24px] font-bold text-[#2168C3]", "VOICE KOREA" }
            }
            div { class: "flex flex-row w-[105px] h-[45px] rounded-[8px] cursor-pointer hover:bg-[#9a9a9a] bg-[#b0b0b0] mr-[45px]",
                Link {
                    class: "flex flex-row justify-center items-center text-[16px] font-bold text-white",
                    to: Route::LoginPage {
                        lang: props.lang.clone(),
                    },
                    "{props.logout}"
                }
            }
        }
    }
}
