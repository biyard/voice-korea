use crate::components::icons::Clear;
use dioxus::prelude::*;

#[component]
pub fn CloseLabel(label: String, onremove: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div { class: "flex flex-row justify-between items-center px-[8px] py-[5px] bg-[#35343f] rounded-[8px]",
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
