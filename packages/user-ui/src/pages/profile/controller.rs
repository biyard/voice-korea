#![allow(unused)]
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::Language;
use models::{
    dto::ProfileProjectsData,
    profile::{DesignProject, ParticipantProject, ProfileSummary},
};

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
        let projects = use_server_future(move || async move {
            ProfileProjectsData::get_client(&crate::config::get().api_url)
                .find_one()
                .await
                .unwrap_or_default()
        })?;

        tracing::debug!("projects: {:?}", projects);

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
                designed_projects: vec![
                    DesignProject {
                        id: 1,
                        title: "공론조사 제목입니다.".to_string(),
                        role: Some(models::profile::Role::Analyst),
                        institution_name: "개인".to_string(),
                        num_of_participation: 1201,
                        created_at: 1737759492,
                        updated_at: 1737759492,
                        status: models::profile::ProjectStatus::Adopted,
                    },
                    DesignProject {
                        id: 2,
                        title: "공론조사 제목입니다.".to_string(),
                        role: Some(models::profile::Role::PublicAdmin),
                        institution_name: "개인".to_string(),
                        num_of_participation: 1201,
                        created_at: 1737759492,
                        updated_at: 1737759492,
                        status: models::profile::ProjectStatus::Inprogress,
                    },
                    DesignProject {
                        id: 3,
                        title: "공론조사 제목입니다.".to_string(),
                        role: Some(models::profile::Role::Admin),
                        institution_name: "개인".to_string(),
                        num_of_participation: 1201,
                        created_at: 1737759492,
                        updated_at: 1737759492,
                        status: models::profile::ProjectStatus::Waiting,
                    },
                    DesignProject {
                        id: 4,
                        title: "공론조사 제목입니다.".to_string(),
                        role: Some(models::profile::Role::Speaker),
                        institution_name: "개인".to_string(),
                        num_of_participation: 1201,
                        created_at: 1737759492,
                        updated_at: 1737759492,
                        status: models::profile::ProjectStatus::Withdrawal,
                    },
                    DesignProject {
                        id: 5,
                        title: "공론조사 제목입니다.".to_string(),
                        role: Some(models::profile::Role::Mediator),
                        institution_name: "개인".to_string(),
                        num_of_participation: 1201,
                        created_at: 1737759492,
                        updated_at: 1737759492,
                        status: models::profile::ProjectStatus::Inprogress,
                    },
                ],
                participant_projects: vec![
                    ParticipantProject {
                        id: 1,
                        title: "공론조사 제목입니다".to_string(),
                        creator: "bd77b".to_string(),
                        num_of_participation: 1201,
                        created_at: 1737759492,
                        updated_at: 1737759492,
                        status: models::profile::ProjectStatus::Inprogress,
                    },
                    ParticipantProject {
                        id: 2,
                        title: "공론조사 제목입니다".to_string(),
                        creator: "bd77b".to_string(),
                        num_of_participation: 1201,
                        created_at: 1737759492,
                        updated_at: 1737759492,
                        status: models::profile::ProjectStatus::Waiting,
                    },
                    ParticipantProject {
                        id: 3,
                        title: "공론조사 제목입니다".to_string(),
                        creator: "bd77b".to_string(),
                        num_of_participation: 1201,
                        created_at: 1737759492,
                        updated_at: 1737759492,
                        status: models::profile::ProjectStatus::Withdrawal,
                    },
                    ParticipantProject {
                        id: 4,
                        title: "공론조사 제목입니다".to_string(),
                        creator: "bd77b".to_string(),
                        num_of_participation: 1201,
                        created_at: 1737759492,
                        updated_at: 1737759492,
                        status: models::profile::ProjectStatus::Adopted,
                    },
                    ParticipantProject {
                        id: 5,
                        title: "공론조사 제목입니다".to_string(),
                        creator: "bd77b".to_string(),
                        num_of_participation: 1201,
                        created_at: 1737759492,
                        updated_at: 1737759492,
                        status: models::profile::ProjectStatus::Inprogress,
                    },
                ],
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
