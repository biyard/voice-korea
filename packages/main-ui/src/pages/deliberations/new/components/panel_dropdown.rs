use dioxus::prelude::*;
use models::PanelV2Summary;

use crate::components::{close_label::CloseLabel, icons::Remove};

#[component]
pub fn PanelDropdown(
    id: String,
    label: String,
    hint: String,

    selected_panels: Vec<PanelV2Summary>,
    panels: Vec<PanelV2Summary>,

    add_panel: EventHandler<PanelV2Summary>,
    remove_panel: EventHandler<i64>,
    clear_panel: EventHandler<MouseEvent>,
) -> Element {
    #[cfg(feature = "web")]
    use crate::components::outside_hook::eventhook::use_outside_click;

    let mut is_focused = use_signal(|| false);

    #[cfg(feature = "web")]
    use_outside_click(&id, move |_| is_focused.set(false));

    rsx! {
        div { class: "flex flex-row w-full justify-start items-center mb-[10px]",
            div { class: "w-[180px] mr-[50px] text-[#222222] font-medium text-[15px]",
                {label}
            }
            div {
                id,
                class: "cursor-pointer relative flex flex-row w-full h-[55px] justify-center items-center bg-[#f7f7f7] rounded-md",
                onclick: move |_| {
                    let prev = is_focused();
                    is_focused.set(!prev);
                },

                div { class: "flex flex-row w-full items-center px-[15px] py-[10px] gap-[10px] justify-between",

                    if selected_panels.clone().len() != 0 {
                        div {
                            class: "flex flex-wrap flex-1 gap-[10px]",
                            visibility: if selected_panels.clone().len() != 0 { "flex" } else { "hidden" },
                            for panel in selected_panels.clone() {
                                CloseLabel {
                                    label: panel.name.clone(),
                                    onremove: move |event: Event<MouseData>| {
                                        event.stop_propagation();
                                        event.prevent_default();
                                        remove_panel.call(panel.id);
                                    },
                                }
                            }
                        }

                        button {
                            onclick: move |event: Event<MouseData>| {
                                event.stop_propagation();
                                event.prevent_default();
                                clear_panel.call(event);
                            },
                            Remove { width: "20", height: "20", fill: "#555462" }
                        }
                    } else {
                        div { class: "font-medium text-[15px] text-[#b4b4b4] bg-[#f7f7f7]",
                            "{hint}"
                        }
                    }
                }
                if is_focused() {
                    div {
                        class: "absolute top-full bg-white border border-[#bfc8d9] shadow-lg rounded-lg w-full h-[150px] overflow-y-scroll z-50",
                        onclick: move |event| {
                            event.stop_propagation();
                            event.prevent_default();
                        },
                        div { class: "flex flex-col w-full justify-start items-start",
                            div { class: format!("flex flex-col w-full justify-start items-center bg-white"),
                                for panel in panels.clone() {
                                    if !selected_panels.iter().any(|p| p.id == panel.id) {
                                        button {
                                            class: "flex flex-col w-full justify-start items-start px-[12px] py-[10px] hover:bg-[#f7f7f7] hover:border-l-2 hover:border-[#2a60d3]",
                                            onclick: move |event: Event<MouseData>| {
                                                event.stop_propagation();
                                                event.prevent_default();
                                                add_panel.call(panel.clone());
                                                is_focused.set(false);
                                            },
                                            div { class: "font-bold text-[#222222] text-[15px] mb-[5px]",
                                                "{panel.name}"
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
    }
}
