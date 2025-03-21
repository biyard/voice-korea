use bdk::prelude::*;

use crate::{
    deliberation_response::DeliberationResponse, deliberation_user::DeliberationUser, step::Step,
    SurveyV2,
};

// TODO(api): implement survey response for Sample, final Survey.
// TODO(web): using resource for Sample survey tab on a project.
// TODO(web): using resource for final survey tab on a project.
#[api_model(base = "/v2/projects/:deliberation-id/surveys", table = deliberations, read_action = read)]
pub struct DeliberationSurvey {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary)]
    pub started_at: i64,
    #[api_model(summary)]
    pub ended_at: i64,
    pub title: String,
    pub description: String,
    #[api_model(one_to_many = deliberation_users, foreign_key = deliberation_id)]
    pub members: Vec<DeliberationUser>,

    #[api_model(many_to_many = deliberation_surveys, table_name = surveys, foreign_primary_key = survey_id, foreign_reference_key = deliberation_id)]
    pub surveys: Vec<SurveyV2>,

    #[api_model(summary, one_to_many = deliberations_steps, foreign_key = deliberation_id)]
    #[serde(default)]
    pub steps: Vec<Step>,

    // responses is a list of responses of a user(requester) for surveys.
    #[api_model(summary, one_to_many = deliberation_responses, foreign_key = deliberation_id)]
    #[serde(default)]
    pub responses: Vec<DeliberationResponse>,
    // NOTE: skipped data for chart, responses per question types
    #[api_model(summary, one_to_many = deliberation_responses, foreign_key = deliberation_id, aggregator = count)]
    pub response_count: i64,
}
