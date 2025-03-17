use by_components::icons::chat::SquareChat;
use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

use crate::pages::projects::_id::{components::comment_item::CommentItem, controller::CommentTree};

#[component]
pub fn Comment(
    lang: Language,
    comments: Vec<CommentTree>,
    send_comment: EventHandler<String>,
    like_comment: EventHandler<i64>,
) -> Element {
    let tr: CommentTranslate = translate(&lang);
    let mut comment = use_signal(|| "".to_string());

    rsx! {
        div { class: "max-w-[1300px] flex flex-row w-full justify-center items-center mt-[40px]",
            div { class: "flex flex-col w-full justify-start items-start gap-[20px]",
                div { class: "flex flex-col w-full justify-start items-start gap-[10px]",
                    div { class: "w-full h-[24px] flex flex-row justify-start items-center gap-[8px]",
                        div { SquareChat {} }
                        p { "{comments.len()}" }
                    }

                    // text write area
                    div { class: "max-w-[1300px] min-h-[48px] w-full relative border-[1px] border-[#B4B4B4] rounded-[8px] flex justify-start items-center pl-[12px] gap-[8px]",
                        SquareChat { color: "#8095EA" }
                        // text input area
                        input {
                            class: "w-full h-[48px] py-[12px] font-semibold text-[15px] leading-normal outline-none",
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

                    //comments
                    div { class: "w-full h-auto flex flex-col justify-center items-start mt-[20px]",
                        for comment in comments.clone() {
                            CommentItem {
                                lang,
                                comment: comment.clone(),
                                like_comment: move |_| {
                                    like_comment.call(comment.id);
                                },
                            }
                        }
                    }
                }
            }
        }
    }
}

translate! {
    CommentTranslate;

    reply_box_text: {
        ko: "답글 남기기",
        en: "Leave a reply"
    }

    anonymity: {
        ko: "익명",
        en: "Anonymity"
    }
}
