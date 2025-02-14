#![allow(unused)]
use dioxus::prelude::*;
use dioxus_translate::Language;

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    lang: Language,
}

impl Controller {
    pub fn init(lang: Language) -> std::result::Result<Self, RenderError> {
        let ctrl = Self { lang };

        use_context_provider(|| ctrl);
        Ok(ctrl)
    }
}
