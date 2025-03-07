#![allow(non_snake_case)]
use crate::components::icons::Clear;
use dioxus::prelude::*;

#[component]
pub fn MemberLabel(label: String, onremove: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div { class: "flex flex-row h-[25px] justify-between items-center px-[8px] bg-[#35343f] rounded-[4px]",
            div { class: "font-semibold text-[14px] text-white", {label} }
            button {
                onclick: move |e: MouseEvent| {
                    onremove.call(e);
                },
                Clear { width: "18", height: "18" }
            }
        }
    }
}
