use crate::deliberation_comment::DeliberationComment;
use crate::deliberation_user::DeliberationUser;
use crate::deliberation_user::DeliberationUserCreateRequest;

#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use validator::Validate;

use crate::deliberation_vote::DeliberationVote;
use crate::{PanelV2, ProjectArea, ResourceFile, SurveyV2};

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
    #[api_model(action = create)]
    pub steps: Vec<Step>,

    // Second page of creating a deliberation
    #[api_model(action = create)]
    pub project_area: ProjectArea,
    #[api_model(action = create)]
    pub title: String,
    #[api_model(action = create)]
    pub description: String,

    #[api_model(many_to_many = deliberation_resources, table_name = resources, foreign_primary_key = resource_id, foreign_reference_key = deliberation_id)]
    pub resources: Vec<ResourceFile>,

    #[api_model(many_to_many = deliberation_surveys, table_name = surveys, foreign_primary_key = survey_id, foreign_reference_key = deliberation_id)]
    pub surveys: Vec<SurveyV2>,
    // Third page of creating a deliberation
    #[api_model(one_to_many = deliberation_users)]
    pub members: Vec<DeliberationUser>,
    #[api_model(one_to_many = deliberation_votes)]
    pub votes: Vec<DeliberationVote>,
    #[api_model(summary, action = create, many_to_many = panel_deliberations, foreign_table_name = panels, foreign_primary_key = panel_id, foreign_reference_key = deliberation_id,)]
    #[serde(default)]
    pub panels: Vec<PanelV2>,
    // TODO: discussion should be added
    #[api_model(one_to_many = deliberation_comments)]
    pub comments: Vec<DeliberationComment>,
    #[api_model(summary, one_to_many = deliberation_responses, foreign_key = deliberation_id, aggregator = count)]
    pub response_count: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "snake_case")]
pub enum Step {
    #[default]
    None,
    GeneralBoard {
        name: String,
        started_at: i64,
        ended_at: i64,
    },
    VideoConference {
        name: String,
        started_at: i64,
        ended_at: i64,
    },
    Survey {
        name: String,
        started_at: i64,
        ended_at: i64,
    },
    Report {
        name: String,
        started_at: i64,
        ended_at: i64,
    },
    PostBoard {
        name: String,
        started_at: i64,
        ended_at: i64,
    },
}
