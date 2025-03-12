use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::Language;
use models::Tab;

use crate::pages::projects::_id::{
    components::{
        basic_info::BasicInfo, comments::Comment, deliberation::Deliberation,
        discussion::Discussion, final_draft::FinalDraft, final_survey::FinalSurvey,
        project_header::ProjectHeader, sample_survey::SampleSurvey,
    },
    controller,
};

#[component]
pub fn ProjectPage(lang: Language, project_id: ReadOnlySignal<i64>) -> Element {
    let ctrl = controller::Controller::init(lang, project_id)?;
    let deliberation = ctrl.get_deliberation();
    let active_tab = use_signal(|| Tab::BasicInfo);
    tracing::debug!("deliberation: {:?}", deliberation);

    rsx! {
        div {
            ProjectHeader { lang, deliberation, active_tab: active_tab.clone() }
            ProjectDetails { lang, active_tab: active_tab.clone(), project_id }
            Comment { lang }
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
        div { class: "w-full bg-[#F7F7F7]",
            match *active_tab.read() {
                Tab::BasicInfo => rsx! {
                    BasicInfo { lang }
                },
                Tab::SampleSurvey => rsx! {
                    SampleSurvey { lang, project_id }
                },
                Tab::Deliberation => rsx! {
                    Deliberation { lang, project_id }
                },
                Tab::Discussion => rsx! {
                    Discussion { lang, project_id }
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
