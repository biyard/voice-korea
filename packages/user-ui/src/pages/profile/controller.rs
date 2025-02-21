#![allow(unused)]
use dioxus::prelude::*;
use dioxus_translate::Language;
use models::profile::ProfileSummary;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProjectType {
    Design,
    Participation,
}

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    lang: Language,
    profile: Signal<ProfileSummary>,

    selected_type: Signal<ProjectType>,
    keyword: Signal<String>,
}

impl Controller {
    pub fn init(lang: Language) -> std::result::Result<Self, RenderError> {
        let ctrl = Self {
            lang,
            profile: use_signal(|| ProfileSummary {
                id: 1,
                created_at: 1737759492,
                updated_at: 1737759492,
                num_of_projects: 15,
                num_of_votes: 39,
                num_of_tokens: 5600,
                image: "".to_string(),
                address: "0xb4c0...6f13".to_string(),
                designed_projects: vec![],
                participant_projects: vec![],
            }),
            selected_type: use_signal(|| ProjectType::Design),
            keyword: use_signal(|| "".to_string()),
        };

        use_context_provider(|| ctrl);
        Ok(ctrl)
    }

    pub fn get_profile(&self) -> ProfileSummary {
        (self.profile)()
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
