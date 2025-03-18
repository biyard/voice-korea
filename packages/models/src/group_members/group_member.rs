use bdk::prelude::*;

#[api_model(base = "/v2/organizations/:org-id/groups/:group-id/members", table = group_members)]
pub struct GroupMemberV2 {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,

    #[api_model(summary, many_to_one = groups)]
    pub group_id: i64,
    #[api_model(summary, many_to_one = users, action = [create, remove_member])]
    pub user_id: i64,
}
