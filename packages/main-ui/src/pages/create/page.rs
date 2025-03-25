#![allow(non_snake_case)]
use super::step_four::StepFourPage;
use super::step_three::StepThreePage;
use crate::prelude::*;
use dioxus::prelude::*;

use super::controller;
use super::i18n::CreateTranslate;
use dioxus_translate::translate;
use dioxus_translate::Language;

#[derive(PartialEq, Props, Clone)]
pub struct CreatePageProps {
    lang: Language,
}

#[derive(PartialEq, Props, Clone)]
pub struct StepOneProps {
    lang: Language,
}

#[component]
pub fn CreatePage(props: CreatePageProps) -> Element {
    let mut ctrl = controller::Controller::init();
    let translates: CreateTranslate = translate(&props.lang.clone());
    let num_step = 2;

    rsx! {
        div {
            class: "flex flex-col w-full h-full px-[120px] pt-[55px]",
            style: "background-color: white",
            div { class: "flex flex-row w-full px-[10px] pb-[40px]",
                Link {
                    to: Route::LoginPage {
                        lang: props.lang.clone(),
                    },
                    img {
                        src: asset!("/public/images/close.png"),
                        alt: "Close Button",
                    }
                }
                div { class: "flex flex-row items-center justify-center w-min pl-[40px]",
                    for i in 0..num_step {
                        if i == ctrl.get_step() {
                            div {
                                onclick: move |_| {
                                    ctrl.set_step(i);
                                },
                                img {
                                    src: asset!("/public/images/current-step.png"),
                                    alt: "Current Step",
                                }
                            }
                        } else if i < ctrl.get_step() {
                            div {
                                onclick: move |_| {
                                    ctrl.set_step(i);
                                },
                                img {
                                    src: asset!("/public/images/prev-step.png"),
                                    alt: "Prev Step",
                                }
                            }
                        } else {
                            div {
                                img {
                                    src: asset!("/public/images/not-current-step.png"),
                                    alt: "Next Step",
                                }
                            }
                        }

                        if i != num_step - 1 {
                            if i < ctrl.get_step() {
                                div { class: "flex items-center justify-center w-[80px] h-[1px] bg-[#2168c3]" }
                            } else {
                                div { class: "flex items-center justify-center w-[80px] h-[1px] bg-[#e0e0e0]" }
                            }
                        }
                    }
                }
            }
            // if ctrl.get_step() == 0 {
            //     StepOnePage {
            //         ctrl: ctrl,
            //         lang: props.lang,
            //         authorization: translates.authorization,
            //         individual: translates.individual,
            //         company: translates.company,
            //         individual_description: translates.individual_description,
            //         phone: translates.phone,
            //         phone_description: translates.phone_description,
            //         check_title: translates.check_title,
            //         check_descriptions: vec![translates.check_description_1, translates.check_description_2, translates.check_description_3],
            //         company_name: translates.company_name,
            //         business_register_number: translates.business_register_number,
            //         company_name_example: translates.company_name_example,
            //         business_register_number_example: translates.business_register_number_example,
            //         next: translates.next,
            //     }
            // } else if ctrl.get_step() == 1 {
            //     StepTwoPage {
            //         ctrl: ctrl,
            //         lang: props.lang,
            //         agree_terms: translates.agree_terms,
            //         agree_membership_terms: translates.agree_membership_terms,
            //         agree_privacy_policy: translates.agree_privacy_policy,
            //         entrust_personal_information: translates.entrust_personal_information,
            //         essential: translates.essential,
            //     }
            // }
            if ctrl.get_step() == 0 {
                StepThreePage {
                    lang: props.lang,
                    join_the_membership: translates.join_the_membership,
                    email_address: translates.email_address,
                    send_authentication: translates.send_authentication,
                    authentication_number: translates.authentication_number,
                    authentication_descriptions: vec![
                        translates.authentication_description_1.to_string(),
                        translates.authentication_description_2.to_string(),
                    ],
                    company_info: translates.company_info,
                    company_example: translates.company_example,
                    name_info: translates.name_info,
                    name_example: translates.name_example,
                    phone_info: translates.phone_info,
                    phone_example: translates.phone_example,
                    password_info: translates.password,
                    password_check_info: translates.password_check,
                    address_info: translates.address_info,
                    search_address: translates.search_address,
                    check_title: translates.check_title,
                    check_membership_descriptions: vec![
                        translates.check_membership_description_1.to_string(),
                        translates.check_membership_description_2.to_string(),
                        translates.check_membership_description_3.to_string(),
                    ],
                    complete_join_membership: translates.complete_join_membership,

                    invalid_authorization_pattern: translates.invalid_authorization_pattern,
                    invalid_password_pattern: translates.invalid_password_pattern,
                    incollect_email_form: translates.incollect_email_form,
                    input_password: translates.input_password,
                    incollect_two_password: translates.incollect_two_password,
                    already_exists_user: translates.already_exists_user,
                    incollect_authentication_number: translates.incollect_authentication_number,
                    failed_store_data: translates.failed_store_data,
                }
            } else {
                StepFourPage {
                    lang: props.lang,
                    complete_join_membership_info: translates.complete_join_membership_info,
                    email_address: translates.email_address,
                    name_info: translates.name_info,
                    phone_info: translates.phone_info,
                    company_name_info: translates.company_name_info,
                    complete: translates.complete,
                    company_name_example: translates.company_name_example,
                }
            }
        }
    }
}
