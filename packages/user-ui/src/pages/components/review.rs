use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::comment::CommentSummary;

use crate::{
    components::icons::{left_arrow::LeftArrow, right_arrow::RightArrow},
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
        div {
            id: "review",
            class: "flex flex-col w-full justify-center items-center",
            div { class: "flex flex-col w-full justify-center items-center py-100 bg-gradient-to-b from-[#f1f3fa] to-[#a6e0d3] gap-30",
                div { class: "font-bold text-[28px] leading-32 text-text-gray",
                    "{tr.participation_review}"
                }
                div { class: "flex flex-row w-full justify-center items-center gap-20",
                    div { class: "bg-button-primary rounded-lg px-10 py-8",
                        div {
                            class: format!(
                                "w-24 h-24 {}",
                                if page == 1 { "cursor-not-allowed" } else { "cursor-pointer" },
                            ),
                            onclick: move |_| {
                                if page != 1 {
                                    set_page.call((page - 1) as i64);
                                }
                            },
                            LeftArrow { stroke: "white" }
                        }
                    }
                    div { class: "flex flex-row w-full max-w-1300",
                        div { class: "grid max-[600px]:grid-cols-1 max-[1100px]:grid-cols-2 grid-cols-3 w-full gap-20",
                            for comment in comments {
                                ReviewItem { lang, comment }
                            }
                        }
                    }
                    div { class: "bg-button-primary rounded-lg px-10 py-8",
                        div {
                            class: format!(
                                "w-24 h-24 {}",
                                if page >= total_pages { "cursor-not-allowed" } else { "cursor-pointer" },
                            ),
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
}

#[component]
pub fn ReviewItem(lang: Language, comment: CommentSummary) -> Element {
    let tr: ReviewSectionTranslate = translate(&lang);
    let prev_date = format_prev_time(comment.created_at);

    rsx! {
        div { class: "flex flex-col w-full h-240 px-32 py-40 bg-white rounded-xl gap-20",
            div { class: "flex flex-row gap-8 justify-start items-center",
                div { class: "w-40 h-40 bg-profile-gray rounded-[100px]" }
                div { class: "flex flex-col gap-4",
                    div { class: "font-semibold text-text-black text-[15px]", "{tr.anonymity}" }
                    div { class: "font-semibold text-review-gray text-xs", "{prev_date}" }
                }
            }

            div {
                class: "font-normal text-[15px] text-review-gray",
                style: "overflow: hidden; text-overflow: ellipsis; display: -webkit-box; -webkit-line-clamp: 4; -webkit-box-orient: vertical;",
                "{comment.comment.clone()}"
            }
        }
    }
}
