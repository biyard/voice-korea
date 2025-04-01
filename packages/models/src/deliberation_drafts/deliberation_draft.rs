#![allow(unused_variables, unused)]
use crate::deliberation_user::DeliberationUser;
use crate::ResourceFile;
use crate::SurveyV2;
use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v2/deliberation-drafts", table = deliberation_drafts)]
pub struct DeliberationDraft {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    // started_at indicates the start time of the deliberation.
    #[api_model(summary, action = create)]
    pub started_at: i64,
    // ended_at indicates the end time of the deliberation.
    #[api_model(summary, action = create)]
    pub ended_at: i64,

    #[api_model(summary, action = create, action_by_id = update)]
    pub title: String,
    #[api_model(summary, action = create, action_by_id = update)]
    pub description: String,

    #[api_model(summary, many_to_one = deliberations)]
    pub deliberation_id: i64,

    #[api_model(summary, many_to_many = draft_members, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = draft_id)]
    #[serde(default)]
    pub members: Vec<DeliberationUser>,

    #[api_model(summary, many_to_many = draft_resources, foreign_table_name = resources, foreign_primary_key = resource_id, foreign_reference_key = draft_id)]
    #[serde(default)]
    pub resources: Vec<ResourceFile>,

    #[api_model(summary, many_to_many = draft_surveys, foreign_table_name = surveys, foreign_primary_key = survey_id, foreign_reference_key = draft_id)]
    #[serde(default)]
    pub surveys: Vec<SurveyV2>,
}
