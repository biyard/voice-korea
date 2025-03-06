#[allow(unused)]
use crate::User;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use by_types::QueryResponse;

#[api_model(base = "/organizations/v2/:org-id/groups", table = groups, action_by_id = [add_group_member(email = String), remove_group_member(user_id = i64)], iter_type=QueryResponse)]
pub struct GroupV2 {
    // FIXME: If add read action, it will be used as unused variable
    #[api_model(summary, primary_key, action = delete)] //read_action = [get_group, find_by_id])]
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
