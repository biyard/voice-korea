use bdk::prelude::*;

use crate::{deliberation_user::DeliberationUser, step::Step, ResourceFile};

// TODO(web): using resource for project deliberation tab
// TODO(api): implement Read action(read) of GET /v2/deliberations/:deliberation-id/contents
#[api_model(base = "/v2/projects/:deliberation-id/contents", table = deliberations, read_action = read)]
pub struct DeliberationContent {
    pub id: i64,
    pub created_at: i64,
    pub updated_at: i64,

    pub title: String,
    pub description: String,

    #[api_model(one_to_many = deliberation_users, foreign_key = deliberation_id)]
    pub members: Vec<DeliberationUser>,

    #[api_model(summary, one_to_many = deliberations_steps, foreign_key = deliberation_id)]
    #[serde(default)]
    pub steps: Vec<Step>,

    // NOTE: Remove PDFs after querying.
    // NOTE: Filter only PDFs after querying. Currently, api_model does not support filtering on joined table.
    #[api_model(many_to_many = deliberation_study_materials, table_name = resources, foreign_primary_key = resource_id, foreign_reference_key = deliberation_id)]
    #[serde(default)]
    pub resources: Vec<ResourceFile>,
    // elearning is a list of elearning PDF resources.
    #[api_model(skip)]
    #[serde(default)]
    pub elearning: Vec<ResourceFile>,
}
