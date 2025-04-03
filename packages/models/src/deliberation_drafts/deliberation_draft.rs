#![allow(unused_variables, unused)]
use crate::deliberation_response::DeliberationResponse;
use crate::deliberation_user::DeliberationUser;
use crate::ResourceFile;
use crate::SurveyV2;
use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v2/deliberations/:deliberation-id/drafts", table = deliberation_drafts, action = [create(users = Vec<i64>, resources = Vec<i64>, surveys = Vec<i64>)])]
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

    #[api_model(summary, many_to_many = deliberation_draft_members, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = draft_id)]
    #[serde(default)]
    pub members: Vec<DeliberationUser>,

    #[api_model(summary, many_to_many = deliberation_draft_resources, foreign_table_name = resources, foreign_primary_key = resource_id, foreign_reference_key = draft_id)]
    #[serde(default)]
    pub resources: Vec<ResourceFile>,

    #[api_model(summary, many_to_many = deliberation_draft_surveys, foreign_table_name = surveys, foreign_primary_key = survey_id, foreign_reference_key = draft_id)]
    #[serde(default)]
    pub surveys: Vec<SurveyV2>,

    // responses is a list of responses of a user(requester) for surveys.
    #[api_model(summary, one_to_many = deliberation_responses, foreign_key = deliberation_id, reference_key = deliberation_id)]
    #[serde(default)]
    pub responses: Vec<DeliberationResponse>,
}
