use chrono::{TimeZone, Utc};
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::Language;
use models::prelude::{OpinionResponse, PanelInfo, ProjectField, ProjectStatus};
use serde::{Deserialize, Serialize};

use crate::service::opinion_api::OpinionApi;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Opinion {
    pub project_id: String,
    pub opinion_type: String,
    pub project_name: String,
    pub total_response_count: u64,
    pub response_count: u64,
    pub panels: Vec<PanelInfo>,
    pub start_date: String,
    pub end_date: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    pub opinion_resource:
        Resource<Result<crate::api::common::CommonQueryResponse<OpinionResponse>, ServerFnError>>,
    pub opinions: Signal<Vec<Opinion>>,
    pub project_types: Signal<Vec<String>>,
    pub project_statuses: Signal<Vec<String>>,
}

impl Controller {
    pub fn init(lang: dioxus_translate::Language) -> Self {
        let api: OpinionApi = use_context();
        let opinion_resource: Resource<
            Result<crate::api::common::CommonQueryResponse<OpinionResponse>, ServerFnError>,
        > = use_resource(move || {
            let api = api.clone();
            async move { api.list_opinions(Some(100), None).await }
        });
        let mut ctrl = Self {
            opinion_resource,
            opinions: use_signal(|| vec![]),
            project_types: use_signal(|| {
                vec![
                    "경제".to_string(),
                    "사회".to_string(),
                    "환경".to_string(),
                    "교육".to_string(),
                    "문화".to_string(),
                    "노동".to_string(),
                    "도시".to_string(),
                    "기술".to_string(),
                    "보건".to_string(),
                    "정치".to_string(),
                ]
            }),
            project_statuses: use_signal(|| {
                vec!["준비".to_string(), "진행".to_string(), "마감".to_string()]
            }),
        };

        let opinions = if let Some(v) = opinion_resource.value()() {
            match v {
                Ok(d) => {
                    let mut items = vec![];

                    for item in d.items {
                        items.push(Opinion {
                            project_id: item.project_id.clone(),
                            opinion_type: ctrl
                                .opinion_field_type_translate(lang.clone(), item.opinion_type)
                                .to_string(),
                            project_name: item.project_name.clone(),
                            total_response_count: item.total_response_count,
                            response_count: item.response_count,
                            panels: item.panels,
                            start_date: ctrl.format_timestamp(item.start_date as i64),
                            end_date: ctrl.format_timestamp(item.end_date as i64),
                            status: ctrl
                                .project_status_translate(lang.clone(), item.status)
                                .to_string(),
                        });
                    }

                    items
                }
                Err(e) => {
                    tracing::error!("Failed to fetch opinion: {:?}", e);
                    vec![]
                }
            }
        } else {
            vec![]
        };

        ctrl.opinions.set(opinions);
        ctrl
    }

    pub fn get_project_types(&self) -> Vec<String> {
        (self.project_types)()
    }

    pub fn get_project_statuses(&self) -> Vec<String> {
        (self.project_statuses)()
    }

    pub fn get_opinions(&self) -> Vec<Opinion> {
        (self.opinions)()
    }

    fn format_timestamp(&self, timestamp: i64) -> String {
        let datetime = Utc.timestamp_opt(timestamp, 0).unwrap();
        datetime.format("%Y.%m.%d").to_string()
    }

    fn project_status_translate(&self, lang: Language, status: ProjectStatus) -> &'static str {
        match lang {
            Language::En => match status {
                ProjectStatus::Ready => "Ready",
                ProjectStatus::InProgress => "In Progress",
                ProjectStatus::Finish => "Finish",
            },
            Language::Ko => match status {
                ProjectStatus::Ready => "준비",
                ProjectStatus::InProgress => "진행",
                ProjectStatus::Finish => "마감",
            },
        }
    }

    pub fn opinion_field_type_translate(
        &self,
        lang: Language,
        opinion_type: ProjectField,
    ) -> &'static str {
        match lang {
            Language::En => match opinion_type {
                ProjectField::Economy => "Economy",
                ProjectField::Society => "Society",
                ProjectField::Environment => "Environment",
                ProjectField::Education => "Education",
                ProjectField::Culture => "Culture",
                ProjectField::Labor => "Labor",
                ProjectField::City => "City",
                ProjectField::Technology => "Technology",
                ProjectField::Health => "Health",
                ProjectField::Politics => "Politics",
            },
            Language::Ko => match opinion_type {
                ProjectField::Economy => "경제",
                ProjectField::Society => "사회",
                ProjectField::Environment => "환경",
                ProjectField::Education => "교육",
                ProjectField::Culture => "문화",
                ProjectField::Labor => "노동",
                ProjectField::City => "도시",
                ProjectField::Technology => "기술",
                ProjectField::Health => "보건",
                ProjectField::Politics => "정치",
            },
        }
    }
}
