#![allow(non_snake_case)]
use crate::pages::deliberations::new::composition_commitee::CompositionCommitee;
use crate::pages::deliberations::new::composition_deliberation::CompositionDeliberation;
use crate::pages::deliberations::new::composition_panel::CompositionPanel;
use crate::pages::deliberations::new::controller::MeetingInfo;
use crate::pages::deliberations::new::input_deliberation::InputDeliberation;
use crate::pages::deliberations::new::preview::Preview;
use crate::pages::deliberations::new::setting_discussion::SettingDiscussion;

use crate::{
    components::{icons::ArrowLeft, stepper::Stepper},
    pages::deliberations::new::controller::{Controller, CurrentStep},
    routes::Route,
};

use super::i18n::DeliberationNewTranslate;
use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::deliberation_user::DeliberationUserCreateRequest;
use models::step::StepCreateRequest;
use models::{File, PanelV2Summary, ResourceFileSummary, Role, SurveyV2Summary};

#[component]
pub fn OpinionCreatePage(lang: Language) -> Element {
    let translates: DeliberationNewTranslate = translate(&lang.clone());
    let mut ctrl = Controller::new(lang)?;
    let surveys = ctrl.surveys()?;
    let metadatas = ctrl.metadatas()?;
    let members = ctrl.members()?;
    let panels = ctrl.panels()?;

    let sequences = ctrl.get_deliberation_sequences();
    let informations = ctrl.get_deliberation_informations();
    let resources = ctrl.resources();
    let step = ctrl.get_current_step();
    let selected_surveys = ctrl.selected_surveys();
    let selected_panels = ctrl.get_selected_panels();
    let committees = ctrl.get_committees();
    let discussions = ctrl.get_discussions();
    let discussion_resources = ctrl.get_discussion_resources();

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "text-[#9b9b9b] font-medium text-[14px] mb-[10px]",
                "{translates.organization_management} / {translates.public_opinion_management}"
            }
            div { class: "flex flex-row w-full justify-start items-center mb-[25px]",
                Link { class: "mr-[6px]", to: Route::DeliberationPage { lang },
                    ArrowLeft { width: "24", height: "24", color: "#3a3a3a" }
                }
                div { class: "text-[#3a3a3a] font-semibold text-[28px] mr-[20px]",
                    "{translates.start_public_opinion}"
                }
            }

            div { class: "flex flex-col w-full justify-start items-center mt-[20px] mb-[80px]",
                div { class: "flex flex-row w-[1400px] min-w-[1400px] justify-center items-center",
                    Stepper {
                        current_step: if step == CurrentStep::PublicOpinionComposition { 1 } else if step == CurrentStep::InputInformation { 2 } else if step == CurrentStep::CommitteeComposition { 3 } else if step == CurrentStep::PanelComposition { 4 } else if step == CurrentStep::DiscussionSetting { 5 } else { 6 },
                        steps: vec![
                            "공론 구성 및 기간".to_string(),
                            "필수정보 입력".to_string(),
                            "공론 위원회 구성".to_string(),
                            "참여자 패널 구성".to_string(),
                            "토론 설정".to_string(),
                            "전체 미리보기".to_string(),
                        ],
                    }
                }
            }

            if step == CurrentStep::PublicOpinionComposition {
                CompositionDeliberation {
                    lang,
                    deliberation_sequences: ctrl.get_deliberation_sequences(),
                    check_sequence: ctrl.check_deliberation_info(),
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
            } else if step == CurrentStep::InputInformation {
                InputDeliberation {
                    lang,
                    resources,
                    surveys,
                    selected_surveys,
                    metadatas,
                    fields: ctrl.get_total_fields(),
                    information: ctrl.get_deliberation_informations(),

                    oncreate: move |file: File| async move {
                        let _ = ctrl.create_resource(file).await;
                    },
                    onremove: move |id: i64| {
                        let _ = ctrl.delete_resource(id);
                    },
                    onadd: move |resource: ResourceFileSummary| {
                        let _ = ctrl.add_resource(resource.into());
                    },
                    onstep: move |step: CurrentStep| {
                        ctrl.change_step(step);
                    },
                    update_projects: move |surveys: Vec<SurveyV2Summary>| {
                        ctrl.set_projects(surveys);
                    },
                    change_information: move |information| {
                        ctrl.update_deliberation_information(information);
                    },
                }
            } else if step == CurrentStep::CommitteeComposition {
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
            } else if step == CurrentStep::PanelComposition {
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
            } else if step == CurrentStep::DiscussionSetting {
                SettingDiscussion {
                    lang,
                    discussions,
                    add_discussion: move |_| {
                        ctrl.add_discussion();
                    },
                    remove_discussion: move |index: usize| {
                        ctrl.remove_discussion(index);
                    },
                    update_discussion: move |(index, discussion): (usize, MeetingInfo)| {
                        ctrl.update_discussion(index, discussion);
                    },

                    discussion_resources,
                    create_resource: move |file: File| async move {
                        let _ = ctrl.create_discussion_resource(file).await;
                    },
                    remove_resource: move |id: i64| {
                        let _ = ctrl.remove_discussion_resource(id);
                    },
                    clear_resource: move |_| {
                        let _ = ctrl.clear_discussion_resource();
                    },
                    onstep: move |step: CurrentStep| {
                        ctrl.change_step(step);
                    },
                }
            } else {
                Preview {
                    lang,
                    sequences,
                    informations,
                    committees,
                    members,
                    selected_panels,

                    onstep: move |step: CurrentStep| {
                        ctrl.change_step(step);
                    },
                    onsend: move |_| async move {
                        let _ = ctrl.create_deliberation(lang).await;
                    },
                }
            }
        }
    }
}
