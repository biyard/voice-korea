use by_components::icons as by_components_icon;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};

mod i18n;
use crate::{
    components::{
        icons::{self, Logo},
        input::InputBox,
    },
    routes::Route,
    service::{
        popup_service::PopupService,
        user_service::{UserEvent, UserService},
    },
};
use i18n::{
    CompletePopupTranslate, GoogleLoginPopupTranslate, SeeDetailButtonTranslate,
    SignupPopupTranslate, Translate,
};

#[component]
pub fn SeeDetailButton(lang: Language) -> Element {
    let tr: SeeDetailButtonTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-row bg-[#7C8292] rounded-[4px] px-[10px] py-[3px] font-semibold text-white text-[14px]",
            "{tr.see_detail}"
        }
    }
}

#[component]
fn CustomCheckbox(lang: Language, mut checked: bool, onchange: EventHandler<bool>) -> Element {
    rsx! {
        label { class: "flex items-center cursor-pointer",
            input {
                r#type: "checkbox",
                class: "hidden",
                checked: "{checked}",
                onchange: move |_| {
                    onchange.call(!checked);
                },
            }
            div {
                class: format!(
                    "w-[24px] h-[24px] flex items-center justify-center rounded-md transition-all {}",
                    if checked { "bg-[#8095EA]" } else { "border-1 bg-white border-gray-400" },
                ),
                div { class: "text-white text-lg",
                    if checked {
                        div { "✔" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn CompletePopup(lang: Language, onclose: EventHandler<MouseEvent>) -> Element {
    let tr: CompletePopupTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col min-w-[420px] justify-center items-center gap-[35px]",
            div { class: "flex flex-col w-full justify-center items-center gap-[15px]",
                div { class: "flex flex-row w-[88px] h-[88px] justify-center items-center bg-[#7C8292] rounded-[100px]",
                    Logo { width: "47", height: "47", class: "fill-[#ffffff]" }
                }
                div { class: "flex flex-col w-full justify-center items-center font-semibold text-[16px] text-[#35343F] leading-[24px]",
                    div { "{tr.complete_message_1}" }
                    div { "{tr.complete_message_2}" }
                }
            }
            div {
                class: "cursor-pointer flex flex-row w-full h-[57px] justify-center items-center rounded-[12px] bg-[#8095EA] font-extrabold text-[18px] text-white",
                onclick: move |e: Event<MouseData>| {
                    onclose.call(e);
                },
                "{tr.start}"
            }
        }
    }
}

#[component]
pub fn SignupPopup(lang: Language, oncomplete: EventHandler<String>) -> Element {
    let tr: SignupPopupTranslate = translate(&lang);
    let mut nickname: Signal<String> = use_signal(|| "".to_string());
    let mut checked_1: Signal<bool> = use_signal(|| false);
    let mut checked_2: Signal<bool> = use_signal(|| false);
    rsx! {
        div { class: "flex flex-col min-w-[420px] justify-between items-center gap-[25px]",
            div { class: "flex flex-col w-full justify-start items-start gap-[15px]",
                div { class: "flex flex-col gap-[5px] w-full",
                    div { class: "flex flex-row w-full justify-start items-start gap-[3px]",
                        div { class: "font-bold text-[#ff0004] text-[14px]", "*" }
                        div { class: "font-bold text-[#222222] text-[14px]", "{tr.nickname}" }
                    }
                    InputBox {
                        placeholder: "{tr.nickname_hint}",
                        value: nickname(),
                        onchange: move |v: String| {
                            nickname.set(v);
                        },
                    }
                    div { class: "font-normal text-[#7C8292] text-[14px]", "{tr.nickname_warning}" }
                }
                div { class: "flex flex-col w-full justify-start items-center gap-[15px]",
                    div { class: "flex flex-row w-full gap-[10px]",
                        CustomCheckbox {
                            lang,
                            checked: checked_1(),
                            onchange: move |v| {
                                checked_1.set(v);
                            },
                        }
                        div { class: "font-medium text-[#555462] text-[16px]", "{tr.agree_1}" }
                        SeeDetailButton { lang }
                    }
                    div { class: "flex flex-row w-full gap-[10px]",
                        CustomCheckbox {
                            lang,
                            checked: checked_2(),
                            onchange: move |v| {
                                checked_2.set(v);
                            },
                        }
                        div { class: "font-medium text-[#555462] text-[16px]", "{tr.agree_2}" }
                        SeeDetailButton { lang }
                    }
                }
            }
            div {
                class: "cursor-pointer flex flex-row w-full h-[57px] justify-center items-center rounded-[12px] bg-[#8095EA] font-extrabold text-[18px] text-white",
                onclick: move |_| async move {
                    oncomplete.call(nickname());
                },
                "{tr.next}"
            }
        }
    }
}

#[component]
pub fn GoogleLoginPopup(lang: Language, onclose: EventHandler<MouseEvent>) -> Element {
    let mut user_service: UserService = use_context();
    let mut popup_service: PopupService = use_context();
    let tr: GoogleLoginPopupTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col min-w-[420px] justify-between items-center",
            div {
                class: "cursor-pointer flex flex-row w-full bg-[#8095EA] rounded-[8px] p-[8px] gap-[15px] justify-start items-center",
                onclick: move |e: Event<MouseData>| {
                    let onclose = onclose.clone();
                    async move {
                        let v: UserEvent = user_service.google_login().await;
                        match v {
                            UserEvent::Signup(_principal, _email, _, _profile_url) => {
                                popup_service
                                    .open(rsx! {
                                        SignupPopup {
                                            lang,
                                            oncomplete: move |nickname: String| async move {
                                                tracing::error!("Inside spawn async move {nickname}");
                                            },
                                        }
                                    })
                                    .with_id("signup")
                                    .with_title(tr.signup);
                            }
                            UserEvent::Login => {
                                onclose.call(e);
                            }
                            UserEvent::Logout => {
                                onclose.call(e);
                            }
                        };
                    }
                },
                div { class: "flex flex-row w-[62px] h-[62px] bg-white rounded-[8px] justify-center items-center",
                    div {
                        by_components_icon::logo::Google { size: 31 }
                    }
                }
                div { class: "flex flex-col w-full justify-start items-start gap-[3px]",
                    div { class: "text-white font-extrabold text-[16px]", "Continue with Google" }
                    div { class: "text-white font-normal text-[14px]", "Quick Sign-in" }
                }
            }

            div { class: "flex flex-row w-full justify-center items-center gap-[20px] font-semibold text-[#A3A3A3] text-[14px] mt-[45px]",
                div { "{tr.privacy}" }
                div { "{tr.usage}" }
            }
        }
    }
}

#[component]
pub fn Header(lang: Language) -> Element {
    let translates: Translate = translate(&lang);
    let mut popup_service: PopupService = use_context();

    let onclick = move |_| {
        tracing::debug!("signup button clicked");
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
    };

    rsx! {
        header { class: "flex justify-between my-6.5 h-[30px]",
            Link {
                class: "flex flex-row items-center justify-around gap-1 h-full",
                to: Route::MainPage {
                    lang: lang.clone(),
                },
                icons::Logo {}
                div { class: "font-extrabold text-base text-[#34333e]", "VOICE KOREA" }
            }
            //TODO: Add more menus
            div { class: "flex font-bold justify-center items-center text-[#35343f] text-[15px] leading-[18.75px] gap-[45px]",
                Link {
                    //TODO: Change Target
                    to: Route::MainPage {
                        lang: lang.clone(),
                    },
                    "{translates.service}"
                }
                Link {
                    //TODO: Change Target
                    to: Route::MainPage {
                        lang: lang.clone(),
                    },
                    "{translates.project}"
                }
                Link {
                    //TODO: Change Target
                    to: Route::MainPage {
                        lang: lang.clone(),
                    },
                    "{translates.organization}"
                }
                Link {
                    //TODO: Change Target
                    to: Route::MainPage {
                        lang: lang.clone(),
                    },
                    "{translates.plan}"
                }
                Link {
                    //TODO: Change Target
                    to: Route::MainPage {
                        lang: lang.clone(),
                    },
                    "{translates.contact}"
                }
                Link {
                    //TODO: Change Target
                    to: Route::MainPage {
                        lang: lang.clone(),
                    },
                    "{translates.guide}"
                }
                div { class: "cursor-pointer", onclick, "{translates.login}" }
                div { class: "flex flex-row w-[105px] h-[30px] justify-center items-center rounded-lg px-[5px] py-[10px] bg-white border border-[#35343f]",
                    "{translates.public_opinion_design}"
                }
            }
        }
    }
}
