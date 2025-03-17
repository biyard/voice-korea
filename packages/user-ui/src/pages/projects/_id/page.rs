use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::Language;
use models::Tab;

use crate::pages::projects::_id::{
    components::{
        basic_info::BasicInfo, deliberation::Deliberation, discussion::DiscussionPage,
        final_draft::FinalDraft, final_survey::FinalSurvey, project_header::ProjectHeader,
        sample_survey::SampleSurvey,
    },
    controller,
};

#[component]
pub fn ProjectPage(lang: Language, project_id: ReadOnlySignal<i64>) -> Element {
    let ctrl = controller::Controller::init(lang, project_id)?;
    let _comments = ctrl.comments()?;
    let deliberation = ctrl.summary()?;
    let active_tab = use_signal(|| Tab::BasicInfo);
    tracing::debug!("deliberation: {:?}", deliberation);

    rsx! {
        div { class: "flex flex-col w-full justify-center items-center",
            ProjectHeader { lang, deliberation, active_tab: active_tab.clone() }
            ProjectDetails { lang, active_tab: active_tab.clone(), project_id }
                // Comment { lang, comments }
        }
    }
}

#[component]
pub fn ProjectDetails(
    lang: Language,
    active_tab: Signal<Tab>,
    project_id: ReadOnlySignal<i64>,
) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-center items-center bg-[#F7F7F7]",
            div { class: "flex flex-col max-w-[1300px] w-full",
                match active_tab() {
                    Tab::BasicInfo => rsx! {
                        BasicInfo { lang, project_id }
                    },
                    Tab::SampleSurvey => rsx! {
                        SampleSurvey { lang, project_id }
                    },
                    Tab::Deliberation => rsx! {
                        Deliberation { lang, project_id }
                    },
                    Tab::Discussion => rsx! {
                        DiscussionPage { lang, project_id }
                    },
                    Tab::FinalSurvey => rsx! {
                        FinalSurvey { lang, project_id }
                    },
                    Tab::FinalDraft => rsx! {
                        FinalDraft { lang, project_id }
                    },
                }
            }
        }
    }
}
