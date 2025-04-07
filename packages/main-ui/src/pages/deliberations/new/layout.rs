#![allow(non_snake_case)]
use crate::pages::deliberations::new::i18n::DeliberationNewTranslate;
use crate::{
    components::{icons::ArrowLeft, stepper::Stepper},
    pages::deliberations::new::controller::{Controller, DeliberationNewStep},
    routes::Route,
};

// use super::routes::DeliberationNewRoute;
use by_components::loaders::cube_loader::CubeLoader;
use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

#[component]
pub fn DeliberationNewLayout(lang: Language) -> Element {
    let tr: DeliberationNewTranslate = translate(&lang.clone());
    let ctrl = Controller::new(lang)?;

    let step = ctrl.get_current_step();

    let _req = ctrl.deliberation_requests();

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div {
                class: format!(
                    "flex flex-col w-full justify-start items-start {}",
                    if step == DeliberationNewStep::EditContent { "hidden" } else { "" },
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
                    div { class: "flex flex-row w-full justify-center items-center",
                        Stepper {
                            current_step: if step == DeliberationNewStep::SettingInfo { 1 } else if step == DeliberationNewStep::CompositionCommittee { 2 } else if step == DeliberationNewStep::CompositionPanel { 3 } else if step == DeliberationNewStep::DeliberationSchedule
    || step == DeliberationNewStep::EditContent { 4 } else { 5 },
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

            SuspenseBoundary {
                fallback: |_| rsx! {
                    div { class: "absolute w-screen h-screen top-0 left-0 flex items-center justify-center text-white",
                        CubeLoader {}
                    }
                },
                div { class: "flex flex-col grow w-full bg-[#f0f2fc] px-[60px] pt-[25px]",
                    Outlet::<Route> {}
                }
            }

        // SettingDeliberation {
        //     lang,
        //     visibility: step == CurrentStep::SettingInfo,
        //     onstep: move |step: CurrentStep| {
        //         ctrl.change_step(step);
        //     },
        // }

        // CompositionCommitee {
        //     lang,
        //     visibility: step == CurrentStep::CompositionCommittee,
        //     roles: vec![
        //         Role::Admin,
        //         Role::DeliberationAdmin,
        //         Role::Analyst,
        //         Role::Moderator,
        //         Role::Speaker,
        //     ],
        //     req: req.clone(),
        //     onprev: move |(req, step): (DeliberationCreateRequest, CurrentStep)| {
        //         ctrl.change_request(req);
        //         ctrl.change_step(step);
        //     },
        //     onnext: move |(req, step): (DeliberationCreateRequest, CurrentStep)| {
        //         ctrl.change_request(req);
        //         ctrl.change_step(step);
        //     },
        // }

        // CompositionPanel {
        //     lang,
        //     visibility: step == CurrentStep::CompositionPanel,
        //     req: req.clone(),
        //     onprev: move |(req, step): (DeliberationCreateRequest, CurrentStep)| {
        //         ctrl.change_request(req);
        //         ctrl.change_step(step);
        //     },
        //     onnext: move |(req, step): (DeliberationCreateRequest, CurrentStep)| {
        //         ctrl.change_request(req);
        //         ctrl.change_step(step);
        //     },
        // }

        // CompositionDeliberation {
        //     lang,
        //     visibility: step == CurrentStep::DeliberationSchedule || step == CurrentStep::EditContent,
        //     req: req.clone(),
        //     onprev: move |(req, step): (DeliberationCreateRequest, CurrentStep)| {
        //         ctrl.change_request(req);
        //         ctrl.change_step(step);
        //     },
        //     onnext: move |(req, step): (DeliberationCreateRequest, CurrentStep)| {
        //         ctrl.change_request(req);
        //         ctrl.change_step(step);
        //     },
        // }

        // Preview {
        //     lang,
        //     visibility: step == CurrentStep::Preview,
        //     onstep: move |step: CurrentStep| {
        //         ctrl.change_step(step);
        //     },
        // }
        }
    }
}
