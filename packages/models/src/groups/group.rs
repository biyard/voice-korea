#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;

use crate::User;

#[api_model(base = "/v2/organizations/:org-id/groups", table = groups, action_by_id = [add_group_member(email = String), remove_group_member(user_id = i64), delete])]
pub struct Group {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,
    #[api_model(summary, many_to_one = organizations)]
    pub org_id: i64,
    #[api_model(summary, action_by_id = update, action = create)]
    pub name: String,

    #[api_model(summary, many_to_many = group_members, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = group_id, type = JSONB, unique)]
    pub members: Vec<User>,
}
