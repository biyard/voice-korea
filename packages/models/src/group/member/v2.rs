#[allow(unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use by_types::QueryResponse;

#[api_model(base = "/v2/organizations/:org-id/groups", table = group_members, iter_type=QueryResponse)]
pub struct GroupMemberV2 {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, many_to_one = groups)]
    pub group_id: i64,
    #[api_model(summary, many_to_one = users)]
    pub user_id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
}
