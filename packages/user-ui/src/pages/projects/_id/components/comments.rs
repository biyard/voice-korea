use by_components::icons::chat::SquareChat;
use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::deliberation_comment::DeliberationCommentSummary;

use crate::pages::projects::_id::components::comment_item::CommentItem;

#[component]
pub fn Comment(lang: Language, comments: Vec<DeliberationCommentSummary>) -> Element {
    let mut comment = use_signal(|| "".to_string());
    let tr: CommentTranslate = translate(&lang);
    let add_comment = move || {};

    rsx! {
        div { class: "max-w-[1300px] h-[48px] mb-[75px]",
            // comment counts
            div { class: "w-full h-[24px] mt-[40px] mb-[10px] flex flex-row justify-start items-center gap-[8px]",
                div {
                    SquareChat { color: "#555462" }
                }
                // TODO: connect to comment numbers
                p { "{comments.len()}" }
            }

            // text write area
            div { class: "max-w-[1300px] min-h-[48px] relative border-[1px] border-[#B4B4B4] rounded-[8px] flex justify-start items-center pl-[12px] gap-[8px]",
                SquareChat { color: "#8095EA" }
                // text input area
                textarea {
                    class: "w-full h-[48px] py-[12px] font-semibold text-[15px] leading-normal outline-none",
                    placeholder: "{tr.reply_box_text}",
                    value: "{comment}",
                    oninput: move |e| comment.set(e.value().clone()),
                    onkeypress: move |e| {
                        if e.key() == Key::Enter {
                            e.prevent_default();
                            add_comment();
                        }
                    },
                }
            }

            //comments
            div { class: "w-full h-auto flex flex-col justify-center items-start",
                for c in comments.iter() {
                    CommentItem { lang, comment: c.clone() }
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

}
