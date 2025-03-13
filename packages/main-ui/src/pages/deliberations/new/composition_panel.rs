use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::PanelV2Summary;

use crate::{
    components::icons::Clear,
    pages::deliberations::new::{
        components::{panel_dropdown::PanelDropdown, panel_setting_input::PanelSettingInput},
        i18n::{CompositionPanelTranslate, SettingTotalPanelTranslate},
    },
};

use super::controller::CurrentStep;

#[component]
pub fn CompositionPanel(
    lang: Language,
    panels: Vec<PanelV2Summary>,
    selected_panels: Vec<PanelV2Summary>,

    add_panel: EventHandler<PanelV2Summary>,
    remove_panel: EventHandler<i64>,
    clear_panel: EventHandler<MouseEvent>,
    change_selected_panel_by_index: EventHandler<(usize, u64)>,
    onstep: EventHandler<CurrentStep>,
) -> Element {
    let translates: CompositionPanelTranslate = translate(&lang);
    let selected_option = use_signal(move || translates.proportional_people_allocated.to_string());

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-row w-full justify-between items-center h-[40px] mb-[15px]",
                div { class: "font-medium text-[16px] text-[#222222] mb-[10px]",
                    "{translates.participant_panel_composition}"
                }
                        // button {
            //     class: "flex flex-row px-[14px] py-[8px] bg-[#2a60d3] rounded-[4px] font-semibold text-white text-[16px]",
            //     onclick: {
            //         let translates = translates.clone();
            //         move |_| {
            //             ctrl.open_create_panel_modal(lang, translates.clone());
            //         }
            //     },
            //     "{translates.create_panel}"
            // }
            }
            SettingPanel {
                lang,
                selected_option,
                panels,
                selected_panels,
                add_panel,
                remove_panel,
                clear_panel,
                change_selected_panel_by_index,
            }

            div { class: "flex flex-row w-full justify-end items-end mt-[40px] mb-[50px]",
                div {
                    class: "flex flex-row w-[70px] h-[55px] rounded-[4px] justify-center items-center bg-white border border-[#bfc8d9] font-semibold text-[16px] text-[#555462] mr-[20px]",
                    onclick: move |_| {
                        onstep.call(CurrentStep::CommitteeComposition);
                    },
                    "{translates.backward}"
                }
                div {
                    class: "flex flex-row w-[105px] h-[55px] rounded-[4px] justify-center items-center bg-white border border-[#bfc8d9] font-semibold text-[16px] text-[#555462] mr-[20px]",
                    onclick: move |_| {},
                    "{translates.temporary_save}"
                }
                div {
                    class: "cursor-pointer flex flex-row w-[110px] h-[55px] rounded-[4px] justify-center items-center bg-[#2a60d3] font-semibold text-[16px] text-white",
                    onclick: move |_| {
                        onstep.call(CurrentStep::DiscussionSetting);
                    },
                    "{translates.next}"
                }
            }
        }
    }
}

// TODO(web): add to create panel modal
// #[component]
// pub fn CreateNewPanelModal(
//     attributes: Signal<Vec<AttributeResponse>>,
//     lang: Language,
//     onclick: EventHandler<String>,
//     onsave: EventHandler<String>,
//     onclose: EventHandler<MouseEvent>,
// ) -> Element {
//     let translates: CreateNewPanelModalTranslate = translate(&lang);
//     let mut panel_name: Signal<String> = use_signal(|| "".to_string());

//     rsx! {
//         div { class: "flex flex-col w-[540px] min-w-[540px] justify-start items-start mt-[40px]",
//             div { class: "flex flex-col w-full justify-start items-start mb-[40px]",
//                 div { class: "font-semibold text-[#222222] text-[14px] mb-[16px]",
//                     "{translates.panel_name}"
//                 }
//                 div { class: "flex flex-row w-full focus:outline-none h-[45px] justify-start items-center bg-[#f7f7f7] rounded-[4px] px-[15px] mb-[5px]",
//                     input {
//                         class: "flex flex-row w-full justify-start items-center bg-transparent focus:outline-none",
//                         r#type: "text",
//                         placeholder: "{translates.panel_name_hint}",
//                         value: panel_name(),
//                         oninput: move |event| {
//                             panel_name.set(event.value());
//                         },
//                     }
//                 }
//                 div { class: "font-normal text-[#222222] text-[13px]", "{translates.panel_name_info}" }
//             }

//             div { class: "flex flex-col w-full justify-start items-start p-[24px] bg-white border border-[#bfc8d9] rounded-[8px] mb-[10px]",
//                 for (i , attribute) in attributes().iter().enumerate() {
//                     div { class: "flex flex-row w-full justify-start items-center h-[45px] mb-[10px]",
//                         div { class: "flex flex-row w-[50px] justify-start items-center font-medium text-[#222222] text-[15px]",
//                             {format!("{}", attribute.name.clone().unwrap_or_default())}
//                         }
//                         div { class: "flex flex-row w-full h-[45px] justify-between items-center bg-[#f7f7f7] rounded-[4px]",
//                             div { class: "flex flex-between w-full h-[55px] justify-start items-center p-[15px]",
//                                 if attributes.len() != 0 {
//                                     div { class: "flex flex-wrap w-full justify-start items-center gap-[5px]",
//                                         for (j , attr) in attribute.attribute.iter().enumerate() {
//                                             div {
//                                                 Label {
//                                                     label: attr.name.clone(),
//                                                     clicked_label: move |_e: MouseEvent| {
//                                                         let mut ats = attributes().clone();
//                                                         ats[i].attribute.remove(j);
//                                                         attributes.set(ats);
//                                                     },
//                                                 }
//                                             }
//                                         }
//                                     }
//                                     button {
//                                         onclick: move |_| {
//                                             let mut ats = attributes().clone();
//                                             ats[i].attribute = vec![];
//                                             attributes.set(ats);
//                                         },
//                                         Remove {
//                                             width: "15",
//                                             height: "15",
//                                             fill: "#555462",
//                                         }
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }

//             div { class: "flex flex-row w-full justify-end items-end mb-[40px]",
//                 button {
//                     onclick: move |_| {
//                         onclick.call(panel_name());
//                     },
//                     class: "font-normal text-[#222222] text-[14px] underline",
//                     "{translates.add_attribute}"
//                 }
//             }

//             div { class: "flex flex-row w-full justify-start items-start mb-[20px]",
//                 //FIXME: fix to real data
//                 div { class: "font-normal text-[#6d6d6d] text-[14px]",
//                     {
//                         format!(
//                             "({}) {} 120명",
//                             if panel_name() == "" {
//                                 translates.panel_name.to_string()
//                             } else {
//                                 panel_name()
//                             },
//                             translates.total_member,
//                         )
//                     }
//                 }
//             }

//             div { class: "flex flex-row w-full justify-start items-start gap-[20px]",
//                 button {
//                     class: "flex flex-row px-[14px] py-[8px] bg-[#2a60d3] rounded-[4px] font-semibold text-white text-[16px]",
//                     onclick: move |_| {
//                         onsave.call(panel_name());
//                     },
//                     "{translates.save}"
//                 }
//                 button {
//                     class: "flex flex-row px-[14px] py-[8px] bg-white font-semibold text-[#222222] text-[16px]",
//                     onclick: move |e| {
//                         onclose.call(e);
//                     },
//                     "{translates.cancel}"
//                 }
//             }
//         }
//     }
// }

#[component]
pub fn SettingPanel(
    lang: Language,
    selected_option: Signal<String>,

    panels: Vec<PanelV2Summary>,
    selected_panels: Vec<PanelV2Summary>,
    add_panel: EventHandler<PanelV2Summary>,
    remove_panel: EventHandler<i64>,
    clear_panel: EventHandler<MouseEvent>,
    change_selected_panel_by_index: EventHandler<(usize, u64)>,
) -> Element {
    let translates: SettingTotalPanelTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white px-[40px] py-[24px]",
            div { class: "font-bold text-[#222222] text-lg mb-[3px]",
                "{translates.setting_total_panel_title}"
            }
            div { class: "font-normal text-[#6d6d6d] text-sm mb-[20px]",
                "{translates.setting_total_panel_description}"
            }

            PanelDropdown {
                id: "dropdown_deliberation_panel",
                label: translates.select_panel,
                hint: translates.panel_hint,
                selected_panels: selected_panels.clone(),
                panels,
                add_panel,
                remove_panel,
                clear_panel,
            }

            div { class: "flex flex-row w-full h-[1px] bg-[#ebeff5] my-[20px]" }
            div { class: "flex flex-col w-full justify-start items-start gap-[10px]",
                for (i , sp) in selected_panels.clone().iter().enumerate() {
                    PanelSettingInput {
                        label: "{sp.name}",
                        unit: translates.unit,
                        value: sp.user_count as i64,
                        oninput: move |value: i64| {
                            change_selected_panel_by_index.call((i, value as u64));
                        },
                    }
                }
            }
        }
    }
}

#[component]
pub fn Label(label: String, clicked_label: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div { class: "flex flex-row h-[25px] justify-between items-center pl-[8px] bg-[#35343f] rounded-[4px]",
            div { class: "font-semibold text-[14px] text-white", {label} }
            button {
                onclick: move |e: MouseEvent| {
                    clicked_label.call(e);
                },
                Clear { width: "24", height: "24" }
            }
        }
    }
}
