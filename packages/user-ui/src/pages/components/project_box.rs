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
    let institution_badge_url = asset!("/public/images/institution_badge.png").to_string();
    let tr: ProjectBoxTranslate = translate(&lang);

    rsx! {
        div {
            class: "flex flex-col justify-end items-end rounded-[30px] shadow-[0px_8px_20px_rgba(148,176,214,0.25)]",
            style: "background-image: url('{project_url}'); background-size: cover; height: 450px; width: 100%;",
            div { class: "flex flex-col w-full justify-start items-start rounded-[20px] bg-white px-[16px] pt-[20px] pb-[12px]",
                div { class: "flex flex-col gap-[16px] w-full",
                    div { class: "flex flex-col gap-[8px]",
                        div { class: "font-bold text-[18px] text-[#222222] h-[30px] truncate",
                            "{deliberation.title}"
                        }
                        div { class: "flex flex-col gap-[12px]",
                            div { class: "font-normal text-[#555462] text-[14px] h-[20px] truncate",
                                "{deliberation.description}"
                            }
                            div { class: "flex flex-col gap-[8px]",
                                div { class: "flex flex-row gap-[4px]",
                                    img {
                                        src: institution_badge_url,
                                        width: 24,
                                        height: 24,
                                    }
                                    div { class: "font-semibold text-[#222222] text-[14px]",
                                        "Organizations"
                                    }
                                }

                                Label { name: deliberation.project_area.to_string() }
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
                                    {deliberation.participants.to_formatted_string(&Locale::en)}
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
        div { class: "inline-block w-fit p-[7px] border-[2px] border-[#7c8292] bg-white font-medium text-[14px] leading-[22.4px] text-[#555462] rounded-[100px]",
            "{name}"
        }
    }
}
