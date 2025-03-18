#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;

use crate::{deliberation_user::DeliberationUser, response::SurveyResponse};

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

    // Draft
    pub title: String,
    pub description: String,

    #[api_model(one_to_many = deliberation_users)]
    pub members: Vec<DeliberationUser>,

    // responses is a list of responses of a user(requester) for surveys.
    #[api_model(skip)]
    pub responses: Vec<SurveyResponse>,
}
