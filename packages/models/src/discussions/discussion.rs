#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use validator::Validate;

use crate::{ResourceFile, User};

// TODO(web): using resource for discussion tab on a project
// TODO(api): implement action_by_id action(start_meeting) of POST /v2/deliberations/:deliberation-id/discussions/:id
#[derive(Validate)]
#[api_model(base = "/v2/deliberations/:deliberation-id/discussions", table = discussions, action = [create(resources = Vec<i64>)], action_by_id = [start_meeting, delete])]
pub struct Discussion {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = deliberations)]
    pub deliberation_id: i64,

    #[api_model(summary, action = create, action_by_id = update)]
    pub started_at: i64,
    #[api_model(summary, action = create, action_by_id = update)]
    pub ended_at: i64,

    #[api_model(summary, action = create, action_by_id = update)]
    pub name: String,

    #[api_model(summary, action = create, action_by_id = update)]
    pub description: String,

    pub zoom_link: Option<String>,

    #[api_model(many_to_many = discussion_groups, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = discussion_id)]
    pub user_id: Vec<User>,

    #[api_model(many_to_many = discussion_resources, table_name = resources, foreign_primary_key = resource_id, foreign_reference_key = discussion_id)]
    pub resources: Vec<ResourceFile>,
}
