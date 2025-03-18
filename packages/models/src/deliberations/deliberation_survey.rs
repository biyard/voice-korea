use bdk::prelude::*;

use crate::{response::SurveyResponse, SurveyV2};

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

    #[api_model(many_to_many = deliberation_surveys, table_name = surveys, foreign_primary_key = survey_id, foreign_reference_key = deliberation_id)]
    pub surveys: Vec<SurveyV2>,

    // responses is a list of responses of a user(requester) for surveys.
    #[api_model(skip)]
    pub responses: Vec<SurveyResponse>,
    // NOTE: skipped data for chart, responses per question types
}
