#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use validator::Validate;

use crate::ProjectArea;

#[derive(Validate)]
#[api_model(base = "/v2/projects", table = deliberations)]
pub struct DeliberationContent {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary)]
    pub title: String,
    #[api_model(summary)]
    pub description: String,
    #[api_model(summary)]
    pub project_area: ProjectArea,

    #[api_model(summary, many_to_one = organizations)]
    pub org_id: i64,
    #[api_model(summary, one_to_many = deliberation_users, foreign_key = deliberation_id, aggregator = count)]
    pub participants: i64,
    #[api_model(summary, one_to_many = deliberation_votes, foreign_key = deliberation_id, aggregator = count)]
    pub votes: i64,
}
