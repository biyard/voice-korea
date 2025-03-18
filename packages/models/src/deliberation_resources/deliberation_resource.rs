use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/", table = deliberation_resources)]
pub struct DeliberationResource {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update], version = v0.1)]
    pub updated_at: i64,

    #[api_model(many_to_one = deliberations)]
    pub deliberation_id: i64,

    #[api_model(many_to_one = resources)]
    pub resource_id: i64,

    #[api_model(type = INTEGER, version = v0.1)]
    pub resource_type: DeliberationResourceType,
}

#[derive(
    Debug, Clone, Eq, PartialEq, Default, by_macros::ApiModel, dioxus_translate::Translate, Copy,
)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum DeliberationResourceType {
    #[default]
    #[translate(ko = "참고자료")]
    Reference = 1,
    #[translate(ko = "토론자료")]
    Debate = 2,
    #[translate(ko = "이러닝", en = "e-learning")]
    Elearning = 3,
}
