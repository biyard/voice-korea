use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use regex::Regex;

use crate::{
    components::{input::InputBox, textarea::TextArea},
    pages::i18n::InquirySectionTranslate,
};

#[component]
pub fn InquirySection(
    lang: Language,
    send_inquiry: EventHandler<(String, String, String)>,
) -> Element {
    let tr: InquirySectionTranslate = translate(&lang);
    let blockchain_info_1 = asset!("/public/images/blockchain-info-1.png");
    let blockchain_info_2 = asset!("/public/images/blockchain-info-2.png");
    let blockchain_info_3 = asset!("/public/images/blockchain-info-3.png");

    let mut name = use_signal(|| "".to_string());
    let mut email = use_signal(|| "".to_string());
    let mut message = use_signal(|| "".to_string());

    let mut email_error = use_signal(|| false);
    let mut message_error = use_signal(|| false);
    rsx! {
        div {
            id: "inquiry",
            class: "flex flex-col w-full justify-center items-center",
            div { class: "max-[1000px]:px-15 max-[400px]:mt-200 flex flex-col w-full max-w-1080 h-full justify-center items-center gap-[50px]",
                div { class: "flex flex-col gap-30",
                    div { class: "font-bold text-[28px] leading-32 text-text-gray",
                        "{tr.inquiry_title}"
                    }
                    div { class: "font-semibold text-[15px] leading-22 text-text-gray whitespace-pre-line text-center",
                        "{tr.inquiry_description}"
                    }
                }

                div { class: "flex max-[1000px]:flex-col max-[1000px]:gap-50 flex-row w-full justify-between items-center",
                    div { class: "flex flex-col gap-32",
                        SolutionInfoComponent {
                            asset: blockchain_info_1,
                            description: "{tr.blockchain_info_1}",
                        }
                        SolutionInfoComponent {
                            asset: blockchain_info_2,
                            description: "{tr.blockchain_info_2}",
                        }
                        SolutionInfoComponent {
                            asset: blockchain_info_3,
                            description: "{tr.blockchain_info_3}",
                        }
                    }

                    div { class: "flex flex-col max-[600px]:w-full w-530 gap-30",
                        div { class: "flex flex-col w-full gap-20",
                            div { class: "flex flex-col w-full gap-10",
                                div { class: "font-semibold text-[15px] text-text-black",
                                    "{tr.name}"
                                }
                                InputBox {
                                    placeholder: "{tr.name_hint}",
                                    value: name(),
                                    onchange: move |value| {
                                        name.set(value);
                                    },
                                }
                            }

                            div { class: "flex flex-col w-full gap-10",
                                div { class: "flex flex-row gap-3 font-semibold text-[15px] items-center",
                                    div { class: "text-red-500", "*" }
                                    div { class: "text-text-black", "{tr.email}" }
                                }
                                InputBox {
                                    placeholder: "{tr.email_hint}",
                                    value: email(),
                                    onchange: move |value| {
                                        email.set(value);
                                    },
                                }

                                if email_error() {
                                    div { class: "font-normal text-sm text-error", "{tr.email_error}" }
                                }
                            }

                            div { class: "flex flex-col w-full gap-10",
                                div { class: "flex flex-row gap-3 font-semibold text-[15px] items-center",
                                    div { class: "text-red-500", "*" }
                                    div { class: "text-text-black", "{tr.message}" }
                                }
                                TextArea {
                                    placeholder: "{tr.message_hint}",
                                    value: message(),
                                    onchange: move |value| {
                                        message.set(value);
                                    },
                                }

                                if message_error() {
                                    div { class: "font-normal text-sm text-error", "{tr.message_error}" }
                                }
                            }
                        }

                        button {
                            class: "flex flex-row w-full justify-center items-center bg-button-primary rounded-xl px-16 py-12 font-semibold text-base text-white cursor-pointer",
                            onclick: move |_| {
                                let name_value = name();
                                let email_value = email();
                                let message_value = message();
                                let re = Regex::new(r"^[a-zA-Z0-9+-\_.]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$")
                                    .unwrap();
                                if !re.is_match(&email_value) {
                                    email_error.set(true);
                                } else {
                                    email_error.set(false);
                                }
                                if message_value == "" {
                                    message_error.set(true);
                                } else {
                                    message_error.set(false);
                                }
                                if !email_error() && !message_error() {
                                    send_inquiry.call((name_value, email_value, message_value));
                                    email.set("".to_string());
                                    message.set("".to_string());
                                    name.set("".to_string());
                                }
                            },
                            "{tr.inquiry}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn SolutionInfoComponent(asset: Asset, description: String) -> Element {
    rsx! {
        div { class: "flex flex-row max-[450px]:w-full w-420 h-104 rounded-xl bg-white shadow-[0px_8px_20px_rgba(148,176,214,0.25)] px-35 py-10 items-center justify-start gap-25",
            img { src: asset, width: 60, height: 60 }

            div { class: "font-bold text-15 text-text-gray leading-22", "{description}" }
        }
    }
}
