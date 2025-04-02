use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

use crate::{
    components::icons::{self, upload::Upload},
    routes::Route,
};

#[component]
pub fn MainFooter(lang: Language) -> Element {
    let tr: MainFooterTranslate = translate(&lang);
    let survey_design = asset!("/public/images/survey_design.png");
    let survey_participation = asset!("/public/images/survey_participation.png");

    rsx! {
        footer {
            id: "footer",
            class: "flex flex-col w-full justify-center items-center  bg-footer",
            div { class: "flex flex-col w-full justify-center items-center pt-80 pb-135 gap-50",
                div { class: "flex flex-col w-full justify-center items-center gap-10 max-[500px]:px-10",
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
            div { class: "flex flex-row w-full max-[600px]:gap-4 gap-120 py-40 justify-center items-center text-sm font-semibold text-white/50 max-[500px]:flex-col",
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

    guideline: {
        ko: "가이드라인",
        en: "Guideline"
    }

    guideline_desc: {
        ko: "공론조사 참여 및 설계 콘솔에 대한 자세한 가이드를 다운로드하실 수 있습니다. 해당 가이드를 통해 플랫폼 사용법과 공론 조사 설계 방법을 쉽게 이해할 수 있습니다.\n파일을 다운로드하여 공론조사와 설계 콘솔에 대한 중요한 정보를  확인하고, 참여 및 설계를 더욱 효과적으로 진행하세요.",
        en: "You can download detailed guides on participating in public opinion surveys and using the design console. Through this guide, you can easily understand how to use the platform and design public opinion surveys.\nDownload the file to check important information about the public opinion survey and design console, and participate and design more effectively."
    }

    public_opinion_participation_guide: {
        ko: "공론 조사 참여 가이드",
        en: "Public Opinion Participation Guide"
    }

    public_opinion_survey_design_console_guide: {
        ko: "공론 조사 설계 콘솔 가이드",
        en: "Public Opinion Survey Design Console Guide"
    }
}
