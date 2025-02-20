use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::v2::PublicOpinionProjectSummary;

use crate::pages::main::i18n::{MainBannerTranslate, OpinionFeatureTranslate};

use super::controller;
use super::i18n::Translate;

#[component]
pub fn MainPage(lang: Language) -> Element {
    let ctrl = controller::Controller::init(lang.clone())?;
    let _translates: Translate = translate(&lang);

    let public_opinions = ctrl.get_public_opinions();

    rsx! {
        div { class: "flex flex-col w-full justify-center items-center gap-[50px]",
            MainBanner { lang }

            OpinionSection { lang, public_opinions }
        }
    }
}

#[component]
pub fn OpinionSection(
    lang: Language,
    public_opinions: Vec<PublicOpinionProjectSummary>,
) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-[120px]",
            OpinionFeature { lang }
            OpinionProject { lang, public_opinions }
        }
    }
}

#[component]
pub fn OpinionProject(
    lang: Language,
    public_opinions: Vec<PublicOpinionProjectSummary>,
) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-[30px]",
            div { class: "flex flex-col w-full justify-start items-start gap-[10px]",
                div { class: "font-bold text-[28px] text-[#555462] leading-[32px]",
                    "프로젝트"
                }
                div { class: "font-normal text-[15px] text-[#555462] leading-[22px]",
                    "여러분의 의견이 정책에 반영될 수 있도록 진행된 공론조사 프로젝트 목록입니다. 함께 살펴보고, 어떤 주제들이 논의되었는지 확인해 보세요."
                }
            }
        }
    }
}

#[component]
pub fn ProjectBox(project: PublicOpinionProjectSummary) -> Element {
    let project_url = asset!("/public/images/project.png").to_string();
    rsx! {
        div { style: "background-image: url('{project_url}'); background-size: cover; height: 300px; width: ;" }
    }
}

#[component]
pub fn OpinionFeature(lang: Language) -> Element {
    let decentralized = asset!("/public/images/decentralized.png");
    let shield = asset!("public/images/shield.png");
    let sound = asset!("public/images/sound.png");

    let tr: OpinionFeatureTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-center items-center gap-[32px]",
            div { class: "font-bold text-[28px] text-[#555462] leading-[32px]", "{tr.title}" }
            div { class: "flex flex-row w-full justify-center items-center gap-[20px]",
                FeatureBox {
                    title: tr.sub_title_1,
                    description: tr.sub_description_1,
                    asset: decentralized,
                }
                FeatureBox {
                    title: tr.sub_title_2,
                    description: tr.sub_description_2,
                    asset: shield,
                }
                FeatureBox {
                    title: tr.sub_title_3,
                    description: tr.sub_description_3,
                    asset: sound,
                }
            }
            div { class: "font-semibold text-[15px] text-[#555462] leading-[22px] text-center whitespace-pre-line",
                "{tr.description}"
            }
        }
    }
}

#[component]
pub fn FeatureBox(title: String, description: String, asset: Asset) -> Element {
    rsx! {
        div {
            class: "flex flex-col w-[310px] justify-start items-start px-[24px] py-[34px] rounded-xl gap-[20px]",
            style: "box-shadow: 0px 8px 20px rgba(148, 128, 214, 0.5);",
            div { class: "font-bold text-[18px] text-[#222222]", "{title}" }
            div { class: "font-normal text-[15px] text-[#555462]", "{description}" }
            div { class: "flex flex-row w-full justify-end items-end",
                img { src: asset, width: 48, height: 48 }
            }
        }
    }
}

#[component]
pub fn MainBanner(lang: Language) -> Element {
    let background_url = asset!("/public/images/main_image.jpeg").to_string();
    let tr: MainBannerTranslate = translate(&lang);
    rsx! {
        div { class: "relative flex flex-col w-full h-[320px] justify-center items-start rounded-2xl p-[65px] gap-[10px] overflow-hidden",
            div { class: "absolute inset-0 bg-[url('{background_url}')] bg-cover bg-center opacity-80 rounded-2xl" }
            div { class: "relative font-bold text-[40px] leading-[58px] text-white",
                "{tr.title}"
            }
            div { class: "relative font-medium text-[16px] leading-6 text-white whitespace-pre-line mb-[10px]",
                "{tr.description}"
            }
            //TODO:Go to public opinion survey page
            button {
                class: "relative flex flex-row px-[16px] py-[12px] bg-[#5b373b] border border-white rounded-[12px] font-semibold text-[16px] text-white cursor-pointer",
                onclick: move |_| {},
                "{tr.button}"
            }
        }
    }
}
