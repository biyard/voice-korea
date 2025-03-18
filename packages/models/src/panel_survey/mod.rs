#![allow(unused_variables)]
#[allow(unused)]
use crate::Result;
use bdk::prelude::*;
use by_types::QueryResponse;

#[api_model(base = "/panel-surveys/v2", table = panel_surveys, iter_type=QueryResponse)]
pub struct PanelSurveys {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,

    #[api_model(many_to_one = panels)]
    pub panel_id: i64,
    #[api_model(many_to_one = surveys)]
    pub survey_id: i64,
}
