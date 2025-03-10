use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::review::ReviewSummary;

use crate::{
    components::icons::{left_arrow::LeftArrow, right_arrow::RightArrow},
    pages::main::i18n::ReviewSectionTranslate,
    utils::time::format_prev_time,
};

#[component]
pub fn ReviewSection(lang: Language, public_opinion_reviews: Vec<ReviewSummary>) -> Element {
    let tr: ReviewSectionTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-center items-center py-[100px] bg-gradient-to-b from-[#f1f3fa] to-[#a6e0d3] gap-[30px]",
            div { class: "font-bold text-[28px] leading-[32px] text-[#555462]",
                "{tr.participation_review}"
            }
            div { class: "flex flex-row w-full justify-center items-center gap-[20px]",
                div { class: "bg-[#8095ea] rounded-[8px] px-[10px] py-[8px]", LeftArrow {} }
                div { class: "flex flex-row w-full max-w-[1300px]",
                    div { class: "grid grid-cols-3 gap-[20px]",
                        for review in public_opinion_reviews {
                            Review { review }
                        }
                    }
                }
                div { class: "bg-[#8095ea] rounded-[8px] px-[10px] py-[8px]", RightArrow {} }
            }
        }
    }
}

#[component]
pub fn Review(review: ReviewSummary) -> Element {
    let prev_date = format_prev_time(review.created_at);
    rsx! {
        div { class: "flex flex-col w-full h-[240px] px-[32px] py-[40px] bg-white rounded-[12px] gap-[20px]",
            div { class: "flex flex-row gap-[8px] justify-start items-center",
                div { class: "w-[40px] h-[40px] bg-[#d9d9d9] rounded-[100px]" }
                div { class: "flex flex-col gap-[4px]",
                    div { class: "font-semibold text-[#222222] text-[15px]", "{review.name}" }
                    div { class: "font-semibold text-[#6d6d6d] text-[12px]", "{prev_date}" }
                }
            }

            div {
                class: "font-normal text-[15px] text-[#6d6d6d]",
                style: "overflow: hidden; text-overflow: ellipsis; display: -webkit-box; -webkit-line-clamp: 4; -webkit-box-orient: vertical;",
                "{review.review.clone()}"
            }
        }
    }
}
