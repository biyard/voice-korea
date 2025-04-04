use dioxus::prelude::*;
use models::OrganizationMemberSummary;

use crate::components::{close_label::CloseLabel, icons::Remove};

#[component]
pub fn CommitteeDropdown(
    id: String,
    hint: String,

    selected_committees: Vec<OrganizationMemberSummary>,
    committees: Vec<OrganizationMemberSummary>,

    add_committee: EventHandler<OrganizationMemberSummary>,
    remove_committee: EventHandler<i64>,
    clear_committee: EventHandler<MouseEvent>,
) -> Element {
    #[cfg(feature = "web")]
    use crate::components::outside_hook::eventhook::use_outside_click;

    let mut is_focused = use_signal(|| false);

    #[cfg(feature = "web")]
    use_outside_click(&id, move |_| is_focused.set(false));

    rsx! {
        div {
            id,
            class: "cursor-pointer relative flex flex-row w-full h-55 justify-center items-center bg-background-gray rounded-md",
            onclick: move |_| {
                let prev = is_focused();
                is_focused.set(!prev);
            },

            div { class: "flex flex-row w-full items-center px-15 py-10 gap-10 justify-between",

                if selected_committees.clone().len() != 0 {
                    div {
                        class: "flex flex-wrap flex-1 gap-10",
                        visibility: if selected_committees.clone().len() != 0 { "flex" } else { "hidden" },
                        for committee in selected_committees.clone() {
                            CloseLabel {
                                label: committee.name.clone(),
                                onremove: move |event: Event<MouseData>| {
                                    event.stop_propagation();
                                    event.prevent_default();
                                    remove_committee.call(committee.id);
                                },
                            }
                        }
                    }

                    button {
                        onclick: move |event: Event<MouseData>| {
                            event.stop_propagation();
                            event.prevent_default();
                            clear_committee.call(event);
                        },
                        Remove { width: "20", height: "20", fill: "#555462" }
                    }
                } else {
                    div { class: "font-medium text-[15px] text-hint-gray bg-background-gray",
                        "{hint}"
                    }
                }
            }
            if is_focused() {
                div {
                    class: "absolute top-full bg-white border border-label-border-gray shadow-lg rounded-lg w-full h-150 overflow-y-scroll z-50",
                    onclick: move |event| {
                        event.stop_propagation();
                        event.prevent_default();
                    },
                    div { class: "flex flex-col w-full justify-start items-start",
                        div { class: format!("flex flex-col w-full justify-start items-center bg-white"),
                            for committee in committees.clone() {
                                if !selected_committees.iter().any(|p| p.id == committee.id) {
                                    button {
                                        class: "flex flex-col w-full justify-start items-start px-12 py-10 hover:bg-background-gray hover:border-l-2 hover:border-hover",
                                        onclick: move |event: Event<MouseData>| {
                                            event.stop_propagation();
                                            event.prevent_default();
                                            add_committee.call(committee.clone());
                                            is_focused.set(false);
                                        },
                                        div { class: "font-bold text-text-black text-[15px] mb-5",
                                            "{committee.name}"
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
