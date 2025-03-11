#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;

use crate::{deliberation_user::DeliberationUser, ResourceFile};

// TODO(web): using resource for project deliberation tab
// TODO(api): implement Read action(read) of GET /v2/deliberations/:deliberation-id/contents
#[api_model(base = "/v2/deliberations/:deliberation-id/contents", table = deliberations, read_action = read)]
pub struct DeliberationContent {
    pub id: i64,
    pub created_at: i64,
    pub updated_at: i64,

    pub description: String,

    #[api_model(one_to_many = deliberation_users)]
    pub members: Vec<DeliberationUser>,

    // NOTE: Remove PDFs after querying.
    // NOTE: Filter only PDFs after querying. Currently, api_model does not support filtering on joined table.
    #[api_model(many_to_many = deliberation_resources, foreign_table_name = resources, foreign_primary_key = resource_id, foreign_reference_key = deliberation_id)]
    pub study_materials: Vec<ResourceFile>,

    // elearning is a list of elearning PDF resources.
    #[api_model(skip)]
    pub elearning: Vec<ResourceFile>,
}
