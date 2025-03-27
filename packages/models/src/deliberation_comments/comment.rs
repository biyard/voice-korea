#![allow(unused_variables, unused)]
use bdk::prelude::*;
use validator::Validate;

// NOTE: comments read only model
#[derive(Validate)]
#[api_model(base = "/v2/comments", table = deliberation_comments)]
pub struct Comment {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary)]
    pub comment: String,
}
