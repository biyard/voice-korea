#[cfg(feature = "web")]
use crate::components::outside_hook::eventhook::use_outside_click;
use crate::components::{close_label::CloseLabel, icons::Remove};
use bdk::prelude::*;

#[component]
pub fn Dropdown(
    id: String,
    hint: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    items: Vec<String>,
    #[props(default = 0)] selected: usize,
    onselect: EventHandler<Vec<String>>,
) -> Element {
    let mut is_focused = use_signal(|| false);
    let mut selected_options: Signal<Vec<String>> = use_signal(|| vec![]);

    #[cfg(feature = "web")]
    use_outside_click(&id, move |_| is_focused.set(false));

    rsx! {
        div {
            id,
            class: "cursor-pointer relative flex flex-row w-full h-[55px] justify-center items-center bg-[#f7f7f7] rounded-md",
            onclick: move |_| {
                let prev = is_focused();
                is_focused.set(!prev);
            },

            div { class: "flex flex-row w-full items-center px-[15px] py-[10px] gap-[10px] justify-between",

                if selected_options().len() != 0 {
                    div {
                        class: "flex flex-wrap flex-1 gap-[10px]",
                        visibility: if selected_options().len() != 0 { "flex" } else { "hidden" },
                        for (i , option) in selected_options.iter().enumerate() {
                            CloseLabel {
                                label: option.clone(),
                                onremove: move |event: Event<MouseData>| {
                                    event.stop_propagation();
                                    event.prevent_default();
                                    let mut so = selected_options();
                                    so.remove(i);
                                    selected_options.set(so);
                                    onselect.call(selected_options());
                                },
                            }
                        }
                    }

                    button {
                        onclick: move |event: Event<MouseData>| {
                            event.stop_propagation();
                            event.prevent_default();
                            selected_options.set(vec![]);
                            onselect.call(selected_options());
                        },
                        Remove { width: "20", height: "20", fill: "#555462" }
                    }
                } else {
                    div { class: "font-medium text-[15px] text-[#b4b4b4] bg-[#f7f7f7]",
                        "{hint}"
                    }
                }
            }
            div {
                class: "absolute top-full bg-white border border-[#bfc8d9] shadow-lg rounded-lg w-full h-[150px] overflow-y-scroll z-50",
                visibility: if !is_focused() { "hidden" },
                onclick: move |event| {
                    event.stop_propagation();
                    event.prevent_default();
                },
                div { class: "flex flex-col w-full justify-start items-start",
                    div { class: format!("flex flex-col w-full justify-start items-center bg-white"),
                        for item in items.into_iter() {
                            if !selected_options().iter().any(|selected| *selected == item) {
                                button {
                                    class: "flex flex-col w-full justify-start items-start px-[12px] py-[10px] hover:bg-[#f7f7f7] hover:border-l-2 hover:border-[#2a60d3]",
                                    onclick: move |event: Event<MouseData>| {
                                        event.stop_propagation();
                                        event.prevent_default();
                                        selected_options.push(item.clone());
                                        is_focused.set(false);
                                        onselect.call(selected_options());
                                    },
                                    div { class: "font-bold text-[#222222] text-[15px] mb-[5px]",
                                        {item.clone()}
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
