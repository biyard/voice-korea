use dioxus::prelude::*;
use num_format::{Locale, ToFormattedString};

#[component]
pub fn PanelSettingInput(
    label: String,
    unit: String,
    value: i64,
    oninput: EventHandler<i64>,
) -> Element {
    tracing::debug!("input value: {}", value);

    rsx! {
        div { class: "flex flex-row w-full justify-between items-center",
            div { class: "font-medium text-[#222222] text-[15px]", "{label}" }
            div { class: "flex flex-row h-[55px] items-center gap-[10px]",
                input {
                    class: "flex flex-row w-[215px] h-[55px] justify-end items-center rounded-[4px] px-[15px] py-[10px] bg-[#f7f7f7] font-medium text-[#222222] text-[15px] text-right",
                    r#type: "text",
                    placeholder: "0",
                    value: value.to_formatted_string(&Locale::en),
                    oninput: move |e| {
                        let v = e.value().parse::<i64>().unwrap_or(value);
                        oninput.call(v);
                    },
                }

                div { class: "font-normal text-black text-[15px]", "{unit}" }
            }
        }
    }
}
