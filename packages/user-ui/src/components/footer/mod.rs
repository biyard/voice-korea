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
        footer { class: "flex flex-col w-full justify-center items-center  bg-footer",
            div { class: "flex flex-col w-full justify-center items-center pt-80 pb-135 gap-50",
                div { class: "flex flex-col w-full justify-center items-center gap-10",
                    div { class: "font-bold text-[28px] leading-32 text-white", "{tr.guideline}" }
                    div { class: "font-normal text-[15px] text-center leading-22 text-white whitespace-pre-line",
                        "{tr.guideline_desc}"
                    }
                }

                div { class: "flex max-[750px]:flex-col flex-row justify-center items-center gap-20",
                    div { class: "flex flex-col gap-20 justify-center items-center",
                        img { src: "{survey_design}", width: 310, height: 200 }
                        div { class: "flex flex-row w-fit justify-center items-center gap-5 px-16 py-12 bg-transparent border border-white rounded-xl",
                            div { class: "w-24 h-24",
                                Upload { width: "24", height: "24", fill: "none" }
                            }
                            button {
                                class: "font-semibold text-white text-base leading-24 cursor-pointer",
                                //TODO: Go to public opinion survey participation guide
                                onclick: move |_| {},
                                "{tr.public_opinion_participation_guide}"
                            }
                        }
                    }
                    div { class: "flex flex-col gap-20 justify-center items-center",
                        img {
                            src: "{survey_participation}",
                            width: 310,
                            height: 200,
                        }
                        div { class: "flex flex-row w-fit justify-center items-center gap-5 px-16 py-12 bg-transparent border border-white rounded-xl",
                            div { class: "w-24 h-24",
                                Upload { width: "24", height: "24", fill: "none" }
                            }
                            button {
                                class: "font-semibold text-white text-base leading-24 cursor-pointer",
                                //TODO: Go to the Public Opinion Survey Design Guide
                                onclick: move |_| {},
                                "{tr.public_opinion_survey_design_console_guide}"
                            }
                        }
                    }
                }
            }
            div { class: "flex flex-row w-full h-1 bg-white opacity-5" }
            div { class: "flex flex-row w-full max-[600px]:gap-4 gap-120 py-40 justify-center items-center text-sm font-semibold text-white/50",
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
