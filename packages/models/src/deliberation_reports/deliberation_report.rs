#![allow(dead_code, unused)]
use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v2/organizations/:org-id/drafts", table = deliberation_reports)]
pub struct DeliberationReport {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, many_to_one = organizations)]
    pub org_id: i64,
    #[api_model(summary, many_to_one = deliberations, action = create)]
    pub deliberation_id: i64,
    #[api_model(summary, many_to_one = users)]
    pub user_id: i64,

    #[api_model(summary, action = create, action_by_id = update)]
    pub title: String,
    #[api_model(summary, action = create, action_by_id = update)]
    pub description: String,

    #[api_model(summary, type = INTEGER, action = create)]
    #[serde(default)]
    pub status: DeliberationReportStatus,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum DeliberationReportStatus {
    #[default]
    #[translate(ko = "초안", en = "Draft")]
    Draft = 1,
}
