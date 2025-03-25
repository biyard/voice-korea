#![allow(non_snake_case)]
use crate::{
    components::{input::Input, table_row::Row},
    pages::create::controller::Controller,
};
use dioxus::prelude::*;
use dioxus_translate::Language;

#[derive(PartialEq, Props, Clone)]
pub struct StepThreeProps {
    lang: Language,
    join_the_membership: String,
    email_address: String,
    send_authentication: String,
    authentication_number: String,
    authentication_descriptions: Vec<String>,
    company_info: String,
    company_example: String,
    name_info: String,
    name_example: String,
    phone_info: String,
    phone_example: String,
    password_info: String,
    password_check_info: String,
    address_info: String,
    search_address: String,
    check_title: String,
    check_membership_descriptions: Vec<String>,
    complete_join_membership: String,

    invalid_authorization_pattern: String,
    invalid_password_pattern: String,
    incollect_email_form: String,
    input_password: String,
    incollect_two_password: String,
    already_exists_user: String,
    incollect_authentication_number: String,
    failed_store_data: String,
}

#[component]
pub fn StepThreePage(props: StepThreeProps) -> Element {
    let mut ctrl: Controller = use_context();
    rsx! {
        div { class: "flex flex-col w-full h-full items-start justify-center px-[70px] py-[40px]",
            div { class: "text-[32px] font-bold text-black pb-[30px]", "{props.join_the_membership}" }
            div { class: "flex flex-col w-full h-full items-start justify-center pb-[30px]",
                Row {
                    enable_bottom_border: false,
                    label: props.email_address,
                    height: if ctrl.get_email_address_error() { 100 as u64 } else { 70 as u64 },
                    element: rsx! {
                        div { class: "flex flex-row w-full h-full justify-start items-center",
                            div { class: "flex flex-col w-min justify-start items-start mx-[10px]",
                                Input {
                                    value: ctrl.get_email_address(),
                                    onchange: move |e| {
                                        ctrl.set_email_address(e);
                                    },
                                    border: if ctrl.get_email_address_error() { "border-[#ff0000]" } else { "border-[#E0E0E0]" },
                                }
                                if ctrl.get_email_address_error() {
                                    div { class: "mt-[10px] font-normal text-[#ff0000] text-[12px]",
                                        {props.incollect_email_form}
                                    }
                                }
                            }
                            ButtonComponent {
                                label: props.send_authentication,
                                lang: props.lang,
                                onclick: move |_| async move {
                                    ctrl.set_click_send_authentication().await;
                                },
                            }
                        }
                    },
                }
                Row {
                    enable_bottom_border: false,
                    height: 135,
                    label: props.authentication_number,
                    element: rsx! {
                        div { class: "flex flex-col w-full h-full justify-start items-start mt-[10px] ml-[10px]",
                            div { class: "pb-[5px]",
                                Input {
                                    value: ctrl.get_authentication_number(),
                                    onchange: move |e| {
                                        ctrl.set_authentication_number(e);
                                    },
                                    border: if ctrl.get_authorization_error() { "border-error" } else { "border-gray-border" },
                                }

                                if ctrl.get_authorization_error() {
                                    div { class: "mt-[10px] font-normal text-[#ff0000] text-[12px]",
                                        {props.invalid_authorization_pattern}
                                    }
                                }
                            }
                            div { class: "text-[16px] font-normal text-[#636363]", "{props.authentication_descriptions[0]}" }
                            div { class: "text-[16px] font-normal text-[#636363]", "{props.authentication_descriptions[1]}" }
                        }
                    },
                }
                Row {
                    enable_bottom_border: false,
                    label: props.password_info,
                    height: if ctrl.get_password_error() { 100 as u64 } else { 70 as u64 },
                    element: rsx! {
                        div { class: "flex flex-row w-full h-full justify-start items-center",
                            div { class: "flex flex-col w-min justify-start items-start mx-[10px]",
                                Input {
                                    value: ctrl.get_password(),
                                    input_type: "password".to_string(),
                                    border: if ctrl.get_password_error() || ctrl.get_password_check_error()
                            || ctrl.get_password_pattern_error() { "border-[#ff0000]" } else { "border-[#E0E0E0]" },
                                    onchange: move |e| {
                                        ctrl.set_password(e);
                                    },
                                }
                                if ctrl.get_password_error() {
                                    div { class: "mt-[10px] font-normal text-[#ff0000] text-[12px]", {props.input_password} }
                                } else if ctrl.get_password_pattern_error() {
                                    div { class: "mt-[10px] font-normal text-[#ff0000] text-[12px]",
                                        {props.invalid_password_pattern}
                                    }
                                }
                            }
                        }
                    },
                }
                Row {
                    enable_bottom_border: true,
                    label: props.password_check_info,
                    height: if ctrl.get_password_check_error() { 100 as u64 } else { 70 as u64 },
                    element: rsx! {
                        div { class: "flex flex-row w-full h-full justify-start items-center",
                            div { class: "flex flex-col w-min justify-start items-start mx-[10px]",
                                Input {
                                    value: ctrl.get_password_check(),
                                    input_type: "password".to_string(),
                                    border: if ctrl.get_password_error() || ctrl.get_password_check_error()
                            || ctrl.get_password_pattern_error() { "border-[#ff0000]" } else { "border-[#E0E0E0]" },
                                    onchange: move |e| {
                                        ctrl.set_password_check(e);
                                    },
                                }
                                if ctrl.get_password_check_error() {
                                    div { class: "mt-[10px] font-normal text-[#ff0000] text-[12px]",
                                        {props.incollect_two_password}
                                    }
                                }
                            }
                        }
                    },
                }
            }
            if ctrl.get_already_exists_user_error() {
                div { class: "flex flex-col w-full h-min min-w-[710px] px-[20px] py-[15px]",
                    div { class: "text-[#ff0000] font-normal text-[12px]",
                        {props.already_exists_user}
                    }
                }
            } else if ctrl.get_invalid_authkey_error() {
                div { class: "flex flex-col w-full h-min min-w-[710px] px-[20px] py-[15px]",
                    div { class: "text-[#ff0000] font-normal text-[12px]",
                        {props.incollect_authentication_number}
                    }
                }
            } else if ctrl.get_unknown_error() {
                div { class: "flex flex-col w-full h-min min-w-[710px] px-[20px] py-[15px]",
                    div { class: "text-[#ff0000] font-normal text-[12px]", {props.failed_store_data} }
                }
            }
            div { class: "flex flex-col w-full h-min min-w-[710px] border-solid border border-[#e0e0e0] px-[20px] py-[15px]",
                div { class: "text-black text-[20px] font-normal pb-[20px]", "{props.check_title}" }
                // div {
                //     class: "text-[#636363] text-[20px] font-normal pb-[5px]",
                //     "{props.check_membership_descriptions[0]}"
                // }
                div { class: "text-[#636363] text-[20px] font-normal pb-[5px]",
                    "{props.check_membership_descriptions[1]}"
                }
                div { class: "text-[#636363] text-[20px] font-normal",
                    "{props.check_membership_descriptions[2]}"
                }
            }
            button { class: "flex flex-row w-full justify-end min-w-[710px] items-end pt-[30px]",
                div {
                    onclick: move |_| async move {
                        ctrl.set_click_complete_join_membership().await;
                    },
                    class: "flex flex-row w-auto h-[60px] justify-end items-end bg-[#2168c3] px-[20px]",
                    div { class: "flex flex-row w-full h-full justify-center items-center text-[24px] font-bold text-white",
                        "{props.complete_join_membership}"
                    }
                }
            }
        }
    }
}

#[component]
pub fn ButtonComponent(
    width: Option<i64>,
    label: String,
    lang: Language,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let width = match width {
        Some(w) => format!("w-[{}px]", w),
        None => {
            if lang == Language::En {
                "w-[270px]".to_string()
            } else {
                "w-[135px]".to_string()
            }
        }
    };

    rsx! {
        button {
            class: "flex flex-col {width} h-[35px] justify-start items-start",
            onclick: move |evt| {
                onclick.call(evt);
            },
            style: "border: 1px solid; border-color: rgba(33, 104, 195, 0.5); border-radius: 5px; background-clip: padding-box; background-color: rgba(33, 104, 195, 0.04);",
            div { class: "flex flex-row justify-center items-center w-full h-full text-[#2168c3] font-normal text-[15px]",
                {label}
            }
        }
    }
}
