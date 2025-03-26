use dioxus::prelude::*;

use crate::components::icons::search::Search;

#[component]
pub fn SearchBox(
    #[props(default = "flex flex-row w-full placeholder-[#bebebe] bg-white text-[#222222] focus:outline-none".to_string())]
    class: String,
    width: Option<i64>,
    height: Option<i64>,
    placeholder: String,
    value: String,
    onsearch: EventHandler<String>,
) -> Element {
    rsx! {
        div { class: "flex flex-row w-full justify-start items-start border border-[#e6e6e6] bg-white rounded-[8px] p-[10px] gap-[8px] focus:border focus:border-[#8095ea]",
            Search { width: "24", height: "24", color: "#afafaf" }

            input {
                class,
                width,
                height,
                placeholder,
                value,
                onchange: move |e| {
                    onsearch.call(e.value());
                },
            }
        }
    }
}
