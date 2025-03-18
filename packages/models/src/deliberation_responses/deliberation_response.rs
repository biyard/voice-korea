use bdk::prelude::*;

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
    #[api_model(many_to_one = users, action = respond_answer, summary)]
    pub user_id: i64,

    #[api_model(summary, action = respond_answer, type = JSONB)]
    pub answers: Vec<Answer>,
    #[api_model(summary, action = [respond_answer], type = INTEGER)]
    pub deliberation_type: DeliberationType,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum DeliberationType {
    #[default]
    #[translate(ko = "표본 조사", en = "Sample Survey")]
    Sample = 1,
    #[translate(ko = "최종 설문", en = "Final Survey")]
    Survey = 2,
}
