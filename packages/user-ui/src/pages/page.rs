use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::deliberation_project::DeliberationProject;
use models::organization::OrganizationSummary;

use crate::components::icons::check::Check;
use crate::pages::components::inquiry::InquirySection;
use crate::pages::components::institution_box::InstitutionBox;
use crate::pages::components::project_box::ProjectBox;
use crate::pages::components::review::ReviewSection;
use crate::pages::i18n::{
    MainBannerTranslate, MoreButtonTranslate, OpinionFeatureTranslate, OpinionInstitutionTranslate,
    OpinionProjectTranslate, PriceSectionTranslate,
};
use crate::routes::Route;

use super::controller;
#[component]
pub fn MainPage(lang: Language) -> Element {
    let ctrl = controller::Controller::init(lang.clone())?;
    let data = ctrl.data()?;
    let deliberations = data.projects;
    let institutions = data.organizations;
    let deliberation_reviews = data.reviews;

    rsx! {
        // TODO(mobile): dashboard implemented to fit mobile size
        div { class: "flex flex-col w-full justify-center items-center gap-100",
            div { class: "flex flex-col w-full justify-center items-center gap-150",
                div { class: "flex flex-col w-full justify-center items-center gap-120",
                    div { class: "flex flex-col w-full justify-center items-center gap-50",
                        MainBanner { lang }

                        ContentSection { lang, deliberations, institutions }
                    }

                    PriceSection { lang }
                }

                InquirySection {
                    lang,
                    send_inquiry: move |(name, email, message): (String, String, String)| {
                        ctrl.send_inquiry(name, email, message);
                    },
                }
            }
            ReviewSection { lang, deliberation_reviews }
        }
    }
}

#[component]
pub fn PriceSection(lang: Language) -> Element {
    let tr: PriceSectionTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full h-fit justify-center items-center px-10",
            div { class: "max-[1300px]:flex max-[1300px]:flex-col max-[1300px]:h-full h-[620px] relative flex flex-col w-full  justify-center items-center",
                div { class: "max-[1300px]:flex max-[1300px]:flex-col relative flex flex-row w-full h-full max-w-1300 justify-center items-center",
                    div {
                        class: "absolute top-0 left-80 w-550 h-620 rounded-xl shadow-[0px_8px_20px_rgba(148,176,214,0.25)] px-40 py-25 bg-transparent max-[1300px]:relative max-[1300px]:w-full max-[1300px]:h-auto max-[1300px]:top-auto max-[1300px]:left-auto",
                        style: "z-index: 10;",
                        div { class: "flex flex-col w-full gap-32",
                            div { class: "flex flex-col w-full gap-40",
                                div { class: "flex flex-col w-full gap-20",
                                    div { class: "font-bold text-[28px] text-text-black leading-32",
                                        "{tr.free_title}"
                                    }
                                    div { class: "font-normal text-[15px] text-text-gray leading-22",
                                        "{tr.free_description}"
                                    }
                                }

                                div { class: "flex flex-col w-full gap-35",
                                    div { class: "flex flex-row gap-5 items-center",
                                        div { class: "font-semibold text-[40px] text-text-black",
                                            "0"
                                        }
                                        div { class: "font-medium text-xl text-light-gray",
                                            "{tr.won}"
                                        }
                                    }
                                    div { class: "flex flex-col w-full gap-5",
                                        InfoBox {
                                            label: "{tr.free_info_label_1}",
                                            description: "{tr.free_info_description_1}",
                                        }
                                        InfoBox {
                                            label: "{tr.free_info_label_2}",
                                            description: "{tr.free_info_description_2}",
                                        }
                                        InfoBox {
                                            label: "{tr.free_info_label_3}",
                                            description: "{tr.free_info_description_3}",
                                        }
                                    }
                                }
                            }

                            div {
                                class: "flex flex-row w-full h-50 justify-center items-center rounded-[100px] bg-button-primary font-semibold text-white text-[15px] cursor-pointer",
                                onclick: move |_| {
                                    tracing::debug!("start button clicked");
                                },
                                "{tr.start}"
                            }
                        }
                    }

                    div {
                        class: "absolute top-0 right-80 w-550 h-620 rounded-xl shadow-[0px_8px_20px_rgba(148,176,214,0.25)] px-40 py-25 bg-transparent max-[1300px]:relative max-[1300px]:w-full max-[1300px]:h-auto max-[1300px]:top-auto max-[1300px]:right-auto max-[1300px]:left-auto",
                        style: "z-index: 10;",
                        div { class: "flex flex-col w-full gap-32",
                            div { class: "flex flex-col w-full gap-40",
                                div { class: "flex flex-col w-full gap-20",
                                    div { class: "font-bold text-[28px] text-text-black leading-32",
                                        "{tr.premium_title}"
                                    }
                                    div { class: "font-normal text-[15px] text-text-gray leading-22",
                                        "{tr.premium_description}"
                                    }
                                }

                                div { class: "flex flex-col w-full gap-35",
                                    div { class: "flex flex-row gap-5 items-center",
                                        div { class: "font-semibold text-[40px] text-text-black",
                                            "2,990"
                                        }
                                        div { class: "font-medium text-xl text-light-gray",
                                            "{tr.won}"
                                        }
                                    }
                                    div { class: "flex flex-col w-full gap-5",
                                        InfoBox {
                                            label: "{tr.premium_info_label_1}",
                                            description: "{tr.premium_info_description_1}",
                                        }
                                        InfoBox {
                                            label: "{tr.premium_info_label_2}",
                                            description: "{tr.premium_info_description_2}",
                                        }
                                        InfoBox {
                                            label: "{tr.premium_info_label_3}",
                                            description: "{tr.premium_info_description_3}",
                                        }
                                        InfoBox {
                                            label: "{tr.premium_info_label_4}",
                                            description: "{tr.premium_info_description_4}",
                                        }
                                    }
                                }
                            }

                            div {
                                class: "flex flex-row w-full h-50 justify-center items-center rounded-[100px] bg-button-primary font-semibold text-white text-[15px] cursor-pointer",
                                onclick: move |_| {
                                    tracing::debug!("start button clicked");
                                },
                                "{tr.start}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn InfoBox(label: String, description: String) -> Element {
    rsx! {
        div { class: "flex flex-row gap-10 w-full justify-start items-center",
            div { class: "w-24 h-24",
                Check { width: "24", height: "24" }
            }
            div {
                class: "text-[15px] text-text-gray leading-22 gap-10",
                style: "word-wrap: break-word;",
                span { class: "font-bold mr-3", "{label}" }
                span { class: "font-normal", "{description}" }
            }
        }
    }
}

#[component]
pub fn ContentSection(
    lang: Language,
    deliberations: Vec<DeliberationProject>,
    institutions: Vec<OrganizationSummary>,
) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-center items-center gap-120",
            div { class: "flex flex-col w-full max-w-1300 justify-center items-center px-10",
                div { class: "flex flex-col w-full justify-start items-start gap-120",
                    DeliberationFeature { lang }
                    DeliberationProjectCard { lang, deliberations }
                }
            }
                // OpinionInstitution { lang, institutions }
        }
    }
}

#[component]
pub fn OpinionInstitution(lang: Language, institutions: Vec<OrganizationSummary>) -> Element {
    let tr: OpinionInstitutionTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-center items-center py-120 bg-gradient-to-b from-[#dbeae8] to-[#ffffff]",
            div { class: "flex flex-col w-full max-w-[1300px] justify-center items-center px-[10px] gap-[30px]",
                div { class: "flex flex-col w-full gap-[10px]",
                    div { class: "font-bold text-[28px] text-[#555462] leading-[32px]",
                        "{tr.institution}"
                    }
                    div { class: "font-normal text-[15px] text-[#555462] leading-[22.5px]",
                        "{tr.institution_description}"
                    }
                }
                div { class: "flex flex-col w-full gap-[40px]",
                    div { class: "grid grid-cols-5 gap-[20px]",
                        for institution in institutions {
                            Link {
                                to: Route::GovernancePage {
                                    lang,
                                    governance_id: institution.id,
                                },
                                InstitutionBox { lang, institution }
                            }
                        }
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
pub fn DeliberationProjectCard(lang: Language, deliberations: Vec<DeliberationProject>) -> Element {
    let tr: OpinionProjectTranslate = translate(&lang);
    let nav = use_navigator();
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-40",
            div { class: "flex flex-col gap-30 w-full",
                div { class: "flex flex-col gap-10",
                    div { class: "font-bold text-[28px] text-text-gray leading-32",
                        "{tr.project}"
                    }
                    div { class: "font-normal text-[15px] text-text-gray leading-22",
                        "{tr.project_description}"
                    }
                }

                div { class: "grid max-[600px]:grid-cols-1 max-[1000px]:grid-cols-2 grid-cols-3 gap-20",
                    for deliberation in deliberations.clone() {
                        div {
                            class: "cursor-pointer",
                            onclick: {
                                let project_id = deliberation.clone().id.clone();
                                move |_| {
                                    nav.push(Route::ProjectPage {
                                        lang,
                                        project_id,
                                    });
                                }
                            },
                            ProjectBox {
                                lang,
                                deliberation: deliberation.clone().into(),
                            }
                        }
                    }
                }

                div { class: "flex flex-wrap  justify-center items-center" }
                div { class: "flex flex-row w-full justify-center items-center",
                    MoreButton {
                        lang,
                        onclick: move |_| {
                            nav.push(Route::ProjectListPage { lang });
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
            class: "flex flex-row px-20 py-12 bg-button-primary font-semibold text-base text-white cursor-pointer rounded-xl",
            onclick: move |e: Event<MouseData>| {
                onclick.call(e);
            },
            "{tr.more}"
        }
    }
}

#[component]
pub fn DeliberationFeature(lang: Language) -> Element {
    let decentralized = asset!("/public/images/decentralized.png");
    let shield = asset!("public/images/shield.png");
    let sound = asset!("public/images/sound.png");

    let tr: OpinionFeatureTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-center items-center gap-32",
            div { class: "font-bold text-[28px] text-text-gray leading-32", "{tr.title}" }
            div { class: "flex flex-row max-[1000px]:flex-col w-full justify-center items-center gap-20",
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
            div { class: "font-semibold text-[15px] text-text-gray leading-22 text-center whitespace-pre-line",
                "{tr.description}"
            }
        }
    }
}

#[component]
pub fn FeatureBox(title: String, description: String, asset: Asset) -> Element {
    rsx! {
        div {
            class: "flex flex-col max-[1000px]:w-full max-[1000px]:max-w-600 w-310 justify-start items-start px-24 py-34 rounded-xl gap-20",
            style: "box-shadow: 0px 8px 20px rgba(148, 128, 214, 0.5);",
            div { class: "font-bold text-lg text-text-black", "{title}" }
            div { class: "font-normal text-[15px] text-text-gray", "{description}" }
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
        div { class: "flex flex-col w-full max-w-1300 px-10",
            div { class: "relative flex flex-col w-full max-[500px]:h-fit h-320 max-[500px]:p-25 p-65 justify-center items-start rounded-2xl gap-10 overflow-hidden ",
                div {
                    class: "absolute inset-0  bg-cover bg-center opacity-80 rounded-2xl",
                    style: "background-image: url({background_url});",
                }
                div { class: "relative font-bold text-[40px] leading-58 text-white",
                    "{tr.title}"
                }
                div { class: "relative font-medium text-16 leading-24 text-white whitespace-pre-line mb-12",
                    "{tr.description}"
                }
                //TODO:Go to public opinion survey page
                button {
                    class: "relative flex flex-row px-16 py-12 bg-[#5b373b] border border-white rounded-xl font-semibold text-base text-white cursor-pointer",
                    onclick: move |_| {},
                    "{tr.button}"
                }
            }
        }
    }
}
