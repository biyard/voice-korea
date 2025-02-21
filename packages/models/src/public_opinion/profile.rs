#![allow(unused)]

#[allow(unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use by_types::QueryResponse;

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
    id: i64, //project의 id
    title: String,
    creator: String,
    num_of_participation: i64,
    created_at: i64,
    updated_at: i64,
    status: ProjectStatus,
    // vote: Vote,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct DesignProject {
    id: i64, //project의 id
    title: String,
    role: Option<Role>,
    institution_name: String,
    num_of_participation: i64,
    created_at: i64,
    updated_at: i64,
    status: ProjectStatus,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum ProjectStatus {
    Admin = 0,
    PublicAdmin = 1,
    Analyst = 2,
    Mediator = 3,
    Speaker = 4,
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
