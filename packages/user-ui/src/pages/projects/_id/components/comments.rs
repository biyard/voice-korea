use by_components::icons::{chat::SquareChat, links_share::Send2};
use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

use crate::pages::projects::_id::{components::comment_item::CommentItem, controller::CommentTree};

#[component]
pub fn Comment(
    lang: Language,
    comments: Vec<CommentTree>,
    send_comment: EventHandler<String>,
    send_reply: EventHandler<(i64, String)>,
    like_comment: EventHandler<i64>,
) -> Element {
    let tr: CommentTranslate = translate(&lang);
    let mut comment = use_signal(|| "".to_string());

    rsx! {
    div { class: "max-w-[1300px] flex flex-row w-full justify-center items-center mt-40 max-[500px]:px-10",
            div { class: "flex flex-col w-full justify-start items-start gap-10",
                div { class: "w-full h-24 flex flex-row justify-start items-center gap-8",
                    div { SquareChat {} }
                    p { "{comments.len()}" }
                }

                // text write area
                div { class: "w-full max-w-[1300px] min-h-48 relative border-1 border-[#B4B4B4] rounded-[8px] flex justify-start items-center pl-12 gap-8",
                    SquareChat { color: "#8095EA" }

                    //desktop
                    div { class: "block max-[500px]:!hidden",
                        input {
                            class: "w-full h-48 py-12 font-semibold text-[15px] leading-normal outline-none",
                            placeholder: "{tr.reply_box_text}",
                            value: "{comment()}",
                            oninput: move |e| comment.set(e.value().clone()),
                            onkeypress: move |e| {
                                if e.key() == Key::Enter {
                                    e.prevent_default();
                                    send_comment.call(comment());
                                    comment.set("".to_string());
                                }
                            },
                        }
                    }

                    //mobile
                    div { class: "hidden max-[500px]:!block",
                        div { class: "w-full flex flex-row justify-start items-center",
                            input {
                                class: "h-48 py-12 font-semibold text-[15px] leading-normal outline-none",
                                placeholder: "{tr.reply_box_text}",
                                value: "{comment()}",
                                oninput: move |e| comment.set(e.value().clone()),
                            }
                            button {
                                class: "absolute right-10 min-w-30 h-full flex justify-center items-center py-12 text-white rounded-r-md text-md font-md",
                                onclick: move |_| {
                                    send_comment.call(comment());
                                    comment.set("".to_string());
                                },
                                Send2{class: "[&>path]:stroke-[#B4B4B4]"}
                            }
                        }
                    }
                }

                //comments
                div { class: "w-full h-auto flex flex-col justify-center items-start mt-20",
                    for comment in comments.clone() {
                        CommentItem {
                            lang,
                            comment: comment.clone(),
                            like_comment: move |_| {
                                like_comment.call(comment.id);
                            },
                            send_reply: move |(id, reply)| {
                                send_reply.call((id, reply));
                            },
                        }
                    }
                }
            }
        }
    }
}

translate! {
    CommentTranslate;

    reply: {
        ko: "답글",
        en: "Reply"
    }

    unit: {
        ko: "개",
        en: "Unit"
    }

    reply_comment: {
        ko: "답글하기",
        en: "Reply"
    }

    reply_box_text: {
        ko: "답글 남기기...",
        en: "Leave a reply..."
    }

    anonymity: {
        ko: "익명",
        en: "Anonymity"
    }
}
