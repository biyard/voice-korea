use crate::user::User;
#[allow(unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use by_types::QueryResponse;
#[allow(unused)]
#[api_model(base = "/organizations/v2/:org-id/groups", table = groups, iter_type=QueryResponse)]
pub struct GroupV2 {
    #[api_model(summary, primary_key, action = delete)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,
    #[api_model(summary, many_to_one = organizations)]
    pub org_id: i64,
    #[api_model(summary, action_by_id = update, action = create)]
    pub name: String,
    #[api_model(many_to_many = group_members, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = group_id, action = create, unique)]
    #[serde(default)]
    pub users: Vec<User>,
}
