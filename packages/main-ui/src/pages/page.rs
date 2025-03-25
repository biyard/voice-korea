#![allow(non_snake_case)]
use super::i18n::LoginTranslate;
use crate::{components::bottom::Bottom, prelude::*};
use dioxus::prelude::*;
use dioxus_translate::translate;
use dioxus_translate::Language;

use super::*;

#[derive(PartialEq, Props, Clone)]
pub struct LoginPageProps {
    lang: Language,
}

#[derive(Props, Clone, PartialEq)]
pub struct InputEmailProps {
    ctrl: controller::Controller,
    email_message: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct InputPasswordProps {
    ctrl: controller::Controller,
    password_message: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct LoginProps {
    ctrl: controller::Controller,
    lang: Language,
    login_message: String,
    email_message: String,
    password_message: String,

    not_matched_error: String,
    not_exists_user_error: String,
    login_failed_error: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct LoginButtonProps {
    login_message: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct MemberInfoProps {
    lang: Language,
    find_email_message: String,
    reset_pw_message: String,
    create_account_message: String,
    check_title_message: String,
    check_description_1_message: String,
    check_description_2_message: String,
}

#[component]
pub fn LoginPage(props: LoginPageProps) -> Element {
    let ctrl = controller::Controller::init();
    let translates: LoginTranslate = translate(&props.lang.clone());
    let logo_path = asset!("/public/images/logo.png");

    let login_message = translates.login;
    let email_message = translates.email;
    let password_message = translates.password;
    let find_email_message = translates.find_email;
    let reset_pw_message = translates.reset_pw;
    let create_account_message = translates.create_account;
    let check_title_message = translates.check_title;
    let check_description_1_message = translates.check_description_1;
    let check_description_2_message = translates.check_description_2;

    rsx! {
        div { class: "bg-white dark:bg-black w-screen min-h-screen flex flex-col",
            div { class: "flex flex-row w-full justify-start items-center px-[30px] py-[3px]",
                div { class: "mr-[7px]",
                    img { src: logo_path, width: 42, height: 42 }
                }
                div { class: "text-[24px] font-bold text-[#2168C3]", "VOICE KOREA" }
            }
            div {
                class: "flex flex-col h-full w-full justify-start items-center",
                style: "height: calc(100vh - 48px)",
                div { class: "flex flex-col w-full h-full justify-center items-center",
                    LoginComponent {
                        ctrl,
                        lang: props.lang,
                        email_message,
                        password_message,
                        login_message,
                        not_matched_error: translates.not_matched_error,
                        not_exists_user_error: translates.not_exists_user_error,
                        login_failed_error: translates.login_failed_error,
                    }
                    MemberInfoComponent {
                        lang: props.lang.clone(),
                        find_email_message,
                        reset_pw_message,
                        create_account_message,
                        check_title_message,
                        check_description_1_message,
                        check_description_2_message,
                    }
                }
                div { class: "flex flex-col h-full w-full justify-end items-end",
                    Bottom { lang: props.lang }
                }
            }
        }
    }
}

#[component]
pub fn MemberInfoComponent(props: MemberInfoProps) -> Element {
    rsx! {
        div { class: "flex flex-col w-full h-[180px] justify-start items-start px-[50px]",
            div { class: "flex flex-row w-full h-min justify-end items-center",
                // Link {
                //     to: Route::FindEmailPage {
                //         lang: props.lang.clone(),
                //     },
                //     div {
                //         class: "flex flex-row text-black text-[20px] font-normal w-[130px] justify-center items-center",
                //         "{props.find_email_message}"
                //     }
                // }
                // div {
                //     class: "text-[#e0e0e0] text-[20px] font-normal",
                //     "|"
                // }
                Link {
                    to: Route::ResetPasswordPage {
                        lang: props.lang.clone(),
                    },
                    div { class: "flex flex-row text-black text-[20px] font-normal w-[160px] justify-center items-center",
                        "{props.reset_pw_message}"
                    }
                }
                div { class: "text-[#e0e0e0] text-[20px] font-normal", "|" }
                Link {
                    to: Route::CreatePage {
                        lang: props.lang.clone(),
                    },
                    div { class: "flex flex-row text-[#2168c3] text-[20px] font-normal w-[130px] justify-center items-center",
                        "{props.create_account_message}"
                    }
                }
            }
            div { class: "flex flex-col w-full h-[135px] border-solid border border-[#e0e0e0] px-[20px] py-[15px]",
                div { class: "text-black text-[20px] font-normal pb-[15px]",
                    "{props.check_title_message}"
                }
                div { class: "text-[#636363] text-[20px] font-normal pb-[5px]",
                    "{props.check_description_1_message}"
                }
                div { class: "text-[#636363] text-[20px] font-normal",
                    "{props.check_description_2_message}"
                }
            }
        }
    }
}

#[component]
pub fn LoginComponent(props: LoginProps) -> Element {
    let mut ctrl = props.ctrl;

    rsx! {
        div { class: "flex flex-col w-[610px] h-[530px] justify-center items-center",
            div { class: "flex flex-row w-full h-40 justify-center items-center text-[32px] font-bold text-black pb-[50px]",
                "{props.login_message}"
            }
            div { class: "flex flex-col justify-start items-start",
                div { class: "flex flex-row justify-between w-[610px] h-[100px]",
                    div { class: "flex flex-col w-[300px] h-full justify-between",
                        InputEmailComponent {
                            ctrl: props.ctrl,
                            email_message: props.email_message,
                        }
                        InputPasswordComponent {
                            ctrl: props.ctrl,
                            password_message: props.password_message,
                        }
                    }
                    button {
                        onclick: move |_| async move {
                            let _ = ctrl.login_clicked(props.lang).await;
                        },
                        LoginButton { login_message: props.login_message }
                    }
                }
                if ctrl.get_not_matched_error() {
                    div { class: "mt-[10px] text-[#ff0000] font-normal text-[12px]",
                        {props.not_matched_error}
                    }
                } else if ctrl.get_exists_error() {
                    div { class: "mt-[10px] text-[#ff0000] font-normal text-[12px]",
                        {props.not_exists_user_error}
                    }
                } else if ctrl.get_login_failed_error() {
                    div { class: "mt-[10px] text-[#ff0000] font-normal text-[12px]",
                        {props.login_failed_error}
                    }
                }
            }
        }
    }
}

#[component]
pub fn LoginButton(props: LoginButtonProps) -> Element {
    rsx! {
        div {
            class: "flex flex-row w-[300px] h-[100px] bg-[#2168c3]",
            style: "width: 300px",
            div { class: "flex flex-row w-full h-full justify-center items-center text-[24px] font-bold text-white",
                "{props.login_message}"
            }
        }
    }
}

#[component]
pub fn InputPasswordComponent(props: InputPasswordProps) -> Element {
    let mut ctrl = props.ctrl;
    rsx! {
        div { class: "flex flex-row w-[300px] h-[42px] justify-start items-start",
            input {
                class: "flex flex-row px-[10px] py-[10px] w-full h-full",
                r#type: "password",
                style: "border: 1px solid #e0e0e0; color: #8e929b;",
                placeholder: "{props.password_message}",
                oninput: move |event| ctrl.set_password(event.value()),
            }
        }
    }
}

#[component]
pub fn InputEmailComponent(props: InputEmailProps) -> Element {
    let mut ctrl = props.ctrl;
    rsx! {
        div { class: "flex flex-row w-[300px] h-[42px] justify-start items-start",
            input {
                class: "flex flex-row px-[10px] py-[10px] w-full h-full",
                r#type: "text",
                style: "border: 1px solid #e0e0e0; color: #8e929b;",
                placeholder: "{props.email_message}",
                oninput: move |event| ctrl.set_email(event.value()),
            }
        }
    }
}
