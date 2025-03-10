use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

use crate::{
    components::icons::{
        add_photo::AddPhotoIcon, attach_file::AttachFileIcon, bold::BoldIcon, code::CodeIcon,
        comment::CommentIcon, down_arrow::DownArrow, like::LikeIcon, more::MoreIcon,
        quotation::QuotationIcon, up_arrow::UpArrow,
    },
    pages::project::components::i18n::CommentTranslate,
};

#[component]
pub fn Comment(lang: Language) -> Element {
    let mut comment = use_signal(|| "".to_string());
    let mut comments = use_signal(|| Vec::<String>::new());
    let tr: CommentTranslate = translate(&lang);
    let mut add_comment = move || {
        if !comment().is_empty() {
            comments.push(comment().clone());
            comment.set("".to_string());
        }
    };

    rsx! {
        div { class: "max-w-[1300px] h-[48px] mb-[75px]",
            // comment counts
            div { class: "w-full h-[24px] mt-[40px] mb-[10px] flex flex-row justify-start items-center gap-[8px]",
                div {
                    CommentIcon { color: "#555462" }
                }
                // TODO: connect to comment numbers
                p { "{comments.len()}" }
            }

            // text write area
            div { class: "max-w-[1300px] min-h-[48px] relative border-[1px] border-[#B4B4B4] rounded-[8px] flex justify-start items-center pl-[12px] gap-[8px]",
                CommentIcon { color: "#8095EA" }
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
                    // Each comment is now handled independently by a separate component
                    CommentItem { lang, comment: c.clone() }
                }
            }
        }
    }
}

#[component]
pub fn CommentItem(lang: Language, comment: String) -> Element {
    let mut show_comments = use_signal(|| false);
    let mut show_reply_input = use_signal(|| false);
    let mut reply = use_signal(|| "".to_string());
    let mut replies = use_signal(|| Vec::<String>::new());
    let tr: CommentTranslate = translate(&lang);
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
                    p { class: "font-semibold text-[15px] justify-start", "id" }
                    // write time
                    p { class: "font-semibold text-[12px] justify-start", "time" }
                }
            }
            //more icon
            MoreIcon {}
        }
        div { class: "w-full px-[40px] mt-[14px]",
            "{comment}"
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
                                UpArrow {
                                    width: "24px",
                                    height: "24px",
                                    color: "#ffffff",
                                }
                            } else {
                                DownArrow {
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
                            CommentIcon { color: "#555462" }
                        }
                        // TODO: connect to comment numbers
                        p { "{replies.len()}" }
                    }
                    div { class: "flex flex-row justify-center gap-[8px]",
                        div {
                            LikeIcon { color: "#555462" }
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
                                QuotationIcon { color: "#555462" }
                            }
                            //code button
                            button {
                                class: "w-[24px] h-[24px] flex justify-center items-center cursor-pointer",
                                onclick: move |_| {
                                    println!("code button clicked");
                                },
                                CodeIcon { color: "#555462" }
                            }
                            //attach image button
                            button {
                                class: "w-[24px] h-[24px] flex justify-center items-center cursor-pointer",
                                onclick: move |_| {
                                    println!("attach image button clicked");
                                },
                                AddPhotoIcon { color: "#555462" }
                            }
                            //attach file button
                            button {
                                class: "w-[24px] h-[24px] flex justify-center items-center cursor-pointer",
                                onclick: move |_| {
                                    println!("attach file button clicked");
                                },
                                AttachFileIcon { color: "#555462" }
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
                            CommentIcon { color: "#8095EA" }
                        }
                    }
                }
            }

            if (*show_comments)() {
                div { class: "flex flex-col",
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
                        MoreIcon {}
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
