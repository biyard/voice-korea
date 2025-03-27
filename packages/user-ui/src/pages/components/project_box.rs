use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::deliberation_project::DeliberationProjectSummary;
use num_format::{Locale, ToFormattedString};

use crate::{
    components::icons::{user::User, vote::Vote},
    pages::i18n::ProjectBoxTranslate,
};

#[component]
pub fn ProjectBox(lang: Language, deliberation: DeliberationProjectSummary) -> Element {
    let project_url = asset!("/public/images/project.png").to_string();
    // let institution_badge_url = asset!("/public/images/institution_badge.png").to_string();
    let tr: ProjectBoxTranslate = translate(&lang);

    rsx! {
        div {
            class: "flex flex-col justify-end items-end rounded-[30px] shadow-[0px_8px_20px_rgba(148,176,214,0.25)]",
            style: "background-image: url('{project_url}'); background-size: cover; height: 450px; width: 100%;",
            div { class: "flex flex-col w-full justify-start items-start rounded-[20px] bg-white px-16 pt-20 pb-12",
                div { class: "flex flex-col gap-[16px] w-full",
                    div { class: "flex flex-col gap-8",
                        div { class: "font-bold text-lg text-text-black h-30 truncate",
                            "{deliberation.title}"
                        }
                        div { class: "flex flex-col gap-12",
                            div { class: "font-normal text-text-gray text-sm h-20 truncate",
                                "{deliberation.description}"
                            }
                            div { class: "flex flex-col gap-8",
                                // div { class: "flex flex-row gap-4",
                                //     img {
                                //         src: institution_badge_url,
                                //         width: 24,
                                //         height: 24,
                                //     }
                                //     div { class: "font-semibold text-text-black text-sm",
                                //         "Organizations"
                                //     }
                                // }

                                Label { name: deliberation.project_area.translate(&lang) }
                            }
                        }
                    }

                    div { class: "flex flex-row w-full justify-between items-center",
                        div { class: "flex flex-row gap-6",
                            User { width: "18", height: "18" }
                            div { class: "flex flex-row gap-4",
                                div { class: "font-normal text-sm text-text-black leading-17",
                                    "{tr.participant}"
                                }
                                div { class: "font-bold text-sm text-text-black leading-17",
                                    {deliberation.participants.to_formatted_string(&Locale::en)}
                                }
                            }
                        }

                        div { class: "flex flex-row gap-6",
                            Vote { width: "18", height: "18" }
                            div { class: "flex flex-row gap-4",
                                div { class: "font-normal text-sm text-text-black leading-17",
                                    "{tr.vote}"
                                }
                                div { class: "font-bold text-sm text-text-black leading-17",
                                    {deliberation.votes.to_formatted_string(&Locale::en)}
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
        div { class: "inline-block w-fit px-12 py-7 border-2 border-light-gray bg-white font-medium text-sm leading-22 text-text-gray rounded-[100px]",
            "{name}"
        }
    }
}
