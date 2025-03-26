#![allow(unused)]
use by_macros::DioxusController;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::Language;
use models::{
    dto::ProfileData,
    profile::{DesignProject, ParticipantProject, ProfileSummary},
};

use crate::service::user_service::UserService;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProjectType {
    Design,
    Participation,
}

#[derive(Debug, Clone, Copy, DioxusController)]
pub struct Controller {
    lang: Language,
    pub projects: Resource<ProfileData>,
    pub user_id: i64,
    selected_type: Signal<ProjectType>,
    keyword: Signal<String>,
}

impl Controller {
    pub fn init(lang: Language) -> std::result::Result<Self, RenderError> {
        let user_service: UserService = use_context();

        let projects = use_server_future(move || async move {
            ProfileData::get_client(&crate::config::get().api_url)
                .find()
                .await
                .unwrap_or_default()
        })?;

        let user_id = (user_service.user_id)();

        let ctrl = Self {
            lang,
            projects,
            user_id,

            selected_type: use_signal(|| ProjectType::Design),
            keyword: use_signal(|| "".to_string()),
        };

        use_context_provider(|| ctrl);
        Ok(ctrl)
    }

    pub fn get_selected_type(&self) -> ProjectType {
        (self.selected_type)()
    }

    pub fn get_keyword(&self) -> String {
        (self.keyword)()
    }

    pub fn change_selected_type(&mut self, selected_type: ProjectType) {
        self.selected_type.set(selected_type);
    }

    pub fn change_keyword(&mut self, keyword: String) {
        self.keyword.set(keyword);
    }
}
