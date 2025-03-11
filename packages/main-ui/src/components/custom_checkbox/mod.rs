use dioxus::prelude::*;

#[component]
pub fn CustomCheckbox(mut checked: bool, onchange: EventHandler<bool>) -> Element {
    rsx! {
        label { class: "flex items-center cursor-pointer",
            input {
                r#type: "checkbox",
                class: "hidden",
                checked: "{checked}",
                onchange: move |_| {
                    onchange.call(!checked);
                },
            }
            div {
                class: format!(
                    "w-[24px] h-[24px] flex items-center justify-center rounded-md transition-all {}",
                    if checked { "bg-[#2A60D3]" } else { "border border-1 bg-white border-gray-400" },
                ),
                div { class: "text-white text-lg",
                    if checked {
                        div { "✔" }
                    }
                }
            }
        }
    }
}
