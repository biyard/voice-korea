#![allow(non_snake_case)]
use crate::pages::deliberations::new::i18n::DeliberationNewTranslate;
use crate::pages::deliberations::new::step::composition_commitee::CompositionCommitee;
use crate::pages::deliberations::new::step::composition_deliberation::CompositionDeliberation;
use crate::pages::deliberations::new::step::composition_panel::CompositionPanel;
use crate::pages::deliberations::new::step::preview::Preview;
use crate::pages::deliberations::new::step::setting_info::SettingDeliberation;
use crate::{
    components::{icons::ArrowLeft, stepper::Stepper},
    pages::deliberations::new::controller::{Controller, CurrentStep},
    routes::Route,
};

use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::deliberation_user::DeliberationUserCreateRequest;
use models::step::StepCreateRequest;
use models::{PanelV2Summary, Role};

#[component]
pub fn OpinionCreatePage(lang: Language) -> Element {
    let tr: DeliberationNewTranslate = translate(&lang.clone());
    let mut ctrl = Controller::new(lang)?;
    let _surveys = ctrl.surveys()?;
    let _metadatas = ctrl.metadatas()?;
    let members = ctrl.members()?;
    let panels = ctrl.panels()?;

    let sequences = ctrl.get_deliberation_sequences();
    let check_sequence = ctrl.check_deliberation_info();
    let _informations = ctrl.get_deliberation_informations();
    let _resources = ctrl.resources();
    let step = ctrl.get_current_step();
    let _selected_surveys = ctrl.selected_surveys();
    let selected_panels = ctrl.get_selected_panels();
    let committees = ctrl.get_committees();
    let _discussions = ctrl.get_discussions();
    let _discussion_resources = ctrl.get_discussion_resources();

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div {
                class: format!(
                    "flex flex-col w-full justify-start items-start {}",
                    if step == CurrentStep::EditContent { "hidden" } else { "" },
                ),
                div { class: "text-header-gray font-medium text-sm mb-10",
                    "{tr.organization_management} / {tr.public_opinion_management}"
                }
                div { class: "flex flex-row w-full justify-start items-center mb-25",
                    Link { class: "mr-6", to: Route::DeliberationPage { lang },
                        ArrowLeft { width: "24", height: "24", color: "#3a3a3a" }
                    }
                    div { class: "text-header-black font-semibold text-[28px] mr-20",
                        "{tr.start_public_opinion}"
                    }
                }

                div { class: "flex flex-col w-full justify-start items-center mt-20 mb-80",
                    div { class: "flex flex-row w-1400 min-w-1400 justify-center items-center",
                        Stepper {
                            current_step: if step == CurrentStep::SettingInfo { 1 } else if step == CurrentStep::CompositionCommittee { 2 } else if step == CurrentStep::CompositionPanel { 3 } else if step == CurrentStep::DeliberationSchedule || step == CurrentStep::EditContent { 4 } else { 5 },
                            steps: vec![
                                tr.setup_deliberation_outline.to_string(),
                                tr.composition_of_deliberation.to_string(),
                                tr.composition_of_panel.to_string(),
                                tr.deliberation_procedures_and_schedule.to_string(),
                                tr.final_review.to_string(),
                            ],
                        }
                    }
                }
            }

            if step == CurrentStep::SettingInfo {
                SettingDeliberation {
                    lang,
                    onstep: move |step: CurrentStep| {
                        ctrl.change_step(step);
                    },
                }
            } else if step == CurrentStep::CompositionCommittee {
                CompositionCommitee {
                    lang,
                    members,
                    committees,
                    add_committee: move |committee: DeliberationUserCreateRequest| {
                        ctrl.add_committee(committee);
                    },
                    remove_committee: move |(user_id, role): (i64, Role)| {
                        ctrl.remove_committee(user_id, role);
                    },
                    clear_committee: move |role: Role| {
                        ctrl.clear_committee(role);
                    },
                    onstep: move |step: CurrentStep| {
                        ctrl.change_step(step);
                    },
                }
            } else if step == CurrentStep::CompositionPanel {
                CompositionPanel {
                    lang,
                    panels,
                    selected_panels,
                    add_panel: move |panel: PanelV2Summary| {
                        ctrl.add_selected_panel(panel);
                    },
                    remove_panel: move |id: i64| {
                        ctrl.remove_selected_panel(id);
                    },
                    clear_panel: move |_| {
                        ctrl.clear_selected_panel();
                    },
                    change_selected_panel_by_index: move |(index, value): (usize, u64)| {
                        ctrl.change_selected_panel_by_index(index, value);
                    },
                    onstep: move |step: CurrentStep| {
                        ctrl.change_step(step);
                    },
                }
            } else if step == CurrentStep::DeliberationSchedule || step == CurrentStep::EditContent {
                CompositionDeliberation {
                    lang,
                    deliberation_sequences: sequences,
                    check_sequence,
                    onadd: move |_| {
                        let _ = ctrl.add_deliberation_info();
                    },
                    onupdate: move |(index, opinion): (usize, StepCreateRequest)| {
                        let _ = ctrl.update_deliberation_info(index, opinion);
                    },
                    ondelete: move |index: usize| {
                        let _ = ctrl.delete_deliberation_info(index);
                    },
                    onstep: move |step: CurrentStep| {
                        ctrl.change_step(step);
                    },
                }
            } else {
                Preview {
                    lang,
                    onstep: move |step: CurrentStep| {
                        ctrl.change_step(step);
                    },
                }
            }
        }
    }
}
