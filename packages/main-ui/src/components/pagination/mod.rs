#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::components::icons::{ArrowLeft, ArrowRight};

#[component]
pub fn Pagination(
    total_page: usize,
    current_page: usize,
    onclick: EventHandler<usize>,
    #[props(default = 10)] size: usize,
    children: Element,
) -> Element {
    if total_page == 0 {
        return rsx! {
            div { class: "w-full h-[40px] flex justify-center items-center", "" }
        };
    }
    let total_slot = use_signal(move || (total_page - 1) / size);
    let mut selected_page = use_signal(move || current_page);
    let mut current_slot = use_signal(move || (current_page - 1) / size);

    let start_page = use_memo(move || current_slot() * size + 1);
    let repeat = use_memo(move || {
        if current_slot() == total_slot() && total_page % size != 0 {
            total_page % size
        } else {
            size
        }
    });

    rsx! {
        div { class: "flex flex-row w-full justify-center items-center mt-[30px]",
            if current_slot() > 0 {
                button {
                    class: "flex flex-row items-center justify-center rounded-lg m-[20px] w-[40px] h-[40px] hover:bg-gray-100",
                    onclick: move |_| {
                        current_slot.set(current_slot() - 1);
                    },
                    ArrowLeft { width: "24", height: "24" }
                }
            } else {
                div { class: "m-[20px] w-[40px] h-[40px]" }
            }

            div { class: "flex flex-row gap-[5px]",
                for i in 0..repeat() {
                    button {
                        class: "cursor-pointer flex flex-row w-[40px] h-[40px] justify-center items-center rounded-lg font-bold text-[15px]",
                        background: if start_page() + i == selected_page() { "#7c8292" } else { "white" },
                        color: if start_page() + i == selected_page() { "white" } else { "#0d1732" },
                        border: if start_page() + i == selected_page() { "none" } else { "1px solid #dfdfdf" },
                        onclick: move |_| {
                            let page = start_page() + i;
                            selected_page.set(page);
                            onclick(page);
                        },

                        "{start_page() + i}"
                    }
                }
            }

            if current_slot() < total_slot() {
                button {
                    class: "flex flex-row items-center justify-center rounded-lg m-[20px] w-[40px] h-[40px] hover:bg-gray-100",
                    onclick: move |_| {
                        current_slot.set(current_slot() + 1);
                    },
                    ArrowRight { width: "24", height: "24" }
                }
            } else {
                div { class: "m-[20px] w-[40px] h-[40px]" }
            }
        }
    }
}
