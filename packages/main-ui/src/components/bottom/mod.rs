#![allow(non_snake_case)]
mod i18n;

use dioxus::prelude::*;

use dioxus_translate::translate;
use dioxus_translate::Language;
use i18n::BottomTranslate;

#[derive(PartialEq, Props, Clone)]
pub struct BottomProps {
    lang: Language,
}

#[component]
pub fn Bottom(props: BottomProps) -> Element {
    let translates: BottomTranslate = translate(&props.lang);

    let address = translates.address;
    let company = translates.company;
    let company_address = translates.company_address;
    let ceo = translates.ceo;
    let register_address = translates.register_address;
    let copyright = translates.copyright;

    let bottom_text_style = "text-[14px] font-normal text-white";
    rsx! {
        div { class: "flex flex-row w-full items-center justify-start min-h-135 bg-[#2168C3] max-[500px]:!flex-col",
            div { class: "flex flex-col min-w-200 justify-center items-center gap-[15px] max-[500px]:!flex-row max-[500px]:!gap-10 max-[500px]:!py-10",
                img {
                    class: "flex flex-col",
                    src: asset!("/public/images/logo-white.png"),
                    width: 40,
                    height: 40,
                    alt: "Voice Korea Logo",
                }
                div { class: "flex flex-row text-[16px] font-bold text-white max-[500px]:!justify-center items-center",
                    "VOICE KOREA"
                }
            }

            div { class: "flex flex-col w-full h-full",
                div { class: "flex flex-col w-full h-full px-[10px] py-[25px]",
                    div { class: "{bottom_text_style} mb-[5px]", "{address}" }
                    div { class: "flex flex-row w-full justify-start items-start mb-[5px] max-[500px]:!flex-col",
                        div { class: "{bottom_text_style} pr-[20px]", "{company}" }
                        div { class: "{bottom_text_style} pr-[20px]", "{company_address}" }
                        div { class: "{bottom_text_style} pr-[20px]", "{ceo}" }
                        div { class: "{bottom_text_style}", "{register_address}" }
                    }

                    div { class: "text-[12px] font-normal text-white", "{copyright}" }
                }
            }
        }
    }
}
