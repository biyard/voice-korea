use crate::by_components::loaders::cube_loader::CubeLoader;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::Language;
use models::Tab;

use crate::pages::projects::_id::{
    components::{
        basic_info::BasicInfo, comments::Comment, deliberation::Deliberation,
        discussion::DiscussionPage, final_draft::FinalDraft, final_survey::FinalSurvey,
        project_header::ProjectHeader, sample_survey::SampleSurvey,
    },
    controller,
};

#[component]
pub fn ProjectPage(lang: Language, project_id: ReadOnlySignal<i64>) -> Element {
    let mut ctrl = controller::Controller::init(lang, project_id)?;
    let comments = ctrl.comment_trees();
    let deliberation = ctrl.summary()?;
    let active_tab = use_signal(|| Tab::BasicInfo);
    tracing::debug!("deliberation: {:?}", deliberation);

    rsx! {
        // TODO(mobile): tab view implemented to fit mobile size
        div { class: "flex flex-col w-full justify-center items-center mt-80",
            ProjectHeader { lang, deliberation, active_tab: active_tab.clone() }
            div { class: "w-full flex flex-col justify-center items-center",
                SuspenseBoundary {
                    fallback: |_| rsx! {
                        div { class: "w-full h-fit flex items-center justify-center", CubeLoader {} }
                    },
                    div { class: "flex flex-col w-full h-fit",
                        ProjectDetails {
                            lang,
                            active_tab: active_tab.clone(),
                            project_id,
                        }
                    }
                }
            }
            Comment {
                lang,
                comments,
                send_comment: move |comment: String| async move {
                    let _ = ctrl.send_comment(comment).await;
                },
                like_comment: move |id: i64| async move {
                    let _ = ctrl.like_comment(id).await;
                },
                send_reply: move |(id, reply): (i64, String)| async move {
                    let _ = ctrl.send_reply(id, reply).await;
                },
            }
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
        div { class: "flex flex-col w-full justify-center items-center bg-box-gray",
            div { class: "flex flex-col max-w-1300 w-full",
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
