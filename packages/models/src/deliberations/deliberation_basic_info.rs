#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;

use crate::{deliberation_user::DeliberationUser, ResourceFile};

// TODO(web): using resource for basic info tab.
#[api_model(base = "/v2/projects/:deliberation-id/basic-info", table = deliberations, read_action = read)]
pub struct DeliberationBasicInfo {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    // Introduction
    pub description: String,

    #[api_model(one_to_many = deliberation_users)]
    pub members: Vec<DeliberationUser>,

    #[api_model(many_to_many = deliberation_resources, table_name = resources, foreign_primary_key = resource_id, foreign_reference_key = deliberation_id)]
    pub resources: Vec<ResourceFile>,
}
