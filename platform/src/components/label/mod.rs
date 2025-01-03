#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::components::icons::Remove;

#[component]
pub fn Label(
    label_name: String,
    label_color: String,
    #[props(default = true)] is_delete: bool,
) -> Element {
    rsx! {
        if is_delete {
            div {
                class: format!(
                    "flex flex-row h-[25px] justify-between items-center {} rounded-[20px] px-[5px] py-[3px]",
                    label_color,
                ),
                div { class: "text-white font-semibold text-[14px] mr-[7px]", {label_name} }
                Remove { width: "18", height: "18" }
            }
        } else {
            div {
                class: format!(
                    "flex flex-row h-[25px] justify-center items-center {} rounded-[20px] px-[10px] py-[6px]",
                    label_color,
                ),
                div { class: "text-white font-semibold text-[14px]", {label_name} }
            }
        }
    }
}
