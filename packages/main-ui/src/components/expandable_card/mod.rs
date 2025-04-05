#![allow(non_snake_case)]
use bdk::prelude::*;

use crate::components::icons::{BottomDropdownArrow, TopDropdownArrow};

#[component]
pub fn ExpandableCard(
    required: bool,
    header: String,
    description: String,
    children: Element,
) -> Element {
    let mut clicked = use_signal(|| false);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white px-40 py-24 gap-10",
            div {
                class: "flex flex-row w-full justify-between items-center cursor-pointer",
                onclick: move |_| {
                    clicked.set(!clicked());
                },

                div { class: "flex flex-col w-full justify-start items-start",
                    div { class: "flex flex-row h-full items-center justify-center",
                        if required {
                            div { class: "text-base font-bold text-necessary mr-2 text-center",
                                "*"
                            }
                        }
                        div { class: "text-lg font-bold text-text-black", "{header}" }
                    }
                    div { class: "text-sm font-normal text-text-gray", "{description}" }
                }
                if clicked() {
                    TopDropdownArrow { width: "24", height: "24" }
                } else {
                    BottomDropdownArrow { width: "24", height: "24" }
                }
            }

            div {
                class: "w-full transition-all",
                display: if clicked() { "block" } else { "none" },
                {children}
            }
        }
    }
}
