use by_components::icons::{
    arrows::{ChevronDown, ChevronUp},
    chat::{SquareChat, SquareMark},
    emoji::ThumbsUp,
    internet_script::Code,
    links_share::Link2,
    photos::AddPhoto,
};
use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::deliberation_comment::DeliberationCommentSummary;

use crate::components::icons::{bold::BoldIcon, more::MoreIcon};

#[component]
pub fn CommentItem(lang: Language, comment: DeliberationCommentSummary) -> Element {
    let mut show_comments = use_signal(|| false);
    let mut show_reply_input = use_signal(|| false);
    let mut reply = use_signal(|| "".to_string());
    let mut replies = use_signal(|| Vec::<String>::new());
    let tr: CommentItemTranslate = translate(&lang);
    let mut is_open1: Signal<bool> = use_signal(|| false);
    let mut is_open2: Signal<bool> = use_signal(|| false);
    let mut show_modal: Signal<bool> = use_signal(|| false);
    let mut submit_reply = move || {
        if !reply().is_empty() {
            replies.push(reply().clone());
            reply.set("".to_string());
        }
    };

    rsx! {
        div { class: "w-full flex justify-between items-center mt-[20px]",
            //user
            div { class: "min-w-[180px] w-[40px] flex flex-row justify-start gap-[8px]",
                img { class: "w-[40px] h-[40px] bg-[#D9D9D9] rounded-full" }
                div { class: "flex flex-col justify-start",
                    //user name
                    p { class: "font-semibold text-[15px] justify-start", "{comment.id}" }
                    // write time
                    p { class: "font-semibold text-[12px] justify-start", "{comment.created_at}" }
                }
            }
            //more icon

            div { class: "relative",
                button {
                    class: "cursor-pointer",
                    onclick: move |_| is_open1.set(!is_open1()),
                    MoreIcon {}
                }
                if is_open1() {
                    div { class: "absolute right-0 ml-2 w-40 bg-white rounded-[8px] shadow-xl",
                        // TODO(web): make a function
                        button { class: "block w-full text-left rounded-[8px] px-4 py-2 hover:bg-gray-100 cursor-pointer",
                            "{tr.edit_button}"
                        }
                        // TODO(web): make a function
                        button {
                            class: "block w-full text-left rounded-[8px] px-4 py-2 hover:bg-gray-100 cursor-pointer",
                            onclick: move |_| show_modal.set(true),
                            "{tr.report_button}"
                        }
                        if show_modal() {
                            div {
                                onclick: move |_| show_modal.set(false),
                                class: "fixed inset-0 bg-opacity-50 flex justify-center items-center bg-black/25 z-[101]",
                                div {
                                    class: "w-[600px] h-[204px] px-[25px] py-[30px] flex flex-col bg-white p-6 rounded-lg shadow-2xl gap-[40px]",
                                    onclick: move |e| e.stop_propagation(),
                                    div { class: "gap-[4px] text-[20px] font-semibold",
                                        "{tr.report_title}"
                                        div { class: "text-[14px]",
                                            "{tr.report_text1}"
                                            br {}
                                            "{tr.report_text2}"
                                        }
                                    }
                                    div { class: "flex justify-start gap-[20px]",
                                        button {
                                            class: "w-[84px] h-[40px] flex justify-center items-center bg-[#8095EA] text-white rounded-[4px]",
                                            onclick: move |_| {
                                                show_modal.set(false);
                                            },
                                            "{tr.report_button}"
                                        }
                                        button {
                                            class: "w-[56px] h-[40px] flex justify-center items-center",
                                            onclick: move |_| show_modal.set(false),
                                            "{tr.cancel_button}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        div { class: "w-full px-[40px] mt-[14px]",
            "{comment.comment}"
            div { class: "flex flex-row justify-between",
                div { class: "flex flex-row mt-[20px] gap-[40px]",

                    // Reply section
                    button { onclick: move |_| show_comments.set(!show_comments()),
                        div { class: if (*show_comments)() { "min-w-[121px] h-[40px] flex flex-row gap-[8px] justify-center items-center cursor-pointer text-[#ffffff] bg-[#8095EA] rounded-[8px]" } else { "flex flex-row gap-[8px] cursor-pointer text-[#2A60D3]" },
                            div { class: "flex flex-row gap-[4px]",
                                p { "{tr.reply_text}" }
                                p { "{replies.len()}" }
                            }
                            if (*show_comments)() {
                                ChevronUp {
                                    width: "24px",
                                    height: "24px",
                                    color: "#ffffff",
                                }
                            } else {
                                ChevronDown {
                                    width: "24px",
                                    height: "24px",
                                    color: "#2A60D3",
                                }
                            }
                        }
                    }
                    //write reply button
                    button {
                        onclick: move |_| show_reply_input.set(!show_reply_input()),
                        class: "cursor-pointer",
                        "{tr.reply_box_text}"
                    }
                }
                div { class: "flex flex-row mt-[20px] gap-[40px]",
                    div { class: "flex flex-row justify-center gap-[8px]",
                        div {
                            SquareChat { color: "#555462" }
                        }
                        // TODO: connect to comment numbers
                        p { "{comment.replies}" }
                    }
                    div { class: "flex flex-row justify-center gap-[8px]",
                        div {
                            ThumbsUp { color: "#555462" }
                        }
                        // TODO: connect to comment numbers
                        p { "number" }
                    }
                }
            }

            // Display reply input area
            if (*show_reply_input)() {
                div { class: "w-full min-h-[97px] mt-[23px] px-[12px] py-[14px] flex flex-col border border-[#ddd] rounded-lg",
                    textarea {
                        class: "w-full min-h-[22px] outline-none",
                        placeholder: "{tr.reply_box_text}",
                        value: "{reply}",
                        oninput: move |e| reply.set(e.value().clone()),
                    }
                    div { class: "w-full min-h-[40px] flex justify-between",
                        div { class: "w-full min-h-[20px] flex flex-row justify-start items-end gap-[20px]",
                            //quotation button
                            button {
                                class: "w-[24px] h-[24px] flex justify-center items-center cursor-pointer",
                                onclick: move |_| {
                                    println!("quotation button clicked");
                                },
                                SquareMark { color: "#555462" }
                            }
                            //code button
                            button {
                                class: "w-[24px] h-[24px] flex justify-center items-center cursor-pointer",
                                onclick: move |_| {
                                    println!("code button clicked");
                                },
                                Code { color: "#555462" }
                            }
                            //attach image button
                            button {
                                class: "w-[24px] h-[24px] flex justify-center items-center cursor-pointer",
                                onclick: move |_| {
                                    println!("attach image button clicked");
                                },
                                AddPhoto { color: "#555462" }
                            }
                            //attach file button
                            button {
                                class: "w-[24px] h-[24px] flex justify-center items-center cursor-pointer",
                                onclick: move |_| {
                                    println!("attach file button clicked");
                                },
                                Link2 { color: "#555462" }
                            }
                            //font style bold button
                            button {
                                class: "w-[24px] h-[24px] flex justify-center items-center cursor-pointer",
                                onclick: move |_| {
                                    println!("font style bold button clicked");
                                },
                                BoldIcon { color: "#555462" }
                            }
                        }
                        // comment summit button
                        button {
                            class: "w-[40px] h-[40px] flex justify-center items-center bg-[#EBEFF5] rounded-full cursor-pointer",
                            onclick: move |_| submit_reply(),
                            SquareChat { color: "#8095EA" }
                        }
                    }
                }
            }

            if (*show_comments)() {
                div { class: "w-full flex flex-col",
                    div { class: "flex flex-row justify-between items-center",
                        div { class: "min-w-[180px] w-[40px] mt-[32px] flex flex-row justify-start gap-[8px]",
                            //user
                            img { class: "w-[40px] h-[40px] bg-[#D9D9D9] rounded-full" }
                            div { class: "flex flex-row justify-start items-center gap-[20px]",
                                //user name
                                p { class: "font-semibold text-[15px] justify-start",
                                    "id"
                                }
                                // write time
                                p { class: "font-semibold text-[12px] justify-start",
                                    "time"
                                }
                            }
                        }
                        //more icon
                        div { class: "relative cursor-pointer",
                            button {
                                class: "cursor-pointer",
                                onclick: move |_| is_open2.set(!is_open2()),
                                MoreIcon {}
                            }
                            if is_open2() {
                                div { class: "absolute right-0 ml-2 w-40 bg-white rounded-[8px] shadow-xl",
                                    // TODO(web): make a function
                                    button { class: "block w-full text-left rounded-[8px] px-4 py-2 hover:bg-gray-100 cursor-pointer",
                                        "수정하기"
                                    }
                                    // TODO(web): make a function
                                    button { class: "block w-full text-left rounded-[8px] px-4 py-2 hover:bg-gray-100 cursor-pointer",
                                        "신고하기"
                                    }
                                }
                            }
                        }
                    }
                    //comment data
                    div { class: "ml-[32px] mt-[10px]",
                        for r in replies.iter() {
                            p { "{r}" }
                        }
                    }
                }
            }
        }
        hr { class: "w-full h-[1px] mt-[20px] border-[#eee]" }
    }
}

translate! {
    CommentItemTranslate;

    reply_box_text: {
        ko: "답글 남기기",
        en: "Leave a reply"
    }

    reply_text: {
        ko: "답글",
        en: "Reply"
    }

    edit_button:{
        ko: "수정하기",
        en: "Edit"
}

    report_title:{
        ko: "신고하시겠습니까?",
        en: "Would you like to report it?"
    }

    report_text1:{
        ko: "해당 댓글이 부적절하거나, 스팸으로 간주될 경우, 신고를 통해 댓글 삭제 신청이 가능합니다. ",
        en: "If the comment is deemed inappropriate or spam, you can request for the comment to be deleted by reporting it. "
}

    report_text2:{
        ko: "전체 참여자의 5%가 신고 시, 자동으로 해당 댓글이 삭제 됩니다.",
        en: "If 5% of all participants report it, the comment will be automatically deleted."
}

    report_button:{
        ko: "신고하기",
        en: "Report"
}

    cancel_button:{
        ko: "취소",
        en: "Cancel"
}
}
