use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::Language;
use models::Tab;

use crate::pages::project::{
    components::{
        comments::Comment, project_details::ProjectDetails, project_menu::ProjectMenu,
        project_profile::ProjectProfile,
    },
    controller,
};

#[component]
pub fn ProjectPage(lang: Language, project_id: i64) -> Element {
    let ctrl = controller::Controller::init(lang)?;
    let deliberation = ctrl.get_deliberation();
    let active_tab = use_signal(|| Tab::Details);
    tracing::debug!("deliberation: {:?}", deliberation);

    rsx! {
        div {
            ProjectProfile { lang, deliberation }
            ProjectMenu { lang, active_tab: active_tab.clone() }
            ProjectDetails { lang, active_tab: active_tab.clone() }
            Comment { lang }
        }
    }
}
