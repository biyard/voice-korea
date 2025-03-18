#![allow(unused_variables)]
#[allow(unused)]
use crate::Result;
use bdk::prelude::*;
use by_types::QueryResponse;

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct PanelCountsV2 {
    pub created_at: i64,
    pub updated_at: i64,

    pub panel_id: i64,
    pub panel_survey_id: i64,
    pub user_count: i64,
}

#[api_model(base = "/panel-counts-surveys/v2", table = panel_counts_surveys, iter_type=QueryResponse)]
pub struct PanelCountSurveys {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,

    #[api_model(many_to_one = surveys)]
    pub survey_id: i64,
    #[api_model(many_to_one = panel_counts)]
    pub panel_counts_id: i64,
}
