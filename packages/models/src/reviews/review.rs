#![allow(unused)]

#[allow(unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use by_types::QueryResponse;

#[api_model(base = "/v2/reviews", table = reviews, iter_type=QueryResponse)]
pub struct Review {
    #[api_model(summary, primary_key, action = delete )]
    pub id: i64,
    #[api_model(summary, auto = insert)]
    pub created_at: i64,
    #[api_model(auto = [insert, update], summary)]
    pub updated_at: i64,

    #[api_model(summary, action = create, many_to_one = deliberations)]
    pub deliberation_id: i64,
    #[api_model(summary, action = create, many_to_one = users)]
    pub user_id: i64,

    #[api_model(summary, action = create, action_by_id = update, query_action = search_by)]
    pub name: String,
    #[api_model(summary, action = create, action_by_id = update)]
    pub image: String,
    #[api_model(summary, action = create, action_by_id = update)]
    pub review: String,
}
