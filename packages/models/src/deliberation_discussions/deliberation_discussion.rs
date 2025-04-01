#![allow(unused_variables, unused)]
use bdk::prelude::*;
use validator::Validate;

use crate::deliberation_user::DeliberationUser;
use crate::discussions::Discussion;
use crate::ResourceFile;

#[derive(Validate)]
#[api_model(base = "/v2/deliberation-contents", table = deliberation_discussions)]
pub struct DeliberationDiscussion {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    // started_at indicates the start time of the deliberation.
    #[api_model(summary, action = create, action_by_id = update)]
    pub started_at: i64,
    // ended_at indicates the end time of the deliberation.
    #[api_model(summary, action = create, action_by_id = update)]
    pub ended_at: i64,

    #[api_model(summary, action = create, action_by_id = update)]
    pub title: String,
    #[api_model(summary, action = create, action_by_id = update)]
    pub description: String,

    #[api_model(summary, many_to_one = deliberations)]
    pub deliberation_id: i64,

    #[api_model(summary, many_to_many = deliberation_discussion_members, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = discussion_id)]
    #[serde(default)]
    pub members: Vec<DeliberationUser>,

    #[api_model(summary, many_to_many = deliberation_discussion_resources, foreign_table_name = resources, foreign_primary_key = resource_id, foreign_reference_key = discussion_id)]
    #[serde(default)]
    pub resources: Vec<ResourceFile>,

    #[api_model(one_to_many = discussions, foreign_key = deliberation_id, reference_key = deliberation_id)]
    #[serde(default)]
    pub discussions: Vec<Discussion>,
}
