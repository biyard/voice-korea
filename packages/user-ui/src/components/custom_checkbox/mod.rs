use dioxus::prelude::*;

#[component]
pub fn CustomCheckbox(
    #[props(default = false)] blocked: bool,
    mut checked: bool,
    onchange: EventHandler<bool>,
) -> Element {
    rsx! {
        label { class: "flex items-center cursor-pointer",
            input {
                r#type: "checkbox",
                class: "hidden",
                checked: "{checked}",
                onchange: move |_| {
                    if !blocked {
                        onchange.call(!checked);
                    }
                },
            }
            div {
                class: format!(
                    "w-[24px] h-[24px] flex items-center justify-center rounded-md transition-all {}",
                    if checked && !blocked {
                        "bg-[#8095EA]"
                    } else if checked && blocked {
                        "bg-[#B4B4B4]"
                    } else {
                        "border-1 bg-white border-gray-400"
                    },
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
