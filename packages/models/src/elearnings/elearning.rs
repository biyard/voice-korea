#![allow(unused_variables, unused)]
use bdk::prelude::*;
use validator::Validate;

use crate::ResourceFile;

#[derive(Validate)]
#[api_model(base = "/v2/elearnings", table = elearnings)]
pub struct Elearning {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, many_to_one = deliberation_contents)]
    pub content_id: i64,

    #[api_model(summary, action = create, action_by_id = update)]
    pub title: String,
    #[api_model(summary, action = create, type = JSONB, action_by_id = update)]
    #[serde(default)]
    pub resources: Vec<ResourceFile>,
    #[api_model(summary, action = create, action_by_id = update)]
    pub necessary: bool,
}
