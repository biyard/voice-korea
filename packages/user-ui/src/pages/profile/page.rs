use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::User;

use crate::{
    components::icons::badge::Badge,
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

    let user_id = ctrl.user_id;

    let project = ctrl.projects()?;

    rsx! {
        div { class: "flex flex-col w-full justify-center items-center mt-80",
            div { class: "flex flex-col max-w-1300 w-full justify-start items-start gap-60",
                ProfileBanner { lang, profile: project.user }

                div { class: "flex flex-col w-full justify-start items-start gap-40",
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

                    div { class: "flex flex-col w-full justify-start items-start gap-10",
                        // SearchBox {
                        //     placeholder: "{tr.search}",
                        //     value: keyword,
                        //     onsearch: move |e| {
                        //         ctrl.change_keyword(e);
                        //     },
                        // }

                        if ctrl.get_selected_type() == controller::ProjectType::Design {
                            DesignedTable {
                                lang,
                                projects: project.designed_projects,
                                user_id,
                            }
                        } else {
                            ParticipantTable {
                                lang,
                                projects: project.participated_projects,
                            }
                        }
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
            class: if clicked { "flex flex-row px-20 py-10 bg-white border-b border-b-black cursor-pointer" } else { "flex flex-row px-20 py-10 bg-white cursor-pointer" },
            onclick: move |e: Event<MouseData>| {
                onclick.call(e);
            },
            "{type_name}"
        }
    }
}

#[component]
pub fn ProfileBanner(lang: Language, profile: User) -> Element {
    let tr: ProfileBannerTranslate = translate(&lang);
    let banner_url = asset!("/public/images/profile-banner.png").to_string();
    let date = formatted_timestamp(profile.created_at);

    let nickname = match profile.nickname {
        Some(v) => v,
        None => profile.email,
    };

    rsx! {
        div { class: "relative flex flex-col w-full h-200 justify-start items-center",
            div { class: "relative flex flex-row w-full h-150 justify-end items-center rounded-2xl py-52 px-73 gap-48 overflow-hidden",
                div {
                    class: "absolute inset-0 bg-cover bg-center rounded-2xl",
                    style: "background-image: url({banner_url});",
                }
                        // div { class: "flex flex-col w-fit gap-[3px] justify-center items-center",
            //     div { class: "font-bold text-white text-[20px] z-1", "{projects}" }
            //     div { class: "font-normal text-white text-[14px] z-1", "{tr.total_project}" }
            // }
            // div { class: "flex flex-col w-fit gap-[3px] justify-center items-center",
            //     div { class: "font-bold text-white text-[20px] z-1", "{votes}" }
            //     div { class: "font-normal text-white text-[14px] z-1", "{tr.vote}" }
            // }
            // div { class: "flex flex-col w-fit gap-[3px] justify-center items-center",
            //     div { class: "font-bold text-white text-[20px] z-1", "{tokens}" }
            //     div { class: "font-normal text-white text-[14px] z-1", "{tr.token}" }
            // }
            }
            div { class: "absolute flex flex-row justify-center items-center left-48 bottom-0 rounded-[100px] bg-white w-100 h-100",
                div {
                    div { class: "w-80 h-80 rounded-[100px] bg-profile-gray" }
                }
            }
            div { class: "absolute right-0 bottom-10",
                div { class: "font-normal text-text-black text-sm", "{tr.create_account} {date}" }
            }
            div { class: "absolute flex flex-row w-fit gap-13 left-160 bottom-0",
                div { class: "font-bold text-black text-[28px] leading-32", "{nickname}" }
                div { class: "flex flex-row w-33 h-33", Badge {} }
            }
        }
    }
}
