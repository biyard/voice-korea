use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_translate::Language;
use serde::{Deserialize, Serialize};

use crate::components::{
    checkbox::Checkbox,
    icons::{Switch, Trash},
};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct AttributeGroupInfo {
    pub name: String,
    pub attribute: String,
    pub rate: i64,
}

#[component]
pub fn ParticipantDistributeTable(
    lang: Language,
    attribute_options: HashMap<String, Vec<String>>,
    selected_attributes: Vec<String>,
) -> Element {
    let is_initialize = use_signal(|| false);
    let mut attribute_groups = use_signal(|| vec![]);

    use_effect(use_reactive(&selected_attributes, {
        let options = attribute_options.clone();

        move |selected_attributes| {
            let mut new_groups = vec![];

            let selected_values: Vec<Vec<String>> = selected_attributes
                .iter()
                .filter_map(|key| {
                    let attributes = options.get(key)?.clone();
                    let group_name = key.clone();
                    for attr in &attributes {
                        new_groups.push(AttributeGroupInfo {
                            name: group_name.clone(),
                            attribute: attr.clone(),
                            rate: 0,
                        });
                    }
                    Some(attributes)
                })
                .collect();

            let mut ind = 0;
            for values in selected_values {
                let len = values.len();
                let d = 100 / len;
                let m = 100 % len;

                for _ in 0..(len - m) {
                    new_groups[ind].rate = d as i64;
                    ind += 1;
                }
                for _ in 0..m {
                    new_groups[ind].rate = (d + 1) as i64;
                    ind += 1;
                }
            }

            attribute_groups.set(new_groups);

            tracing::debug!("attributes: {:?} {:?}", is_initialize, attribute_groups);
        }
    }));

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start gap-[20px]",
            div { class: "flex flex-row w-full justify-start items-center",
                div { class: "min-w-150 font-medium text-[15px] text-black leading-18",
                    "인원 분배"
                }
                div { class: "flex flex-row w-full justify-start items-center gap-50",
                    div { class: "flex flex-row w-fit justify-start items-center",
                        Checkbox {
                            id: "evenly",
                            checked: Some(true),
                            onchange: move |_| {},
                        }
                        div { class: "font-normal text-[15px] text-text-black leading-18",
                            "균등 분배"
                        }
                    }

                    div { class: "flex flex-row w-fit justify-start items-center",
                        Checkbox {
                            id: "manual",
                            checked: Some(false),
                            onchange: move |_| {},
                        }
                        div { class: "font-normal text-[15px] text-text-black leading-18",
                            "수동 지정"
                        }
                    }
                }
            }

            div { class: "flex flex-col w-full justify-start items-start border rounded-lg border-label-border-gray",
                div { class: "flex flex-row w-full min-h-55 justify-start items-center",
                    div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                        div { class: "text-table-text-gray font-semibold text-sm", "속성 그룹" }
                        Switch { width: "19", height: "19" }
                    }
                    div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                        div { class: "text-table-text-gray font-semibold text-sm", "속성" }
                        Switch { width: "19", height: "19" }
                    }
                    div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                        div { class: "text-table-text-gray font-semibold text-sm", "비율(%)" }
                        Switch { width: "19", height: "19" }
                    }
                    div { class: "flex flex-row w-100 h-full justify-center items-center" }
                }

                for (index , group) in attribute_groups.iter().enumerate() {
                    div { class: "flex flex-col w-full justify-start items-start",
                        div { class: "flex flex-row w-full h-1 bg-label-border-gray" }
                        div { class: "flex flex-row w-full min-h-55 h-fit py-5",
                            div { class: "flex flex-row flex-1 h-full justify-center items-center",
                                div { class: "font-semibold text-sm text-third", "{group.name}" }
                            }
                            div { class: "flex flex-row flex-1 h-full justify-center items-center",
                                AttributeLabel { label: group.attribute.clone() }
                            }
                            div { class: "flex flex-row flex-1 h-full justify-center items-center",
                                input {
                                    r#type: "number",
                                    class: "flex flex-row w-50 bg-transparent text-text-black focus:outline-none",
                                    value: group.rate,
                                    oninput: {
                                        let mut attribute_groups = attribute_groups.clone();
                                        move |e: Event<FormData>| {
                                            if let Ok(v) = e.value().parse::<i64>() {
                                                attribute_groups
                                                    .with_mut(|groups| {
                                                        if let Some(g) = groups.get_mut(index) {
                                                            g.rate = v;
                                                        }
                                                    });
                                            }
                                        }
                                    },
                                }
                            }
                            div {
                                class: "cursor-pointer flex flex-row w-100 h-full justify-center items-center",
                                onclick: move |_| {},
                                div { class: "flex flex-row w-fit h-fit px-8 py-4 border border-delete-border-gray rounded-b-sm gap-5",
                                    div { class: "font-medium text-sm text-table-text-gray leading-22",
                                        "삭제"
                                    }
                                    Trash { width: "18", height: "18" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn AttributeLabel(label: String) -> Element {
    rsx! {
        div { class: "flex flex-row w-fit h-fit px-8 py-3 rounded-sm bg-label-black",
            div { class: "font-semibold text-white text-sm leading-18", {label} }
        }
    }
}
