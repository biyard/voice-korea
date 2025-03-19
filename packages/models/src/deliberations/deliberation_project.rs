use bdk::prelude::*;

use crate::ProjectArea;

// TODO(web): using resource for project.
#[api_model(base = "/v2/projects", custom_query_type = ProjectQueryBy, table = deliberations)]
pub struct DeliberationProject {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    pub started_at: i64,
    pub ended_at: i64,

    #[api_model(summary, query_action = search)]
    pub title: String,
    #[api_model(summary)]
    pub description: String,
    #[api_model(summary)]
    pub project_area: ProjectArea,

    #[api_model(summary, many_to_one = organizations)]
    pub org_id: i64,
    #[api_model(summary, one_to_many = deliberation_users, foreign_key = deliberation_id, aggregator = count)]
    pub participants: i64,
    #[api_model(summary, one_to_many = deliberation_votes, foreign_key = deliberation_id, aggregator = count)]
    pub votes: i64,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct ProjectQueryBy {
    pub sorter: ProjectSorter,
}

#[derive(
    Debug, Clone, Copy, Eq, PartialEq, serde::Serialize, serde::Deserialize, Translate, Default,
)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum ProjectSorter {
    #[default]
    #[translate(ko => "오래된순")]
    Oldest,
    #[translate(ko => "최신순")]
    Newest,
}

impl DeliberationProject {
    pub fn period(&self) -> String {
        // TODO(web): returns Feb. 12, 2025 - Mar. 15, 2025

        todo!()
    }
}
