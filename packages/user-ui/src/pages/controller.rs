#![allow(unused)]
use by_macros::DioxusController;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::Language;
use models::dto::LandingData;

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,

    data: Resource<LandingData>,
}

impl Controller {
    pub fn init(lang: Language) -> std::result::Result<Self, RenderError> {
        let data = use_server_future(move || async move {
            LandingData::get_client(&crate::config::get().api_url)
                .find_one()
                .await
                .unwrap_or_default()
        })?;

        let ctrl = Self { lang, data };

        use_context_provider(|| ctrl);
        Ok(ctrl)
    }

    pub fn send_inquiry(&self, name: String, email: String, message: String) {
        tracing::debug!(
            "send inquiry button clicked: {} {} {}",
            name,
            email,
            message
        );
    }
}
