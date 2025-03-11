#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use validator::Validate;

use crate::Role;

#[derive(Validate)]
#[api_model(base = "/", table = deliberation_users)]
pub struct DeliberationUser {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = users, action = create)]
    pub user_id: i64,
    #[api_model(many_to_one = organizations)]
    pub organization_id: i64,
    #[api_model(many_to_one = deliberations)]
    pub deliveration_id: i64,

    #[api_model(action = create)]
    pub role: Role,
}
