pub mod deliberation_user;

use crate::user::User;
pub use deliberation_user::*;

#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::{api_model, ApiModel};
use validator::Validate;

use crate::{PanelV2, ProjectArea, Resource, SurveyV2};

#[derive(Validate)]
#[api_model(base = "/organizations/v2/:org-id/deliberations", action = [create(resource_ids = Vec<i64>, survey_ids = Vec<i64>, roles = Vec<DeliberationUserCreateRequest>)], table = deliberations)]
pub struct Deliberation {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, many_to_one = organizations)]
    pub org_id: i64,

    // First page of creating a deliberation
    // started_at indicates the start time of the deliberation.
    #[api_model(action = create)]
    pub started_at: i64,
    // ended_at indicates the end time of the deliberation.
    #[api_model(action = create)]
    pub ended_at: i64,
    #[api_model(summary, one_to_many = steps, action = create)]
    pub steps: Vec<Step>,

    // Second page of creating a deliberation
    #[api_model(summary, type = INTEGER, action = create)]
    pub project_area: ProjectArea,
    #[api_model(action = create)]
    pub title: String,
    #[api_model(action = create)]
    pub description: String,

    #[api_model(many_to_many = deliberation_resources, table_name = resources foreign_primary_key = resource_id, foreign_reference_key = deliberation_id)]
    pub resources: Vec<Resource>,

    #[api_model(many_to_many = deliberation_surveys, table_name = surveys, foreign_primary_key = survey_id, foreign_reference_key = deliberation_id)]
    pub surveys: Vec<SurveyV2>,
    // Third page of creating a deliberation
    #[api_model(many_to_many = deliberations_users, table_name = deliberation_users, foreign_primary_key = user_id, foreign_reference_key = deliberation_id)]
    #[serde(default)]
    pub members: Vec<User>,

    #[api_model(summary, action = create, many_to_many = panel_deliberations, foreign_table_name = panels, foreign_primary_key = panel_id, foreign_reference_key = deliberation_id,)]
    #[serde(default)]
    pub panels: Vec<PanelV2>,
    // TODO: discussion should be added
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum StepType {
    #[default]
    None = 0,
    GeneralBoard = 1,
    VideoConference = 2,
    Survey = 3,
    Report = 4,
    PostBoard = 5,
}

#[api_model(base = "/organizations/v2/:org-id/deliberations", table = deliberations)]
pub struct Step {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,
    #[api_model(summary, many_to_one = deliberations)]
    pub deliberation_id: i64,
    #[api_model(summary, type = INTEGER, action = create)]
    pub step_type: StepType,
    #[api_model(summary, action = create)]
    pub name: String,
    #[api_model(summary, action = create)]
    pub started_at: i64,
    #[api_model(summary, action = create)]
    pub ended_at: i64,
}
