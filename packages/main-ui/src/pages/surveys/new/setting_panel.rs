use super::i18n::*;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::*;
use models::PanelV2;
use num_format::{Locale, ToFormattedString};

use crate::{
    components::icons::{Clear, Remove},
    pages::surveys::new::controller::*,
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PanelRequest {
    pub total_panels: i64,
    pub selected_panels: Vec<PanelV2>,
}

#[component]
pub fn SettingPanel(
    lang: Language,
    visibility: bool,

    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    survey_id: Option<i64>,
    onback: EventHandler<()>,
    onnext: EventHandler<PanelRequest>,
) -> Element {
    let mut is_open = use_signal(|| false);
    let mut ctrl = PanelController::new(lang, survey_id)?;

    let total_panels = ctrl.input_total_panels_memo;
    let translate: SettingPanelTranslate = translate(&lang);
    let mut panel_error = use_signal(|| false);
    let mut selected_panels = ctrl.selected_panels;

    rsx! {
        div {
            class: "flex flex-col w-full justify-start items-start",
            visibility: if !visibility { "hidden" },
            width: if !visibility { "0px" },
            height: if !visibility { "0px" },
            ..attributes,
            div { class: "flex flex-row w-full justify-between items-center mb-[10px]",
                div { class: "font-medium text-black text-[16px] leading-[22.5px]",
                    "{translate.composition_panel}"
                }
                button {
                    class: "bg-[#2a60d3] rounded-4px px-[14px] py-[8px] font-semibold text-white text-[16px] rounded-[4px]",
                    onclick: move |_| {
                        ctrl.open_create_panel_modal();
                    },
                    "{translate.create_panel}"
                }
            }

            div {
                class: "flex flex-col w-full justify-start items-start px-[40px] py-[24px] bg-white rounded-[8px]",
                style: "box-shadow: 0 4px 6px rgba(53, 70, 177, 0.05);",
                div { class: "flex flex-col w-full justify-start items-start mb-[20px]",
                    div { class: "font-bold text-[#222222] text-lg mb-[5px]",
                        "{translate.total_panel_setting}"
                    }
                    div { class: "font-normal text-[#6d6d6d] text-[14px]",
                        "{translate.total_panel_setting_description}"
                    }
                }

                div { class: "flex flex-col w-full justify-start items-start",

                    PanelSettingInput {
                        label: "{translate.total_panel}",
                        unit: "{translate.person}",
                        value: total_panels(),
                        oninput: move |value: i64| {
                            ctrl.change_total_panels(value);
                        },
                    }

                    div { class: "flex flex-row w-full justify-between items-center mt-[10px]",
                        div { class: "flex flex-row w-[200px] font-medium text-[15px] text-black",
                            "{translate.select_panel}"
                        }
                        div { class: "relative w-full",
                            button {
                                class: "flex flex-row w-full min-h-[55px] justify-start items-center bg-[#f7f7f7] rounded-[4px] p-[15px]",
                                onclick: move |_| {
                                    is_open.set(true);
                                },
                                if selected_panels.len() != 0 {
                                    div { class: "flex flex-wrap flex-1 justify-start items-center gap-[5px]",
                                        for (i , panel) in selected_panels.iter().enumerate() {
                                            PanelLabel {
                                                label: panel.0.name.clone(),
                                                onclose: move |_| {
                                                    selected_panels.remove(i);
                                                },
                                            }
                                        }
                                    }
                                    button {
                                        onclick: move |_| {
                                            selected_panels.clear();
                                        },
                                        Remove {
                                            width: "15",
                                            height: "15",
                                            fill: "#555462",
                                        }
                                    }
                                }
                            }
                            if is_open() {
                                div { class: "absolute flex flex-col w-full justify-start items-center shadow-[0px_8px_20px_rgba(20,26,62,0.25)] bg-white py-4 rounded-md",
                                    div { class: "flex flex-row w-full justify-end px-[10px]",
                                        button {
                                            onclick: move |_| {
                                                is_open.set(false);
                                            },
                                            Remove {
                                                width: "15",
                                                height: "15",
                                                fill: "#555462",
                                            }
                                        }
                                    }
                                    if let Some(panels) = ctrl.panels.value()() {
                                        for panel in panels.items {
                                            if !selected_panels.iter().any(|selected| selected.0.name == panel.name) {
                                                div {
                                                    class: "flex flex-col w-full h-[60px] justify-start items-start py-[9px] bg-white hover:bg-[#f7f7f7] hover:border-l hover:border-l-[#2a60d3] cursor-pointer",
                                                    onclick: move |_| {
                                                        ctrl.add_selected_panel(panel.clone());
                                                        is_open.set(false);
                                                    },
                                                    div { class: "flex flex-col w-full px-4",
                                                        div { class: "font-bold text-[15px] text-[#222222] mb-[5px]",
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


                div { class: "flex flex-col w-full justify-start items-start",
                    div { class: "flex flex-row w-full h-[1px] bg-[#ebeff5] my-[20px]" }
                    div { class: "flex flex-col w-full justify-start items-start gap-[10px]",
                        for (i , sp) in selected_panels().iter().enumerate() {
                            PanelSettingInput {
                                label: "{sp.0.name}",
                                unit: "{translate.person}",
                                value: sp.1,
                                oninput: move |value: i64| {
                                    ctrl.change_number_by_index(i, value);
                                },
                            }
                        }
                    }
                }
            }

            if panel_error() {
                div { class: "text-red-500 text-sm mt-2", "{translate.panel_error_message}" }
            }

            div { class: "flex flex-row w-full justify-end items-center gap-[20px] text-white mt-[40px]",
                button {
                    class: "px-[20px] py-[10px] border-[#BFC8D9] bg-white border-[1px] text-[#555462] font-semibold text-[14px] rounded-[4px]",
                    onclick: move |_| {
                        onback(());
                    },
                    "{translate.btn_cancel}"
                }

                button {
                    class: "px-[20px] py-[10px] bg-[#2A60D3] font-semibold text-[14px] rounded-[4px]",
                    onclick: move |_| async move {
                        let panels = selected_panels();
                        let is_valid = !panels.is_empty() && panels.iter().all(|v| v.1 > 0);
                        panel_error.set(!is_valid);
                        if !is_valid {
                            return;
                        }
                        onnext(PanelRequest {
                            total_panels: total_panels(),
                            selected_panels: panels
                                .iter()
                                .map(|v| {
                                    let user_count = v.1;
                                    let v = v.0.clone();
                                    PanelV2 {
                                        id: v.id.clone(),
                                        created_at: v.created_at,
                                        updated_at: v.updated_at,
                                        name: v.name.clone(),
                                        user_count: user_count as u64,
                                        attributes: v.attributes,
                                        org_id: v.org_id,
                                    }
                                })
                                .collect(),
                        });
                    },
                    "{translate.btn_complete}"
                }
            
            }
        }
    }
}

#[component]
pub fn PanelSettingBox(label: String, unit: String, value: i64) -> Element {
    rsx! {
        div { class: "flex flex-row w-full justify-between items-center",
            div { class: "font-medium text-[#222222] text-[15px]", "{label}" }
            div { class: "flex flex-row h-[55px] items-center gap-[10px]",
                div { class: "flex flex-row w-[215px] h-[55px] justify-end items-center rounded-[4px] px-[15px] py-[10px] bg-[#f7f7f7] font-medium text-[#222222] text-[15px] text-right",
                    "{value}"
                }

                div { class: "font-normal text-black text-[15px]", "{unit}" }
            }
        }
    }
}

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
// TODO: 패널 선택이 작동 안함, 코드 수정 필요
#[component]
pub fn PanelLabel(label: String, onclose: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div { class: "flex flex-row h-[25px] justify-center items-center px-[8px] py-[3px] bg-[#35343f] rounded-[5px] gap-[10px]",
            div { class: "font-semibold text-[14px] text-white", {label} }
            button {
                onclick: move |e: Event<MouseData>| {
                    onclose.call(e);
                },
                Clear { width: "20", height: "20" }
            }
        }
    }
}
