#![allow(non_snake_case)]
use crate::pages::deliberations::new::composition_commitee::CompositionCommitee;
use crate::pages::deliberations::new::composition_opinion::CompositionOpinion;
use crate::pages::deliberations::new::composition_panel::CompositionPanel;
use crate::pages::deliberations::new::input_opinion::InputOpinion;
use crate::pages::deliberations::new::preview::Preview;
use crate::pages::deliberations::new::setting_discussion::SettingDiscussion;

use crate::{
    components::{icons::ArrowLeft, stepper::Stepper},
    pages::deliberations::new::controller::{Controller, CurrentStep},
    routes::Route,
};

use super::i18n::OpinionNewTranslate;
use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::File;

#[derive(Props, Clone, PartialEq)]
pub struct OpinionProps {
    lang: Language,
}

#[component]
pub fn OpinionCreatePage(props: OpinionProps) -> Element {
    let translates: OpinionNewTranslate = translate(&props.lang.clone());
    let mut ctrl = Controller::new(props.lang)?;
    let surveys = ctrl.surveys()?;

    let step = ctrl.get_current_step();
    let resources = ctrl.resources();

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "text-[#9b9b9b] font-medium text-[14px] mb-[10px]",
                "{translates.organization_management} / {translates.public_opinion_management}"
            }
            div { class: "flex flex-row w-full justify-start items-center mb-[25px]",
                Link {
                    class: "mr-[6px]",
                    to: Route::DeliberationPage {
                        lang: props.lang,
                    },
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
                CompositionOpinion { lang: props.lang.clone() }
            } else if step == CurrentStep::InputInformation {
                InputOpinion {
                    lang: props.lang.clone(),
                    resources,
                    surveys,
                    oncreate: move |file: File| async move {
                        let _ = ctrl.create_resource(file).await;
                    },
                    onremove: move |id: i64| {
                        let _ = ctrl.delete_resource(id);
                    },
                    onstep: move |step: CurrentStep| {
                        ctrl.change_step(step);
                    },
                }
            } else if step == CurrentStep::CommitteeComposition {
                CompositionCommitee { lang: props.lang.clone() }
            } else if step == CurrentStep::PanelComposition {
                CompositionPanel { lang: props.lang.clone() }
            } else if step == CurrentStep::DiscussionSetting {
                SettingDiscussion { lang: props.lang.clone() }
            } else {
                Preview { lang: props.lang.clone() }
            }
        }
    }
}
