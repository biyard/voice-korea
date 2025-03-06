#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::{api_model, ApiModel};

use crate::response::Answer;

#[api_model(base = "/v2/deliberations/:deliberation-id/responses", table = deliberation_responses)]
pub struct DeliberationResponse {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = deliberations)]
    pub deliberation_id: i64,
    #[api_model(summary, auto = [insert, update])]
    pub user_id: i64,

    #[api_model(summary, action = respond_answer, type = JSONB)]
    pub answers: Vec<Answer>,
    #[api_model(summary, action = [respond_answer], type = INTEGER, nullable)]
    pub deliberation_type: DeliberationType,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum DeliberationType {
    #[default]
    Sample = 1,
    Survey = 2,
}
