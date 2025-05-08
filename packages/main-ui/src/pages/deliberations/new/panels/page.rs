use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

use crate::{
    components::{form_field::EnterTextField, section::MainSection},
    pages::deliberations::new::{
        components::footer_buttons::FooterButtons, panels::components::panel_table::PanelTable,
    },
};

#[component]
pub fn CompositionPanel(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: CompositionPanelTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-row w-full justify-between items-center h-40 mb-15",
                div { class: "font-medium text-base text-text-black mb-10",
                    {tr.participant_panel_composition}
                }
            }

            div { class: "flex flex-col w-full justify-start items-start gap-15",
                SettingPanelByEmail {
                    lang,
                    value: ctrl.panel_emails(),
                    oninput: move |e: FormEvent| {
                        ctrl.update_panel_email(e.value());
                    },
                    onenter: move |_| {
                        ctrl.add_panel_email();
                    },
                }

                MainSection {
                    lang,
                    required: false,
                    header: Some(tr.summary_title.to_string()),
                    description: Some(tr.summary_description.to_string()),

                    PanelTable {
                        lang,
                        emails: ctrl.emails(),
                        onremove: move |index: usize| {
                            ctrl.remove_email(index);
                        },
                    }
                }
            }
            // SettingPanel {
            //     lang,
            //     panels,
            //     selected_panels: ctrl.selected_panels(),
            //     add_panel: move |panel: PanelV2Summary| {
            //         ctrl.add_selected_panel(panel);
            //     },
            //     remove_panel: move |id: i64| {
            //         ctrl.remove_selected_panel(id);
            //     },
            //     clear_panel: move |_| {
            //         ctrl.clear_selected_panel();
            //     },
            //     change_selected_panel_by_index: move |(index, value): (usize, u64)| {
            //         ctrl.change_selected_panel_by_index(index, value);
            //     },
            // }

            FooterButtons {
                lang,
                on_backward: move |_| {
                    ctrl.back();
                },
                on_temp_save: move |_| async move { ctrl.temp_save().await },
                on_next: move |_| {
                    ctrl.next();
                },
                on_save: None,
                next_valid: ctrl.is_valid(),
            }
        }
    }
}

#[component]
pub fn SettingPanelByEmail(
    lang: Language,
    value: String,
    oninput: EventHandler<FormEvent>,
    onenter: EventHandler<KeyboardEvent>,
) -> Element {
    let tr: SettingTotalPanelTranslate = translate(&lang);

    rsx! {
        MainSection {
            lang,
            required: false,
            header: Some(tr.setting_total_panel_title.to_string()),
            description: Some(tr.setting_total_panel_description.to_string()),
            EnterTextField {
                height: 300,
                placeholder: tr.input_panel_hint,
                value,
                oninput,
                onenter,
            }
        }
    }
}

// #[component]
// pub fn SettingPanel(
//     lang: Language,

//     panels: Vec<PanelV2Summary>,
//     selected_panels: Vec<PanelV2Summary>,
//     add_panel: EventHandler<PanelV2Summary>,
//     remove_panel: EventHandler<i64>,
//     clear_panel: EventHandler<MouseEvent>,
//     change_selected_panel_by_index: EventHandler<(usize, u64)>,
// ) -> Element {
//     let tr: SettingTotalPanelTranslate = translate(&lang);
//     rsx! {
//         div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white px-40 py-24",
//             div { class: "font-bold text-text-black text-lg mb-3", {tr.setting_total_panel_title} }
//             div { class: "font-normal text-text-gray text-sm mb-20",
//                 {tr.setting_total_panel_description}
//             }

//             PanelDropdown {
//                 id: "dropdown_deliberation_panel",
//                 label: tr.select_panel,
//                 hint: tr.panel_hint,
//                 selected_panels: selected_panels.clone(),
//                 panels,
//                 add_panel,
//                 remove_panel,
//                 clear_panel,
//             }

//             div { class: "flex flex-row w-full h-1 bg-period-border-gray my-20" }
//             div { class: "flex flex-col w-full justify-start items-start gap-10",
//                 for (i , sp) in selected_panels.clone().iter().enumerate() {
//                     PanelSettingInput {
//                         label: sp.name.clone(),
//                         unit: tr.unit,
//                         value: sp.user_count as i64,
//                         oninput: move |value: i64| {
//                             change_selected_panel_by_index.call((i, value as u64));
//                         },
//                     }
//                 }
//             }
//         }
//     }
// }

// #[component]
// pub fn Label(label: String, clicked_label: EventHandler<MouseEvent>) -> Element {
//     rsx! {
//         div { class: "flex flex-row h-25 justify-between items-center pl-8 bg-label-black rounded-sm",
//             div { class: "font-semibold text-sm text-white", {label} }
//             button {
//                 onclick: move |e: MouseEvent| {
//                     clicked_label.call(e);
//                 },
//                 Clear { width: "24", height: "24" }
//             }
//         }
//     }
// }
