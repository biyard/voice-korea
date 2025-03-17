use by_components::icons::emoji::ThumbsUp;
use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

use crate::{
    pages::projects::_id::{components::comments::CommentTranslate, controller::CommentTree},
    utils::time::format_prev_time,
};

#[component]
pub fn CommentItem(
    lang: Language,
    comment: CommentTree,
    like_comment: EventHandler<MouseEvent>,
) -> Element {
    let prev_time = format_prev_time(comment.created_at);
    let tr: CommentTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",

            div { class: "flex flex-col w-full justify-start items-start gap-[14px]",

                // Profile Section
                div { class: "flex flex-row w-full justify-start items-center gap-[8px]",
                    div { class: "w-[40px] h-[40px] bg-[#D9D9D9] rounded-[100px]" }
                    div { class: "flex flex-col w-full justify-start items-start gap-[4px]",
                        div { class: "font-semibold text-[15px] text-[#222222] leading-[19px]",
                            "{tr.anonymity}"
                        }
                        div { class: "font-semibold text-[12px] text-[#6D6D6D] leading-[15px]",
                            "{prev_time}"
                        }
                    }
                }

                div { class: "flex flex-col w-full justify-start items-start gap-[20px] px-[40px]",
                    div { class: "font-medium text-[16px] text-[#222222]", {comment.comment} }
                    div { class: "flex flex-row w-full justify-end items-end",
                        div {
                            class: format!(
                                "{} flex flex-row justify-center gap-[8px]",
                                if comment.liked { "cursor-not-allowed" } else { "cursor-pointer" },
                            ),
                            onclick: move |e: Event<MouseData>| {
                                if !comment.liked {
                                    like_comment.call(e);
                                }
                            },
                            div {
                                ThumbsUp { fill: if comment.liked { "red" } else { "none" } }
                            }
                            p { "{comment.likes}" }
                        }
                    }
                }
            }

            div { class: "flex flex-row w-full h-[1px] justify-start items-start bg-[#EEEEEE] my-[20px]" }
        }
    }
}
