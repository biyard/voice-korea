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
    let public_opinion_reviews = data.reviews;

    rsx! {
        // TODO(mobile): dashboard implemented to fit mobile size
        div { class: "flex flex-col w-full justify-center items-center gap-[100px]",
            div { class: "flex flex-col w-full justify-center items-center gap-[150px]",
                div { class: "flex flex-col w-full justify-center items-center gap-[50px]",
                    MainBanner { lang }

                    ContentSection { lang, deliberations, institutions }

                    PriceSection { lang }
                }

                InquirySection {
                    lang,
                    send_inquiry: move |(name, email, message): (String, String, String)| {
                        ctrl.send_inquiry(name, email, message);
                    },
                }
            }
            ReviewSection { lang, public_opinion_reviews }
        }
    }
}

#[component]
pub fn PriceSection(lang: Language) -> Element {
    let tr: PriceSectionTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full h-[620px] justify-center items-center",

            div { class: "relative flex flex-col w-full h-full justify-center items-center",
                div { class: "relative flex flex-row w-full h-full max-w-[1300px] justify-center items-center",
                    div {
                        class: "absolute top-0 left-[80px] w-[550px] h-[620px] rounded-[12px] shadow-[0px_8px_20px_rgba(148,176,214,0.25)] px-[40px] py-[25px] bg-transparent",
                        style: "z-index: 10;",
                        div { class: "flex flex-col w-full gap-[32px]",
                            div { class: "flex flex-col w-full gap-[40px]",
                                div { class: "flex flex-col w-full gap-[20px]",
                                    div { class: "font-bold text-[28px] text-[#222222] leading-[32px]",
                                        "{tr.free_title}"
                                    }
                                    div { class: "font-normal text-[15px] text-[#555462] leading-[22px]",
                                        "{tr.free_description}"
                                    }
                                }

                                div { class: "flex flex-col w-full gap-[35px]",
                                    div { class: "flex flex-row gap-[5px] items-center",
                                        div { class: "font-semibold text-[40px] text-[#222222]",
                                            "0"
                                        }
                                        div { class: "font-medium text-[20px] text-[#7c8292]",
                                            "{tr.won}"
                                        }
                                    }
                                    div { class: "flex flex-col w-full gap-[5px]",
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
                                class: "flex flex-row w-full h-[50px] justify-center items-center rounded-[100px] bg-[#8095ea] font-semibold text-white text-[15px] cursor-pointer",
                                onclick: move |_| {
                                    tracing::debug!("start button clicked");
                                },
                                "{tr.start}"
                            }
                        }
                    }

                    div {
                        class: "absolute top-0 right-[80px] w-[550px] h-[620px] rounded-[12px] shadow-[0px_8px_20px_rgba(148,176,214,0.25)] px-[40px] py-[25px] bg-transparent",
                        style: "z-index: 10;",
                        div { class: "flex flex-col w-full gap-[32px]",
                            div { class: "flex flex-col w-full gap-[40px]",
                                div { class: "flex flex-col w-full gap-[20px]",
                                    div { class: "font-bold text-[28px] text-[#222222] leading-[32px]",
                                        "{tr.premium_title}"
                                    }
                                    div { class: "font-normal text-[15px] text-[#555462] leading-[22px]",
                                        "{tr.premium_description}"
                                    }
                                }

                                div { class: "flex flex-col w-full gap-[35px]",
                                    div { class: "flex flex-row gap-[5px] items-center",
                                        div { class: "font-semibold text-[40px] text-[#222222]",
                                            "2,990"
                                        }
                                        div { class: "font-medium text-[20px] text-[#7c8292]",
                                            "{tr.won}"
                                        }
                                    }
                                    div { class: "flex flex-col w-full gap-[5px]",
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
                                class: "flex flex-row w-full h-[50px] justify-center items-center rounded-[100px] bg-[#8095ea] font-semibold text-white text-[15px] cursor-pointer",
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
        div { class: "flex flex-row gap-[10px] w-full justify-start items-center",
            div { class: "w-[24px] h-[24px]",
                Check { width: "24", height: "24" }
            }
            div {
                class: "text-[15px] text-[#555462] leading-[22px] gap-[10px]",
                style: "word-wrap: break-word;",
                span { class: "font-bold mr-[3px]", "{label}" }
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
        div { class: "flex flex-col w-full justify-center items-center gap-[120px]",
            div { class: "flex flex-col w-full max-w-[1300px] justify-center items-center px-[10px]",
                div { class: "flex flex-col w-full justify-start items-start gap-[120px]",
                    OpinionFeature { lang }
                    OpinionProject { lang, deliberations }
                }
            }

            OpinionInstitution { lang, institutions }
        }
    }
}

#[component]
pub fn OpinionInstitution(lang: Language, institutions: Vec<OrganizationSummary>) -> Element {
    let tr: OpinionInstitutionTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-center items-center py-[120px] bg-gradient-to-b from-[#dbeae8] to-[#ffffff]",
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
pub fn OpinionProject(lang: Language, deliberations: Vec<DeliberationProject>) -> Element {
    let tr: OpinionProjectTranslate = translate(&lang);
    let nav = use_navigator();
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-[40px]",
            div { class: "flex flex-col gap-[30px] w-full",
                div { class: "flex flex-col gap-[10px]",
                    div { class: "font-bold text-[28px] text-[#555462] leading-[32px]",
                        "{tr.project}"
                    }
                    div { class: "font-normal text-[15px] text-[#555462] leading-[22px]",
                        "{tr.project_description}"
                    }
                }

                div { class: "grid grid-cols-3 gap-[20px]",
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
            class: "flex flex-row px-[20px] py-[12px] bg-[#8095ea] font-semibold text-[16px] text-white cursor-pointer rounded-[12px]",
            onclick: move |e: Event<MouseData>| {
                onclick.call(e);
            },
            "{tr.more}"
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
        div { class: "flex flex-col w-full max-w-[1300px] px-[10px]",
            div { class: "relative flex flex-col w-full h-[320px] justify-center items-start rounded-2xl p-[65px] gap-[10px] overflow-hidden ",
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
}
