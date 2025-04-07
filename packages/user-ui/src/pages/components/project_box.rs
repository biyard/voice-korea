use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::deliberation_project::DeliberationProjectSummary;
use num_format::{Locale, ToFormattedString};

use crate::{
    components::{
        button::Button,
        icons::{user::User, vote::Vote},
        label::Label,
    },
    pages::i18n::ProjectBoxTranslate,
    routes::Route,
};

#[component]
pub fn ProjectCard(lang: Language, deliberation: DeliberationProjectSummary) -> Element {
    let project_url = asset!("/public/images/project.png").to_string();
    let project_id = deliberation.id;
    let tr: ProjectBoxTranslate = translate(&lang);
    let nav = use_navigator();
    rsx! {
        div {
            class: "relative w-full h-420 rounded-[12px] shadow-md hover:shadow-xl overflow-hidden group",
            tabindex: 0,
            // Thumbnail
            div {
                background_image: format!("url({project_url})"),
                class: "h-260 bg-cover bg-center",
            }
            // Overlay
            div { class: "flex flex-col transition-all duration-400 ease-[cubic-bezier(.17,.67,.5,1.03)] absolute bottom-0 w-full bg-white rounded-[12px] px-16 py-20 h-170 group-hover:h-300 group-focus-within:h-300",
                div { class: "font-bold text-[16px] leading-normal text-text-black truncate",
                    "{deliberation.title}"
                }
                div { class: "font-normal text-text-gray text-sm/22 truncate flex-1",
                    "{deliberation.description}"
                }
                div { class: "group-hover:hidden group-focus-within:hidden",
                    Label { name: deliberation.project_area.translate(&lang) }
                }
                Button {
                    class: "hidden group-hover:block group-focus-within:block w-full py-8 border border-bt-grey bg-white rounded-full font-semibold text-black text-[15px]/25",
                    onclick: move |_| {
                        nav.push(Route::ProjectPage {
                            lang,
                            project_id,
                        });
                    },
                    "{tr.detail}"
                }
                div { class: "flex flex-row w-full justify-between items-center mt-16",
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
#[component]
pub fn ProjectBox(lang: Language, deliberation: DeliberationProjectSummary) -> Element {
    let project_url = asset!("/public/images/project.png").to_string();
    // let institution_badge_url = asset!("/public/images/institution_badge.png").to_string();
    let tr: ProjectBoxTranslate = translate(&lang);

    rsx! {
        div {
            class: "flex flex-col justify-end items-end overflow-hidden shadow-[0px_8px_20px_rgba(148,176,214,0.25)] rounded-[12px]",
            style: "background-image: url('{project_url}'); background-size: cover; height: 450px; width: 100%;",
            div { class: "flex flex-col w-full justify-start items-start rounded-[12px] bg-white px-16 pt-20 pb-12",
                div { class: "flex flex-col gap-[16px] w-full",
                    div { class: "flex flex-col gap-8",
                        div { class: "font-bold text-[16px] leading-normal text-text-black truncate",
                            "{deliberation.title}"
                        }
                        div { class: "flex flex-col gap-12",
                            div { class: "font-normal text-text-gray text-sm/22 h-20 truncate",
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
