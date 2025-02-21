use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

mod i18n;
use crate::{
    components::icons::{self, upload::Upload},
    routes::Route,
};
use i18n::FooterTranslate;

#[component]
pub fn Footer(lang: Language) -> Element {
    let tr: FooterTranslate = translate(&lang);
    let survey_design = asset!("/public/images/survey_design.png");
    let survey_participation = asset!("/public/images/survey_participation.png");

    rsx! {
        footer { class: "flex flex-col w-full justify-center items-center  bg-[#1f1d2c]",
            div { class: "flex flex-col w-full justify-center items-center pt-[80px] pb-[135px] gap-[50px]",
                div { class: "flex flex-col w-full justify-center items-center gap-[10px]",
                    div { class: "font-bold text-[28px] leading-8 text-white", "{tr.guideline}" }
                    div { class: "font-normal text-[15px] text-center leading-[22px] text-white whitespace-pre-line",
                        "{tr.guideline_desc}"
                    }
                }

                div { class: "flex flex-row justify-center items-center gap-[20px]",
                    div { class: "flex flex-col gap-[20px] justify-center items-center",
                        img { src: "{survey_design}", width: 310, height: 200 }
                        div { class: "flex flex-row w-[220px] justify-center items-center gap-[5px] px-[16px] py-[12px] bg-transparent border border-white rounded-[12px]",
                            Upload { width: "24", height: "24", fill: "none" }
                            button {
                                class: "font-semibold text-white text-[16px] leading-6 cursor-pointer",
                                //TODO: Go to public opinion survey participation guide
                                onclick: move |_| {},
                                "{tr.public_opinion_participation_guide}"
                            }
                        }
                    }
                    div { class: "flex flex-col gap-[20px] justify-center items-center",
                        img {
                            src: "{survey_participation}",
                            width: 310,
                            height: 200,
                        }
                        div { class: "flex flex-row w-[250px] justify-center items-center gap-[5px] px-[16px] py-[12px] bg-transparent border border-white rounded-[12px]",
                            Upload { width: "24", height: "24", fill: "none" }
                            button {
                                class: "font-semibold text-white text-[16px] leading-6 cursor-pointer",
                                //TODO: Go to the Public Opinion Survey Design Guide
                                onclick: move |_| {},
                                "{tr.public_opinion_survey_design_console_guide}"
                            }
                        }
                    }
                }
            }
            div { class: "flex flex-row w-full h-[1px] bg-white opacity-5" }
            div { class: "flex flex-row w-full gap-30 py-10 justify-center items-center text-sm font-semibold text-white/50",
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
