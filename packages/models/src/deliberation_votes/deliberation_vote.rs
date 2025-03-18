#![allow(dead_code, unused)]
use bdk::prelude::*;
use validator::Validate;

use crate::User;

#[derive(Validate)]
#[api_model(base = "/v2/deliberations/votes", table = deliberation_votes)]
pub struct DeliberationVote {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, type = INTEGER, action = voting, action_by_id = update)]
    pub vote: VoteResult,
    #[api_model(summary, many_to_one = organizations)]
    pub org_id: i64,
    #[api_model(summary, many_to_one = deliberations)]
    pub deliberation_id: i64,
    #[api_model(summary, many_to_one = users)]
    pub user_id: i64,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum VoteResult {
    #[default]
    Neutral = 0,
    Supportive = 1,
    Against = 2,
}
