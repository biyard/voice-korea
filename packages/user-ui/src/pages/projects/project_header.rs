use bdk::prelude::*;

use by_components::icons::alignments::AlignJustify;
use dioxus_translate::{translate, Language};

use crate::{
    components::{header::GoogleLoginPopup, icons},
    routes::Route,
    service::{popup_service::PopupService, user_service::UserService},
};

#[component]
pub fn ProjectHeader(lang: Language) -> Element {
    let tr: ProjectHeaderTranslate = translate(&lang);

    let user_service: UserService = use_context();
    let mut popup_service: PopupService = use_context();

    let email = (user_service.email)();

    let onclick = {
        let email = email.clone();
        move |_e: Event<MouseData>| {
            tracing::debug!("signup button clicked");

            if email != "" {
                return;
            }

            popup_service
                .open(rsx! {
                    GoogleLoginPopup {
                        lang: lang.clone(),
                        onclose: move |_| {
                            popup_service.close();
                        },
                    }
                })
                .with_id("google_login")
                .with_title(tr.login);
        }
    };

    rsx! {
        div { class: "block max-[1300px]:!hidden",
            ProjectDesktopHeader { lang, email: email.clone(), onclick: onclick.clone() }
        }
        div { class: "hidden max-[1300px]:!block",
            ProjectMobileHeader { lang, email, onclick }
        }
    }
}

#[component]
pub fn ProjectMobileHeader(
    lang: Language,
    email: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let nav = use_navigator();
    let translates: ProjectHeaderTranslate = translate(&lang);
    let mut expanded = use_signal(|| false);
    let custom_class = "fixed top-0 left-0 z-100";

    rsx! {
        div { class: "{custom_class} w-full h-70 flex flex-row items-center justify-between bg-white px-[20px]",
            button {
                class: "cursor-pointer flex flex-row items-center justify-around gap-4 h-full w-fit",
                onclick: move |_| {
                    nav.push(Route::MainPage { lang });
                    expanded.set(false);
                },
                icons::Logo {}
                div { class: "font-extrabold text-base text-logo", "VOICE KOREA" }
            }
            button {
                onclick: move |_| {
                    expanded.set(!expanded());
                },
                AlignJustify { class: "cursor-pointer w-[30px] h-[30px] text-black" }
            }
        }

        if expanded() {
            div { class: "fixed top-70 left-0 w-full h-full grow bg-white flex flex-col items-start text-black z-100 px-20 py-[20px]",
                div { class: "flex flex-col font-bold justify-start items-start text-key-gray text-15 leading-19",
                    button {
                        class: "cursor-pointer flex flex-row w-full h-50 justify-start items-start",
                        onclick: move |_| {
                            nav.push(Route::ComingSoonPage { lang });
                            expanded.set(false);
                        },
                        {translates.space}
                    }
                    button {
                        class: "cursor-pointer flex flex-row w-full h-50 justify-start items-start",
                        onclick: move |_| {
                            nav.push(Route::ComingSoonPage { lang });
                            expanded.set(false);
                        },
                        {translates.reward}
                    }
                    button {
                        class: "cursor-pointer flex flex-row w-full h-50 justify-start items-start",
                        onclick: move |_| {
                            nav.push(Route::ComingSoonPage { lang });
                            expanded.set(false);
                        },
                        {translates.participant}
                    }
                    button {
                        class: "cursor-pointer flex flex-row w-full h-50 justify-start items-start",
                        onclick: move |_| {
                            nav.push(Route::ComingSoonPage { lang });
                            expanded.set(false);
                        },
                        {translates.deliberation_committee}
                    }
                    button {
                        class: "cursor-pointer flex flex-row w-full h-50 justify-start items-start",
                        onclick: move |_| {
                            nav.push(Route::ComingSoonPage { lang });
                            expanded.set(false);
                        },
                        {translates.data_room}
                    }
                    button {
                        class: "cursor-pointer flex flex-row w-full h-50 justify-start items-start",
                        onclick: move |_| {
                            nav.push(Route::ComingSoonPage { lang });
                            expanded.set(false);
                        },
                        {translates.activity_details}
                    }

                    button {
                        class: "cursor-pointer flex flex-row w-full h-50 justify-start items-start",
                        onclick: move |e: Event<MouseData>| {
                            onclick.call(e);
                            expanded.set(false);
                        },
                        div {
                            if email == "" {
                                "{translates.login}"
                            } else {
                                "{translates.logout}"
                            }
                        }
                    }

                    if email != "" {
                        button {
                            class: "cursor-pointer flex flex-row w-full h-50 justify-start items-start",
                            onclick: move |_| {
                                nav.push(Route::ProfilePage { lang });
                                expanded.set(false);
                            },
                            {translates.my_profile}
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ProjectDesktopHeader(
    lang: Language,
    email: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let tr: ProjectHeaderTranslate = translate(&lang);

    rsx! {
        div { class: "fixed top-0 left-0 w-screen h-80 overflow-hidden flex items-center justify-center z-100 bg-white",
            div { class: "flex flex-row w-full max-w-1300 justify-between my-25 h-30 items-center",
                Link {
                    class: "flex flex-row items-center justify-around gap-4 h-full",
                    to: Route::MainPage {
                        lang: lang.clone(),
                    },
                    icons::Logo {}
                    div { class: "font-extrabold text-base text-logo", "VOICE KOREA" }
                }
                //TODO: Add more menus
                div { class: "flex font-bold justify-center items-center text-key-gray text-15 leading-19 gap-45",
                    Link {
                        //TODO: Change Target
                        to: Route::ComingSoonPage { lang },
                        {tr.space}
                    }
                    Link {
                        //TODO: Change Target
                        to: Route::ComingSoonPage { lang },
                        {tr.reward}
                    }
                    Link {
                        //TODO: Change Target
                        to: Route::ComingSoonPage { lang },
                        {tr.participant}
                    }
                    Link {
                        //TODO: Change Target
                        to: Route::ComingSoonPage { lang },
                        {tr.deliberation_committee}
                    }
                    Link {
                        //TODO: Change Target
                        to: Route::ComingSoonPage { lang },
                        {tr.data_room}
                    }
                    Link {
                        //TODO: Change Target
                        to: Route::ComingSoonPage { lang },
                        {tr.activity_details}
                    }

                    div { class: "cursor-pointer", onclick,
                        if email == "" {
                            "{tr.login}"
                        } else {
                            "{tr.logout}"
                        }
                    }

                    if email == "" {
                        div { class: "flex flex-row w-fit h-fit justify-center items-center rounded-lg px-5 py-10 bg-white border border-key-gray",
                            "{tr.deliberation_design}"
                        }
                    } else {
                        Link {
                            class: "cursor-pointer w-28 h-28 rounded-full bg-profile-gray",
                            to: Route::ProfilePage { lang },
                        }
                    }
                }
            }
        }
    }
}

translate! {
    ProjectHeaderTranslate;

    login: {
        ko: "로그인",
        en: "Login"
    },

    logout: {
        ko: "로그아웃",
        en: "Logout"
    },

    space: {
        ko: "참여 공간",
        en: "Participation Space"
    }

    reward: {
        ko: "참여 보상",
        en: "Participation Reward"
    }

    participant: {
        ko: "참여자",
        en: "Participant"
    }

    deliberation_committee: {
        ko: "공론 위원회",
        en: "Deliberation Committee"
    }

    data_room: {
        ko: "자료실",
        en: "Data Room"
    }

    activity_details: {
        ko: "활동 내역",
        en: "Activity Details"
    }

    deliberation_design: {
        ko: "공론 조사 설계",
        en: "Deliberation Design"
    }

    my_profile: {
        ko: "나의 프로필",
        en: "My Profile"
    }
}
