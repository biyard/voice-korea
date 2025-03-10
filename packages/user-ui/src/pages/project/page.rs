use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::Language;

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

    tracing::debug!("deliberation: {:?}", deliberation);

    rsx! {
        div {
            ProjectProfile { lang, deliberation }
            ProjectMenu { lang }
            ProjectDetails { lang }
            Comment { lang }
        }
    }
}
