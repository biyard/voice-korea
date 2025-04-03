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
use models::deliberation::DeliberationCreateRequest;
use models::Role;

#[component]
pub fn OpinionCreatePage(lang: Language) -> Element {
    let tr: DeliberationNewTranslate = translate(&lang.clone());
    let mut ctrl = Controller::new(lang)?;

    let step = ctrl.get_current_step();

    let req = ctrl.deliberation_requests();

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

            SettingDeliberation {
                lang,
                visibility: step == CurrentStep::SettingInfo,
                onstep: move |step: CurrentStep| {
                    ctrl.change_step(step);
                },
            }

            CompositionCommitee {
                lang,
                visibility: step == CurrentStep::CompositionCommittee,
                roles: vec![
                    Role::Admin,
                    Role::DeliberationAdmin,
                    Role::Analyst,
                    Role::Moderator,
                    Role::Speaker,
                ],
                req: req.clone(),
                onprev: move |(req, step): (DeliberationCreateRequest, CurrentStep)| {
                    ctrl.change_request(req);
                    ctrl.change_step(step);
                },
                onnext: move |(req, step): (DeliberationCreateRequest, CurrentStep)| {
                    ctrl.change_request(req);
                    ctrl.change_step(step);
                },
            }

            CompositionPanel {
                lang,
                visibility: step == CurrentStep::CompositionPanel,
                req: req.clone(),
                onprev: move |(req, step): (DeliberationCreateRequest, CurrentStep)| {
                    ctrl.change_request(req);
                    ctrl.change_step(step);
                },
                onnext: move |(req, step): (DeliberationCreateRequest, CurrentStep)| {
                    ctrl.change_request(req);
                    ctrl.change_step(step);
                },
            }

            CompositionDeliberation {
                lang,
                visibility: step == CurrentStep::DeliberationSchedule || step == CurrentStep::EditContent,
                req: req.clone(),
                onprev: move |(req, step): (DeliberationCreateRequest, CurrentStep)| {
                    ctrl.change_request(req);
                    ctrl.change_step(step);
                },
                onnext: move |(req, step): (DeliberationCreateRequest, CurrentStep)| {
                    ctrl.change_request(req);
                    ctrl.change_step(step);
                },
            }

            Preview {
                lang,
                visibility: step == CurrentStep::Preview,
                onstep: move |step: CurrentStep| {
                    ctrl.change_step(step);
                },
            }
        }
    }
}
