use super::step_type::StepType;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;

#[api_model(base = "/organizations/v2/:org-id/deliberations", table = deliberations_steps)]
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
    #[serde(default)]
    pub step_type: StepType,
    #[api_model(summary, action = create)]
    pub name: String,
    #[api_model(summary, action = create)]
    pub started_at: i64,
    #[api_model(summary, action = create)]
    pub ended_at: i64,
}
