#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;

use crate::{
    deliberation_report::DeliberationReport, deliberation_response::DeliberationResponse,
    deliberation_user::DeliberationUser, SurveyV2,
};

// TODO(web): using resource for basic info tab.
// TODO(api): implement query survey response.
#[api_model(base = "/v2/projects/:deliberation-id/draft", table = deliberations, read_action = read)]
pub struct DeliberationDraft {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, many_to_one = organizations)]
    pub org_id: i64,

    #[api_model(one_to_many = deliberation_users, foreign_key = deliberation_id)]
    pub members: Vec<DeliberationUser>,

    #[api_model(one_to_many = deliberation_reports, foreign_key = deliberation_id)]
    pub reports: Vec<DeliberationReport>,

    #[api_model(many_to_many = deliberation_surveys, table_name = surveys, foreign_primary_key = survey_id, foreign_reference_key = deliberation_id)]
    pub surveys: Vec<SurveyV2>,

    // responses is a list of responses of a user(requester) for surveys.
    #[api_model(summary, one_to_many = deliberation_responses, foreign_key = deliberation_id)]
    #[serde(default)]
    pub responses: Vec<DeliberationResponse>,
}
