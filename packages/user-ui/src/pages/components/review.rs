use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::comment::CommentSummary;

use crate::{
    components::{
        button::Button,
        icons::{left_arrow::LeftArrow, right_arrow::RightArrow},
    },
    pages::i18n::ReviewSectionTranslate,
    utils::time::format_prev_time,
};

#[component]
pub fn ReviewSection(
    lang: Language,
    comments: Vec<CommentSummary>,
    page: usize,
    total_pages: usize,
    set_page: EventHandler<i64>,
) -> Element {
    let tr: ReviewSectionTranslate = translate(&lang);

    tracing::debug!("total page: {} page: {}", total_pages, page);

    rsx! {
        section {
            id: "review",
            class: "flex flex-col w-full justify-center items-center",
            div { class: "flex flex-col w-full bg-gradient-to-b from-[#f1f3fa] to-[#a6e0d3]/30 gap-30 px-20 desktop:px-0 py-60 desktop:py-100",
                div { class: "font-bold text-[28px] leading-32 text-text-gray self-center",
                    "{tr.participation_review}"
                }
                div { class: "flex flex-row w-full justify-center items-center gap-20 max-w-1300 self-center",
                    Button {
                        class: "rounded-lg px-6 py-8",
                        disabled: page == 1,
                        onclick: move |_| {
                            if page != 1 {
                                set_page.call((page - 1) as i64);
                            }
                        },
                        LeftArrow { stroke: "white" }
                    }
                    div { class: "flex flex-row w-full",
                        div { class: "w-full gap-20 grid grid-cols-1 tablet:grid-cols-2 desktop:grid-cols-3
                        [&>:nth-child(n+2)]:hidden tablet:[&>:nth-child(n+2)]:block tablet:[&>:nth-child(n+3)]:hidden desktop:[&>*]:!block",
                            for comment in comments {
                                ReviewItem { lang, comment }
                            }
                        }
                    }
                    Button {
                        class: "rounded-lg px-6 py-8",
                        disabled: page >= total_pages,
                        onclick: move |_| {
                            if page < total_pages {
                                set_page.call((page + 1) as i64);
                            }
                        },
                        RightArrow {}
                    }
                }
            }
        }
    }
}

#[component]
pub fn ReviewItem(lang: Language, comment: CommentSummary) -> Element {
    let tr: ReviewSectionTranslate = translate(&lang);
    let prev_date = format_prev_time(comment.created_at);

    rsx! {
        div { class: "flex flex-col w-full px-32 py-40 min-h-200 bg-white rounded-xl hover:shadow-xl",
            div { class: "flex flex-row gap-8 justify-start items-center mb-20",
                div { class: "w-40 h-40 bg-profile-gray rounded-[100px]" }
                div { class: "flex flex-col gap-4",
                    div { class: "font-semibold text-text-black text-[15px]", "{tr.anonymity}" }
                    div { class: "font-semibold text-review-gray text-xs", "{prev_date}" }
                }
            }

            div {
                class: "font-normal text-[15px] text-review-gray min-h-80 desktop:min-h-100",
                style: "overflow: hidden; text-overflow: ellipsis; display: -webkit-box; -webkit-line-clamp: 4; -webkit-box-orient: vertical;",
                "{comment.comment.clone()}"
            }
        }
    }
}
