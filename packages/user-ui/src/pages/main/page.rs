use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

use super::controller;
use super::i18n::Translate;
#[component]
pub fn MainPage(lang: Language) -> Element {
    let _ctrl = controller::Controller::init(lang.clone())?;
    let translates: Translate = translate(&lang);

    rsx! {
        //TODO: remove bg color
        div { class: "h-full bg-red-50", {translates.text} }
    }
}
