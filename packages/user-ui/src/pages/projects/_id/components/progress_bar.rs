#![allow(non_snake_case, dead_code, unused_variables)]
use dioxus::prelude::*;

#[component]
pub fn ProgressBar(
    watched_pages: u32,
    total_pages: u32,
    process_data: Option<fn(String)>,
) -> Element {
    // TODO(web): connect to page data
    // page progress time calculate
    let page_progress = if total_pages == 0 {
        0
    } else {
        (watched_pages * 100 / total_pages).min(100)
    };

    // progress format
    let formatted_text = format!(
        "{}% ({} / {} 페이지)",
        page_progress, watched_pages, total_pages,
    );

    rsx!(
        div { class: "w-full flex flex-row items-center gap-[20px]",
            div { class: "w-full flex justify-start items-center bg-gray-300 h-1 rounded-[100px]",
                div {
                    class: "bg-green-500 h-2 transition-all duration-500 rounded-[100px]",
                    style: format!("width: {}%;", page_progress),
                }
            }
            div { class: "w-full text-[14px]", "{formatted_text}" }
        }
    )
}
