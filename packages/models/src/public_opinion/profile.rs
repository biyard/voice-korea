#![allow(unused)]

#[allow(unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use by_types::QueryResponse;
use dioxus_translate::Language;

#[api_model(base = "/profile", table = profile, iter_type=QueryResponse)]
pub struct Profile {
    #[api_model(summary, primary_key, action = delete )]
    pub id: i64,
    #[api_model(summary, auto = insert)]
    pub created_at: i64,
    #[api_model(auto = [insert, update], summary)]
    pub updated_at: i64,

    #[api_model(summary, action = create, action_by_id = update)]
    pub num_of_projects: i64,
    #[api_model(summary, action = create, action_by_id = update)]
    pub num_of_votes: i64,
    #[api_model(summary, action = create, action_by_id = update)]
    pub num_of_tokens: i64,

    #[api_model(summary, action = create, action_by_id = update)]
    pub image: String,
    #[api_model(summary, action = create, action_by_id = update)]
    pub address: String,

    #[api_model(summary, action = create, type = JSONB, version = v0.1, action_by_id = update)]
    pub designed_projects: Vec<DesignProject>,
    #[api_model(summary, action = create, type = JSONB, version = v0.1, action_by_id = update)]
    pub participant_projects: Vec<ParticipantProject>,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct ParticipantProject {
    pub id: i64, //project의 id
    pub title: String,
    pub creator: String,
    pub num_of_participation: i64,
    pub created_at: i64,
    pub updated_at: i64,
    pub status: ProjectStatus,
    // vote: Vote,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct DesignProject {
    pub id: i64, //project의 id
    pub title: String,
    pub role: Option<Role>,
    pub institution_name: String,
    pub num_of_participation: i64,
    pub created_at: i64,
    pub updated_at: i64,
    pub status: ProjectStatus,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum ProjectStatus {
    Inprogress = 0,
    Withdrawal = 1,
    Waiting = 2,
    Adopted = 3,
}

impl ProjectStatus {
    pub fn to_type(&self, lang: &Language) -> String {
        match (&self, lang) {
            (&ProjectStatus::Inprogress, &Language::En) => "In Progress".to_string(),
            (&ProjectStatus::Inprogress, &Language::Ko) => "진행중".to_string(),

            (&ProjectStatus::Withdrawal, &Language::En) => "Withdrawal".to_string(),
            (&ProjectStatus::Withdrawal, &Language::Ko) => "철회".to_string(),

            (&ProjectStatus::Waiting, &Language::En) => "Waiting".to_string(),
            (&ProjectStatus::Waiting, &Language::Ko) => "대기".to_string(),

            (&ProjectStatus::Adopted, &Language::En) => "Adopted".to_string(),
            (&ProjectStatus::Adopted, &Language::Ko) => "채택".to_string(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum Role {
    Admin = 0,
    PublicAdmin = 1,
    Analyst = 2,
    Mediator = 3,
    Speaker = 4,
}

impl Role {
    pub fn to_type(&self, lang: &Language) -> String {
        match (&self, lang) {
            (&Role::Admin, &Language::En) => "Admin".to_string(),
            (&Role::Admin, &Language::Ko) => "관리자".to_string(),

            (&Role::PublicAdmin, &Language::En) => "Public Opinion Admin".to_string(),
            (&Role::PublicAdmin, &Language::Ko) => "공론 관리자".to_string(),

            (&Role::Analyst, &Language::En) => "Analyst".to_string(),
            (&Role::Analyst, &Language::Ko) => "분석가".to_string(),

            (&Role::Mediator, &Language::En) => "Mediator".to_string(),
            (&Role::Mediator, &Language::Ko) => "중계자".to_string(),

            (&Role::Speaker, &Language::En) => "Speaker".to_string(),
            (&Role::Speaker, &Language::Ko) => "강연자".to_string(),
        }
    }
}
