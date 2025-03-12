use dioxus::prelude::*;

use crate::{components::icons::ClockIcon, utils::time::get_hour_from_timestamp};

#[component]
pub fn ClockDropdown(id: String, time: i64, onchange: EventHandler<i64>) -> Element {
    #[cfg(feature = "web")]
    use crate::components::outside_hook::eventhook::use_outside_click;

    let mut is_focused = use_signal(|| false);

    #[cfg(feature = "web")]
    use_outside_click(&id, move |_| is_focused.set(false));

    let time_array = generate_time_array();

    let hour = get_hour_from_timestamp(time) as usize;
    let time = time_array[hour].clone();

    rsx! {
        div {
            id,
            class: "cursor-pointer relative flex flex-row w-fit h-[55px]",
            onclick: move |_| {
                let prev = is_focused();
                is_focused.set(!prev);
            },

            button { class: "flex flex-row w-[190px] focus:outline-none h-[55px] justify-between items-center bg-white border border-[#bfc8d9] rounded-[8px] px-[20px]",
                div { class: "font-normal text-[#222222] text-[16px]", "{time}" }
                ClockIcon { width: "28", height: "28" }
            }

            if is_focused() {
                div {
                    class: "absolute top-full bg-white border border-[#bfc8d9] shadow-lg rounded-lg w-full h-[200px] overflow-y-scroll z-50",
                    onclick: move |event| {
                        event.stop_propagation();
                        event.prevent_default();
                    },
                    div { class: "flex flex-col w-full justify-start items-start",
                        div { class: format!("flex flex-col w-full justify-start items-center bg-white"),
                            for (i , time) in time_array.clone().iter().enumerate() {
                                button {
                                    class: "flex flex-col w-full justify-start items-start px-[12px] py-[10px] hover:bg-[#f7f7f7] hover:border-l-2 hover:border-[#2a60d3]",
                                    onclick: move |event: Event<MouseData>| {
                                        event.stop_propagation();
                                        event.prevent_default();
                                        onchange.call(i as i64);
                                        is_focused.set(false);
                                    },
                                    div { class: "font-bold text-[#222222] text-[15px] mb-[5px]",
                                        "{time}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn generate_time_array() -> Vec<String> {
    (0..24)
        .map(|hour| format!("{:02}:00 {}", hour, if hour < 12 { "AM" } else { "PM" }))
        .collect()
}
