use bdk::prelude::*;

use dioxus_translate::{translate, Language};

use crate::{
    components::{
        header::GoogleLoginPopup,
        icons::{self},
    },
    routes::Route,
    service::{popup_service::PopupService, user_service::UserService},
};

#[component]
pub fn MainHeader(lang: Language) -> Element {
    let translates: HeaderTranslate = translate(&lang);
    let user_service: UserService = use_context();
    let mut popup_service: PopupService = use_context();

    let email = (user_service.email)();

    let onclick = {
        let email = email.clone();
        move |_| {
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
                .with_title(translates.login);
        }
    };

    rsx! {
        div { class: "fixed top-0 left-0 backdrop-blur-[20px] w-screen h-80 overflow-hidden flex items-center justify-center z-100",
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
                    A { lang, href: "/#service", {translates.service} }
                    A { lang, href: "/#project", {translates.project} }
                    A { lang, href: "/#institution", {translates.organization} }
                    A { lang, href: "/#price", {translates.plan} }
                    A { lang, href: "/#inquiry", {translates.contact} }
                    A { lang, href: "/#footer", {translates.guide} }

                    div { class: "cursor-pointer", onclick,
                        if email == "" {
                            "{translates.login}"
                        } else {
                            "{translates.logout}"
                        }
                    }

                    if email == "" {
                        div { class: "flex flex-row w-fit h-fit justify-center items-center rounded-lg px-5 py-10 bg-white border border-key-gray",
                            "{translates.public_opinion_design}"
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

#[component]
pub fn A(children: Element, lang: Language, href: String) -> Element {
    let current_path: Route = use_route();
    let is_home = matches!(current_path, Route::MainPage { .. });

    rsx! {
        if is_home {
            a { class: "cursor-pointer", href, {children} }
        } else {
            Link { class: "cursor-pointer", to: Route::MainPage { lang }, {children} }
        }
    }
}

translate! {
    HeaderTranslate;

    service: {
        ko: "서비스 소개",
        en: "Main Page"
    },

    organization: {
        ko: "정책 결정 기관",
        en: "Policy Making organization"
    },
    project: {
        ko: "프로젝트",
        en: "Project"
    },

    login: {
        ko: "로그인",
        en: "Login"
    },

    logout: {
        ko: "로그아웃",
        en: "Logout"
    },

    plan: {
        ko: "플랜",
        en: "Plan"
    },

    contact: {
        ko: "문의하기",
        en: "Contact"
    },

    guide: {
        ko: "가이드",
        en: "Guide"
    },

    public_opinion_design: {
        ko: "공론 조사 설계",
        en: "Public Opinion Design"
    }
}
