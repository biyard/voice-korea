use dioxus::prelude::*;
use dioxus_translate::Language;

#[component]
pub fn UserLoginPage(lang: Language) -> Element {
    // let mut ctrl = controller::Controller::init();
    // let translates: CreateTranslate = translate(&props.lang.clone());

    rsx! {
        div { "LoginPage" }
    }
}
