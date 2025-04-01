#![allow(unused_variables, unused)]
use bdk::prelude::*;
use validator::Validate;

use crate::deliberation_user::DeliberationUser;
use crate::SurveyV2;

// NOTE: comments read only model
#[derive(Validate)]
#[api_model(base = "/v2/deliberation-final-surveys", table = deliberation_final_surveys)]
pub struct DeliberationFinalSurvey {
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

    #[api_model(summary, action = create, action_by_id = update)]
    #[serde(default)]
    pub estimate_time: i64,
    #[api_model(summary, action = create, action_by_id = update)]
    #[serde(default)]
    pub point: i64,

    #[api_model(summary, many_to_many = final_survey_members, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = final_survey_id)]
    #[serde(default)]
    pub members: Vec<DeliberationUser>,

    #[api_model(summary, many_to_many = final_surveys, foreign_table_name = surveys, foreign_primary_key = survey_id, foreign_reference_key = final_survey_id)]
    #[serde(default)]
    pub surveys: Vec<SurveyV2>,
}
