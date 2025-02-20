use dioxus::prelude::*;
use dioxus_translate::Language;

use crate::pages::governance::controller;

#[component]
pub fn GovernancePage(lang: Language, governance_id: i64) -> Element {
    let ctrl = controller::Controller::init(lang.clone())?;
    let _institution = ctrl.get_public_opinion_institution();

    rsx! {
        div { "governance page" }
    }
}
