#![allow(unused_variables, unused)]
use crate::ProjectArea;
use bdk::prelude::*;
use validator::Validate;

// NOTE: comments read only model
#[derive(Validate)]
#[api_model(base = "/v2/deliberation-areas", table = areas)]
pub struct Area {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, type = INTEGER, action = create)]
    #[serde(default)]
    pub project_area: ProjectArea,
}
