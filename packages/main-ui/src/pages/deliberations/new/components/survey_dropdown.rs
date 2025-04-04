use dioxus::prelude::*;
use models::SurveyV2Summary;

use crate::components::{close_label::CloseLabel, icons::Remove};

#[component]
pub fn SurveyDropdown(
    id: String,
    hint: String,

    selected_surveys: Vec<SurveyV2Summary>,
    surveys: Vec<SurveyV2Summary>,

    add_survey: EventHandler<SurveyV2Summary>,
    remove_survey: EventHandler<i64>,
    clear_survey: EventHandler<MouseEvent>,
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

                if selected_surveys.clone().len() != 0 {
                    div {
                        class: "flex flex-wrap flex-1 gap-10",
                        visibility: if selected_surveys.clone().len() != 0 { "flex" } else { "hidden" },
                        for survey in selected_surveys.clone() {
                            CloseLabel {
                                label: survey.name.clone(),
                                onremove: move |event: Event<MouseData>| {
                                    event.stop_propagation();
                                    event.prevent_default();
                                    remove_survey.call(survey.id);
                                },
                            }
                        }
                    }

                    button {
                        onclick: move |event: Event<MouseData>| {
                            event.stop_propagation();
                            event.prevent_default();
                            clear_survey.call(event);
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
                            for survey in surveys.clone() {
                                if !selected_surveys.iter().any(|p| p.id == survey.id) {
                                    button {
                                        class: "flex flex-col w-full justify-start items-start px-12 py-10 hover:bg-background-gray hover:border-l-2 hover:border-hover",
                                        onclick: move |event: Event<MouseData>| {
                                            event.stop_propagation();
                                            event.prevent_default();
                                            add_survey.call(survey.clone());
                                            is_focused.set(false);
                                        },
                                        div { class: "font-bold text-text-black text-[15px] mb-5",
                                            "{survey.name}"
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
