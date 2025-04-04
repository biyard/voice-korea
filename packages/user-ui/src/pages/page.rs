#![allow(dead_code, unused)]

use by_components::icons::upload_download::Download1;
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
    GuideLineTranslate, MainBannerTranslate, MoreButtonTranslate, OpinionFeatureTranslate,
    OpinionInstitutionTranslate, OpinionProjectTranslate, PriceSectionTranslate,
};
use crate::routes::Route;

use super::controller;
#[component]
pub fn MainPage(lang: Language) -> Element {
    let mut ctrl = controller::Controller::init(lang.clone())?;
    let data = ctrl.data()?;
    let deliberations = data.projects;
    let institutions = data.organizations;
    let deliberation_reviews = data.reviews;

    let comments = ctrl.get_comments();
    let page = ctrl.page();
    let total_pages = ctrl.total_pages();
    rsx! {
        div { class: "flex flex-col w-full justify-center items-center gap-60 tablet:gap-90 desktop:gap-120 [&>section]:scroll-mt-(--header-height)",

            MainSection { lang }
            ProjectSection { lang, deliberations }
            InstitutionSection { lang, institutions }
            PriceSection { lang }
            InquirySection {
                lang,
                send_inquiry: move |(name, email, message): (String, String, String)| async move {
                    ctrl.send_inquiry(name, email, message).await;
                },
            }
            ReviewSection {
                lang,
                comments,
                page,
                total_pages,
                set_page: move |page: i64| {
                    ctrl.set_page(page as usize);
                },
            }
        }
        GuideSection { lang }
    }
}
#[component]
pub fn GuideSection(lang: Language) -> Element {
    let survey_design = asset!("/public/images/survey_design.png");
    let survey_participation = asset!("/public/images/survey_participation.png");
    let tr: GuideLineTranslate = translate(&lang);

    rsx! {
        section {
            id: "guide",
            class: "flex flex-col w-full justify-center items-center pt-80 pb-135 gap-50 bg-netural-9",
            div { class: "flex flex-col w-full justify-center items-center gap-10 px-20 desktop:px-0",
                div { class: "font-bold text-[28px]/32 text-white", "{tr.guideline}" }
                div { class: "font-normal text-[15px]/22 text-center text-white break-keep desktop:whitespace-pre-line",
                    "{tr.guideline_desc}"
                }
            }

            div { class: "flex max-[750px]:flex-col flex-row justify-center items-center gap-20",
                div { class: "flex flex-col gap-20 justify-center items-center",
                    img { src: "{survey_design}", width: 310, height: 200 }
                    div { class: "flex flex-row w-fit justify-center items-center gap-5 px-16 py-12 bg-transparent border border-white rounded-xl",
                        div { class: "w-24 h-24",
                            Download1 {
                                width: "24",
                                height: "24",
                                fill: "none",
                                class: "[&>path]:stroke-white",
                            }
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
                            Download1 {
                                width: "24",
                                height: "24",
                                fill: "none",
                                class: "[&>path]:stroke-white",
                            }
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
    }
}
#[component]
pub fn MainSection(lang: Language) -> Element {
    let nav = use_navigator();
    let background_url = asset!("/public/images/main_image.jpeg").to_string();
    let console_url = &crate::config::get().console_url;
    let tr: MainBannerTranslate = translate(&lang);
    rsx! {
        section {
            id: "service",
            class: "flex flex-col w-full justify-center items-center",
            div { class: "flex flex-col w-full max-w-1300 items-start justify-start gap-50 px-20 desktop:px-0",
                div { class: "relative flex flex-col w-full max-[500px]:h-fit h-320 max-[500px]:p-25 p-65 justify-center items-start rounded-2xl gap-10 overflow-hidden",
                    div {
                        class: "absolute inset-0 bg-cover bg-center opacity-80 rounded-2xl",
                        style: "background-image: url({background_url});",
                    }
                    div { class: "relative font-bold text-xl tablet:text-[40px]/58 text-white",
                        "{tr.title}"
                    }
                    div { class: "relative font-medium text-sm tablet:text-[16px]/24  text-white whitespace-pre-line mb-12",
                        "{tr.description}"
                    }
                    button {
                        class: "relative flex flex-row text-sm px-10 py-8 desktop:text-base desktop:px-16 desktop:py-12 bg-[#5b373b] border border-white rounded-xl font-semibold  text-white cursor-pointer",
                        onclick: move |_| {
                            nav.push(format!("{}", console_url));
                        },
                        "{tr.button}"
                    }
                }

                DeliberationFeature { lang }
            }
        }
    }
}

// #[component]
// pub fn PriceSection(lang: Language) -> Element {
//     let tr: PriceSectionTranslate = translate(&lang);
//     rsx! {
//         section {
//             id: "price",
//             class: "w-full flex justify-center bg-[#F1F3FA] mt-32 mb-70",
//             div { class: "w-full flex flex-row justify-center self-center gap-130 tablet:px-20",
//                 div { class: "flex flex-col w-420 py-60",
//                     div { class: "text-2xl tablet:text-[28px]/32 mb-20", "{tr.free_title}" }
//                     div { class: "font-normal text-[15px] text-text-gray leading-22 mb-40",
//                         "{tr.free_description}"
//                     }
//                     div { class: "flex flex-row gap-5 items-center mb-34",
//                         div { class: "font-medium text-[40px] text-text-black", "0" }
//                         div { class: "font-medium text-xl text-light-gray", "{tr.won}" }
//                     }
//                     div { class: "flex flex-col w-full gap-14",
//                         InfoBox {
//                             label: "{tr.free_info_label_1}",
//                             description: "{tr.free_info_description_1}",
//                         }
//                         InfoBox {
//                             label: "{tr.free_info_label_2}",
//                             description: "{tr.free_info_description_2}",
//                         }
//                         InfoBox {
//                             label: "{tr.free_info_label_3}",
//                             description: "{tr.free_info_description_3}",
//                         }
//                     }
//                 }
//                 div { class: "relative w-530",
//                     div { class: "absolute rounded-xl shadow-[0px_8px_20px_0px_rgba(148,176,214,0.50)] px-55 py-16 bg-white left-0 top-1/2 translate-y-[-50%]",
//                         div { class: "flex flex-col w-full gap-20 max-w-420",
//                             div { class: "font-bold text-[28px] text-text-black leading-32",
//                                 "{tr.premium_title}"
//                             }
//                             div { class: "font-normal text-[15px] text-text-gray leading-22",
//                                 "{tr.premium_description}"
//                             }

//                             div { class: "flex flex-row gap-5 items-center",
//                                 div { class: "font-medium text-[40px] text-text-black",
//                                     "2,990"
//                                 }
//                                 div { class: "font-medium text-xl text-light-gray",
//                                     "{tr.won}"
//                                 }
//                             }
//                             div { class: "flex flex-col w-full gap-5",
//                                 InfoBox {
//                                     label: "{tr.premium_info_label_1}",
//                                     description: "{tr.premium_info_description_1}",
//                                 }
//                                 InfoBox {
//                                     label: "{tr.premium_info_label_2}",
//                                     description: "{tr.premium_info_description_2}",
//                                 }
//                                 InfoBox {
//                                     label: "{tr.premium_info_label_3}",
//                                     description: "{tr.premium_info_description_3}",
//                                 }
//                                 InfoBox {
//                                     label: "{tr.premium_info_label_4}",
//                                     description: "{tr.premium_info_description_4}",
//                                 }
//                             }
//                             button {
//                                 class: "flex flex-row w-full py-13 mt-32 justify-center items-center rounded-[100px] bg-button-primary font-semibold text-white text-[15px]/25 cursor-pointer",
//                                 onclick: move |_| {
//                                     tracing::debug!("start button clicked");
//                                 },
//                                 "{tr.start}"
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }

#[component]
pub fn PriceSection(lang: Language) -> Element {
    let tr: PriceSectionTranslate = translate(&lang);
    rsx! {
        section { id: "price", class: "relative w-full flex justify-center",
            div { class: "absolute w-full bg-[#F1F3FA] left-0 top-1/2 translate-y-[-50%] h-480" }
            div { class: "w-full flex flex-col tablet:flex-row justify-center self-center gap-30 desktop:gap-130 tablet:px-20 z-1",
                div { class: "rounded-xl shadow-[0px_8px_20px_0px_rgba(148,176,214,0.50)] p-30 desktop:px-55 desktop:py-16 bg-white",
                    div { class: "flex flex-col w-full gap-20 desktop:max-w-420 h-full",
                        div { class: "font-bold text-[28px] text-text-black leading-32",
                            "{tr.free_title}"
                        }
                        div { class: "font-normal text-[15px] text-text-gray leading-22",
                            "{tr.free_description}"
                        }

                        div { class: "flex flex-row gap-5 items-center",
                            div { class: "font-medium text-[40px] text-text-black",
                                "0"
                            }
                            div { class: "font-medium text-xl text-light-gray", "{tr.won}" }
                        }
                        div { class: "flex flex-col w-full gap-14 mb-auto",
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
                        button {
                            class: "flex flex-row w-full py-13 mt-32 justify-center items-center rounded-[100px] bg-button-primary font-semibold text-white text-[15px]/25 cursor-pointer",
                            onclick: move |_| {
                                tracing::debug!("start button clicked");
                            },
                            "{tr.start}"
                        }
                    }
                }
                div { class: "rounded-xl shadow-[0px_8px_20px_0px_rgba(148,176,214,0.50)] p-30 desktop:px-55 desktop:py-16 bg-white",
                    div { class: "flex flex-col w-full gap-20 desktop:max-w-420",
                        div { class: "font-bold text-[28px] text-text-black leading-32",
                            "{tr.premium_title}"
                        }
                        div { class: "font-normal text-[15px] text-text-gray leading-22",
                            "{tr.premium_description}"
                        }

                        div { class: "flex flex-row gap-5 items-center",
                            div { class: "font-medium text-[40px] text-text-black",
                                "2,990"
                            }
                            div { class: "font-medium text-xl text-light-gray", "{tr.won}" }
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
                        button {
                            class: "flex flex-row w-full py-13 mt-32 justify-center items-center rounded-[100px] bg-button-primary font-semibold text-white text-[15px]/25 cursor-pointer",
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
pub fn InstitutionSection(lang: Language, institutions: Vec<OrganizationSummary>) -> Element {
    let tr: OpinionInstitutionTranslate = translate(&lang);
    rsx! {
        section { class: "flex flex-col w-full justify-center items-center bg-gradient-to-b from-gradient-green to-white px-20 desktop:px-0",
            div {
                id: "institution",
                class: "flex flex-col w-full max-w-1300 justify-center pt-60 desktop:pt-120",
                SectionHeader {
                    title: tr.institution,
                    description: Some(tr.institution_description.to_string()),
                }
                div { class: "flex flex-col w-full gap-40 py-32",
                    div { class: "grid grid-cols-1 tablet:grid-cols-3 desktop:grid-cols-5 gap-20 [&>:nth-child(n+4)]:hidden tablet:[&>:nth-child(n+4)]:block tablet:[&>:nth-child(n+7)]:hidden desktop:[&>*]:!block",
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
pub fn ProjectSection(lang: Language, deliberations: Vec<DeliberationProject>) -> Element {
    let tr: OpinionProjectTranslate = translate(&lang);
    let nav = use_navigator();
    rsx! {
        section {
            id: "project",
            class: "relative flex flex-col w-full justify-center items-center",
            div { class: "absolute w-full bg-primary blur-[250px] h-130" }
            div { class: "max-w-1300 w-full z-1 px-20 desktop:px-0",
                SectionHeader {
                    title: tr.project,
                    description: Some(tr.project_description.to_string()),
                }
                div { class: "grid grid-cols-1 tablet:grid-cols-2 desktop:grid-cols-3 gap-20 w-full mt-30 [&>:nth-child(n+3)]:hidden tablet:[&>:nth-child(n+3)]:block tablet:[&>:nth-child(n+5)]:hidden desktop:[&>*]:!block",
                    for deliberation in deliberations.iter().take(6).cloned() {
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

                div { class: "flex flex-row w-full justify-center items-center mt-40",
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
pub fn SectionHeader(
    title: String,
    #[props(default = None)] description: Option<String>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        div {..attributes,
            h1 { class: "font-bold text-2xl tablet:text-[28px]/32 text-text-gray",
                "{title}"
            }
            if let Some(desc) = description {
                h2 { class: "font-normal text-[15px]/22 text-text-gray self-start mt-10",
                    "{desc}"
                }
            }
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
            SectionHeader { title: tr.title }
            div { class: "flex flex-col desktop:flex-row w-full justify-center items-center gap-20",
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
            div { class: "font-semibold text-[15px] text-text-gray leading-22 text-left px-10 break-keep desktop:px-0 desktop:text-center desktop:whitespace-pre-line",
                "{tr.description}"
            }
        }
    }
}

#[component]
pub fn FeatureBox(title: String, description: String, asset: Asset) -> Element {
    rsx! {
        div {
            class: "flex flex-col w-full desktop:w-310 justify-start items-start px-24 py-34 rounded-xl gap-20",
            style: "box-shadow: 0px 8px 20px rgba(148, 128, 214, 0.5);",
            div { class: "font-bold text-lg text-text-black", "{title}" }
            div { class: "font-normal text-[15px] text-text-gray", "{description}" }
            div { class: "flex flex-row w-full justify-end items-end hidden desktop:block",
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
