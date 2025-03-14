#![allow(non_snake_case, dead_code, unused_variables)]
use by_macros::*;
use dioxus::prelude::*;
use dioxus_translate::*;

#[component]
pub fn SampleSurvey(
    lang: Language,
    project_id: ReadOnlySignal<i64>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let ctrl = Controller::new(lang, project_id)?;
    let tr: BasicInfoTranslate = translate(&lang);

    rsx! {
        div { id: "sample-survey", ..attributes,
            {tr.title}
            {children}
        }
    }
}

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    project_id: ReadOnlySignal<i64>,
}

impl Controller {
    pub fn new(
        lang: Language,
        project_id: ReadOnlySignal<i64>,
    ) -> std::result::Result<Self, RenderError> {
        let ctrl = Self { lang, project_id };

        Ok(ctrl)
    }
}

translate! {
    BasicInfoTranslate;

    title: {
        ko: "BasicInfo",
        en: "BasicInfo",
    },
}
