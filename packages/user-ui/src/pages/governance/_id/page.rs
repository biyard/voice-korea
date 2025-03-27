use dioxus::prelude::*;
use dioxus_translate::Language;

use crate::pages::governance::_id::controller;

#[component]
pub fn GovernancePage(lang: Language, governance_id: i64) -> Element {
    let ctrl = controller::Controller::init(lang.clone())?;
    let _institution = ctrl.get_public_opinion_institution();

    rsx! {
        div { class: "flex flex-col w-full justify-center items-center mt-80",
            div { class: "max-w-1300 mt-60 flex flex-col w-full justify-start items-start",
                "governance page"
            }
        }
    }
}
