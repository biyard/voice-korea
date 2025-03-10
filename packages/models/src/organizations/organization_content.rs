#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v2/organizations/contents", table = organizations)]
pub struct OrganizationContent {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary)]
    pub name: String,
    #[api_model(summary)]
    pub description: Option<String>,
    #[api_model(summary, one_to_many = deliberations, foreign_key = organization_id, aggregator = count)]
    pub projects: i64,
    #[api_model(summary, one_to_many = deliberation_votes, foreign_key = organization_id, aggregator = count)]
    pub votes: i64,
}
