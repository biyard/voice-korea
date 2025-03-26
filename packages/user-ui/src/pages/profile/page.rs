use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::profile::ProfileSummary;
use num_format::{Locale, ToFormattedString};

use crate::{
    components::{icons::badge::Badge, search::SearchBox},
    pages::profile::{
        components::{designed_table::DesignedTable, participant_table::ParticipantTable},
        controller,
        i18n::{ProfileBannerTranslate, ProfileTranslate},
    },
    utils::time::formatted_timestamp,
};

#[component]
pub fn ProfilePage(lang: Language) -> Element {
    let mut ctrl = controller::Controller::init(lang)?;
    let tr: ProfileTranslate = translate(&lang);

    let profile = ctrl.get_profile();
    let designed_projects = profile.clone().designed_projects;
    let participant_projects = profile.clone().participant_projects;
    let keyword = ctrl.get_keyword();

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-[60px]",
            ProfileBanner { lang, profile }

            div { class: "flex flex-col w-full justify-start items-start gap-[40px]",
                div { class: "flex flex-row w-full justify-start items-start",
                    ClickableType {
                        type_name: "{tr.designed_project}",
                        clicked: ctrl.get_selected_type() == controller::ProjectType::Design,
                        onclick: move |_| {
                            ctrl.change_selected_type(controller::ProjectType::Design);
                        },
                    }
                    ClickableType {
                        type_name: "{tr.participant_project}",
                        clicked: ctrl.get_selected_type() == controller::ProjectType::Participation,
                        onclick: move |_| {
                            ctrl.change_selected_type(controller::ProjectType::Participation);
                        },
                    }
                }

                div { class: "flex flex-col w-full justify-start items-start gap-[10px]",
                    SearchBox {
                        placeholder: "{tr.search}",
                        value: keyword,
                        onsearch: move |e| {
                            ctrl.change_keyword(e);
                        },
                    }

                    if ctrl.get_selected_type() == controller::ProjectType::Design {
                        DesignedTable { lang, projects: designed_projects }
                    } else {
                        ParticipantTable { lang, projects: participant_projects }
                    }
                }

            }

        }
    }
}

#[component]
pub fn ClickableType(
    type_name: String,
    clicked: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div {
            class: if clicked { "flex flex-row px-[20px] py-[10px] bg-white border-b border-b-black cursor-pointer" } else { "flex flex-row px-[20px] py-[10px] bg-white cursor-pointer" },
            onclick: move |e: Event<MouseData>| {
                onclick.call(e);
            },
            "{type_name}"
        }
    }
}

#[component]
pub fn ProfileBanner(lang: Language, profile: ProfileSummary) -> Element {
    let tr: ProfileBannerTranslate = translate(&lang);
    let banner_url = asset!("/public/images/profile-banner.png").to_string();
    let date = formatted_timestamp(profile.created_at);

    let projects = profile.num_of_projects.to_formatted_string(&Locale::ko);
    let votes = profile.num_of_votes.to_formatted_string(&Locale::ko);
    let tokens = profile.num_of_tokens.to_formatted_string(&Locale::ko);
    rsx! {
        div { class: "relative flex flex-col w-full h-[200px] justify-start items-center",
            div { class: "relative flex flex-row w-full h-[150px] justify-end items-center rounded-[16px] py-[52px] px-[73px] gap-[48px] overflow-hidden",
                div {
                    class: "absolute inset-0 bg-cover bg-center rounded-2xl",
                    style: "background-image: url({banner_url});",
                }
                div { class: "flex flex-col w-fit gap-[3px] justify-center items-center",
                    div { class: "font-bold text-white text-[20px] z-1", "{projects}" }
                    div { class: "font-normal text-white text-[14px] z-1", "{tr.total_project}" }
                }
                div { class: "flex flex-col w-fit gap-[3px] justify-center items-center",
                    div { class: "font-bold text-white text-[20px] z-1", "{votes}" }
                    div { class: "font-normal text-white text-[14px] z-1", "{tr.vote}" }
                }
                div { class: "flex flex-col w-fit gap-[3px] justify-center items-center",
                    div { class: "font-bold text-white text-[20px] z-1", "{tokens}" }
                    div { class: "font-normal text-white text-[14px] z-1", "{tr.token}" }
                }
            }
            div { class: "absolute flex flex-row justify-center items-center left-[40px] bottom-[0px] rounded-[100px] bg-white w-[100px] h-[100px]",
                div {
                    div { class: "w-[80px] h-[80px] rounded-[100px] bg-[#d9d9d9]" }
                }
            }
            div { class: "absolute right-[0px] bottom-[10px]",
                div { class: "font-normal text-[#222222] text-[14px]", "{tr.create_account} {date}" }
            }
            div { class: "absolute flex flex-row w-fit gap-[13px] left-[160px] bottom-[0px]",
                div { class: "font-bold text-[#000000] text-[28px] leading-[32px]",
                    "{profile.address}"
                }
                div { class: "flex flex-row w-[33px] h-[33px]", Badge {} }
            }
        }
    }
}
