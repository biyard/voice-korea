use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::v2::PublicOpinionProjectSummary;
use num_format::{Locale, ToFormattedString};

use crate::components::icons::user::User;
use crate::components::icons::vote::Vote;
use crate::pages::main::i18n::{
    MainBannerTranslate, MoreButtonTranslate, OpinionFeatureTranslate, OpinionProjectTranslate,
    ProjectBoxTranslate,
};

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
    let tr: OpinionProjectTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-[40px]",
            div { class: "flex flex-col gap-[30px]",
                div { class: "flex flex-col gap-[10px]",
                    div { class: "font-bold text-[28px] text-[#555462] leading-[32px]",
                        "{tr.project}"
                    }
                    div { class: "font-normal text-[15px] text-[#555462] leading-[22px]",
                        "{tr.project_description}"
                    }
                }

                div { class: "flex flex-wrap gap-[20px] justify-center items-center",
                    for project in public_opinions {
                        ProjectBox { lang, project }
                    }
                }
                div { class: "flex flex-row w-full justify-center items-center",
                    MoreButton {
                        lang,
                        onclick: move |_| {
                            tracing::debug!("more button clicked");
                        },
                    }
                }
            }
        }
    }
}

#[component]
pub fn MoreButton(lang: Language, onclick: EventHandler<MouseEvent>) -> Element {
    let tr: MoreButtonTranslate = translate(&lang);
    rsx! {
        div {
            class: "flex flex-row px-[20px] py-[12px] bg-[#8095ea] font-semibold text-[16px] text-white cursor-pointer rounded-[12px]",
            onclick: move |e: Event<MouseData>| {
                onclick.call(e);
            },
            "{tr.more}"
        }
    }
}

#[component]
pub fn ProjectBox(lang: Language, project: PublicOpinionProjectSummary) -> Element {
    let project_url = asset!("/public/images/project.png").to_string();
    let institution_badge_url = asset!("/public/images/institution_badge.png").to_string();
    let tr: ProjectBoxTranslate = translate(&lang);

    rsx! {
        div {
            class: "flex flex-col justify-end items-end rounded-[30px] shadow-[0px_8px_20px_rgba(148,176,214,0.25)]",
            style: "background-image: url('{project_url}'); background-size: cover; height: 450px; width: 400px;",
            div { class: "flex flex-col w-full justify-start items-start rounded-[20px] bg-white px-[16px] pt-[20px] pb-[12px]",
                div { class: "flex flex-col gap-[16px]",
                    div { class: "flex flex-col gap-[8px]",
                        div { class: "font-bold text-[18px] text-[#222222]", "{project.title}" }
                        div { class: "flex flex-col gap-[12px]",
                            div { class: "font-normal text-[#555462] text-[14px]",
                                "{project.description}"
                            }
                            div { class: "flex flex-col gap-[8px]",
                                div { class: "flex flex-row gap-[4px]",
                                    img {
                                        src: institution_badge_url,
                                        width: 24,
                                        height: 24,
                                    }
                                    div { class: "font-semibold text-[#222222] text-[14px]",
                                        "{project.policy_making_institution}"
                                    }
                                }
                                if project.project_area.is_some() {
                                    Label { name: project.project_area.unwrap().to_string() }
                                }
                            }
                        }
                    }

                    div { class: "flex flex-row w-full justify-between items-center",
                        div { class: "flex flex-row gap-[6px]",
                            User { width: "18", height: "18" }
                            div { class: "flex flex-row gap-[4px]",
                                div { class: "font-normal text-[14px] text-[#222222] leading-[17px]",
                                    "{tr.participant}"
                                }
                                div { class: "font-bold text-[14px] text-[#222222] leading-[17px]",
                                    {project.num_of_participation.to_formatted_string(&Locale::en)}
                                }
                            }
                        }

                        div { class: "flex flex-row gap-[6px]",
                            Vote { width: "18", height: "18" }
                            div { class: "flex flex-row gap-[4px]",
                                div { class: "font-normal text-[14px] text-[#222222] leading-[17px]",
                                    "{tr.vote}"
                                }
                                div { class: "font-bold text-[14px] text-[#222222] leading-[17px]",
                                    {project.num_of_vote.to_formatted_string(&Locale::en)}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Label(name: String) -> Element {
    rsx! {
        div { class: "inline-block w-fit p-[7px] border-[2px] border-[#7c8292] bg-white font-medium text-[14px] leading-[22.4px] text-[#555462] rounded-[100px]",
            "{name}"
        }
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
