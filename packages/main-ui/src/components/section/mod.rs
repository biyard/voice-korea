use bdk::prelude::*;
#[component]
pub fn Section(required: bool, title: String, children: Element) -> Element {
    rsx! {
        div { class: "flex flex-row w-full justify-start items-center",
            div { class: "flex flex-row justify-start items-center w-150",
                if required {
                    div { class: "text-base font-bold text-necessary mr-2", "*" }
                }
                div { class: "text-[15px] font-medium text-text-black", "{title}" }
            }
            {children}
        }
    }
}
