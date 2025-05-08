use bdk::prelude::*;
use by_components::icons::{links_share::Send2, validations::Clear};

use super::super::Chat;
use crate::{
    components::icons::Logo,
    utils::time::{current_date, format_timestamp_to_ampm},
};

#[component]
pub fn ConversationSidebar(
    messages: Vec<Chat>,
    show_conversation: bool,
    hide_conversation: EventHandler<MouseEvent>,

    onsend: EventHandler<String>,
) -> Element {
    let mut text = use_signal(|| "".to_string());

    rsx! {
        div {
            class: "fixed top-0 right-0 h-full w-[320px] bg-white shadow-lg z-50 transition-transform duration-500 transform aria-active:translate-x-0 translate-x-full rounded-l-lg",
            "aria-active": show_conversation,
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "flex flex-row w-full justify-between items-center px-20 py-18 bg-netural-9",
                    div { class: "flex flex-row w-fit justify-start items-center gap-10",
                        Logo { width: "30", height: "20", class: "fill-third" }
                        div { class: "font-semibold text-white text-sm/17", "Chat" }
                    }
                    button {
                        onclick: move |e: Event<MouseData>| {
                            hide_conversation.call(e);
                        },
                        Clear {
                            width: "24",
                            height: "24",
                            fill: "#bfc8d9",
                            class: "[&>path]:stroke-discussion-border-gray",
                        }
                    }
                }
                div { class: "relative flex flex-col w-full h-[calc(100dvh-130px)] justify-start items-start px-10 py-20 bg-key-gray gap-20",
                    div { class: "flex-1 w-full overflow-y-auto flex flex-col justify-start items-start gap-20",
                        div { class: "flex flex-col w-full h-fit justify-start items-start py-20 px-10 gap-20",
                            div { class: "flex flex-row w-full justify-center items-center font-normal text-[13px] text-third",
                                {current_date()}
                            }
                            for message in messages {
                                Message { message }
                            }
                        }
                    }
                    div { class: "fixed bottom-0 left-0 w-full px-20 py-15 bg-netural-9 gap-10",
                        div { class: "flex flex-row w-full justify-start items-start",
                            input {
                                class: "flex flex-row w-full justify-start items-start bg-transparent focus:outline-none font-medium text-sm/18 placeholder:text-text-gray text-white",
                                r#type: "text",
                                placeholder: "Type message here",
                                value: text(),
                                oninput: move |e| {
                                    text.set(e.value());
                                },
                                onkeypress: move |e: KeyboardEvent| {
                                    let key = e.key();
                                    if key == Key::Enter {
                                        onsend.call(text());
                                        text.set("".to_string());
                                        e.prevent_default();
                                        e.stop_propagation();
                                    }
                                },
                            }
                        }

                        div { class: "flex flex-row w-full justify-end items-end",
                            button {
                                class: "flex flex-row w-fit h-fit justify-center items-center p-8 rounded-sm border border-text-gray",
                                onclick: move |_| {
                                    onsend.call(text());
                                    text.set("".to_string());
                                },
                                Send2 {
                                    class: "[&>path]:stroke-white",
                                    width: "10",
                                    height: "10",
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Message(message: Chat) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-5",
            div { class: "flex flex-row w-full justify-start items-center gap-5",
                div { class: "flex flex-row w-30 h-30 justify-center items-center rounded-full bg-text-gray",
                    Logo { class: "fill-white", width: "20", height: "11" }
                }
                div { class: "font-medium text-sm text-third", {message.email} }
                div { class: "font-normal text-xs text-third",
                    {format_timestamp_to_ampm(message.timestamp_ms as i64)}
                }
            }

            div { class: "min-w-200 max-w-250 h-fit p-10 bg-text-gray rounded-3xl font-medium text-white text-sm whitespace-normal break-words",
                {message.text}
            }
        }
    }
}
