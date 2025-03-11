use crate::User;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use by_types::QueryResponse;
#[cfg(feature = "server")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct OrganizationMiddlewareParams {
    pub id: String,
}

#[api_model(base = "/v2/organizations", table = organizations, iter_type=QueryResponse)]
pub struct Organization {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary)]
    pub name: String,

    #[api_model(summary, version = v0.1)]
    pub description: Option<String>,
    #[api_model(summary, one_to_many = deliberations, foreign_key = org_id, aggregator = count)]
    #[serde(default)]
    pub projects: i64,
    #[api_model(summary, one_to_many = deliberation_votes, foreign_key = org_id, aggregator = count)]
    #[serde(default)]
    pub votes: i64,
    #[api_model(many_to_many = organization_members, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = org_id, unique)]
    #[serde(default)]
    pub users: Vec<User>,
}
