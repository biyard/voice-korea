use dioxus::prelude::*;

use crate::{
    components::{calendar::Calendar, icons::CalendarIcon},
    utils::time::change_date_from_timestamp,
};

#[component]
pub fn CalendarDropdown(id: String, date: i64, onchange: EventHandler<i64>) -> Element {
    #[cfg(feature = "web")]
    use crate::components::outside_hook::eventhook::use_outside_click;

    let mut is_focused = use_signal(|| false);

    #[cfg(feature = "web")]
    use_outside_click(&id, move |_| is_focused.set(false));

    rsx! {
        div {
            id,
            class: "cursor-pointer relative flex flex-row w-fit h-[55px]",
            onclick: move |_| {
                let prev = is_focused();
                is_focused.set(!prev);
            },
            div { class: "group relative",
                button { class: "flex flex-row w-[190px] focus:outline-none h-[55px] justify-between items-center bg-white border border-[#bfc8d9] rounded-[8px] px-[20px]",
                    div { class: "font-normal text-[16px] text-[#9b9b9b] leading-[24px]",
                        {change_date_from_timestamp(date)}
                    }
                    CalendarIcon { width: "28", height: "28" }
                }
                nav { class: "absolute top-full right-0 z-20 opacity-0 invisible group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 transition-all",
                    Calendar {
                        timestamp: date as u64,
                        update_date: move |timestamp: i64| {
                            onchange.call(timestamp);
                        },
                    }
                }
            }
        }
    }
}
