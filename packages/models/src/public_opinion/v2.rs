#![allow(unused)]

#[allow(unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use by_types::QueryResponse;

use crate::survey::ProjectArea;

//FIXME: fix to full public opinion model
#[api_model(base = "/organizations/v2/:org_id/public-opinions", table = public_opinions, iter_type=QueryResponse)]
pub struct PublicOpinionProject {
    #[api_model(summary, primary_key, action = delete )]
    pub id: i64,
    #[api_model(auto = insert)]
    pub created_at: i64,
    #[api_model(auto = [insert, update], summary)]
    pub updated_at: i64,

    #[api_model(summary, action = create, action_by_id = update)]
    pub title: String,
    #[api_model(summary, action = create, action_by_id = update)]
    pub description: String,
    #[api_model(summary, action = create, action_by_id = update)]
    pub policy_making_institution: String,
    #[api_model(summary, action = create, action_by_id = update, type = INTEGER, nullable)]
    pub project_area: Option<ProjectArea>,
    #[api_model(summary, many_to_one = organizations)]
    pub org_id: i64,

    #[api_model(summary, many_to_one = public_opinion_institutions)]
    pub institution_id: i64,
    #[api_model(summary, action = create, action_by_id = update,)]
    pub num_of_participation: i64,
    #[api_model(summary, action = create, action_by_id = update,)]
    pub num_of_vote: i64,
    #[api_model(summary, action = create, action_by_id = update,)]
    pub accepters: i64,
    #[api_model(summary, action = create, action_by_id = update,)]
    pub rejecters: i64,
}

#[api_model(base = "/organizations/v2/:org_id/public-opinion-institutions", table = public_opinion_institutions, iter_type=QueryResponse)]
pub struct PublicOpinionInstitution {
    #[api_model(summary, primary_key, action = delete )]
    pub id: i64,
    #[api_model(auto = insert)]
    pub created_at: i64,
    #[api_model(auto = [insert, update], summary)]
    pub updated_at: i64,

    #[api_model(summary, action = create, action_by_id = update)]
    pub name: String,
    #[api_model(summary, action = create, action_by_id = update)]
    pub description: String,
    #[api_model(summary, one_to_many = public_opinions, foreign_key = institution_id)]
    pub projects: Vec<PublicOpinionProject>,

    #[api_model(summary, action = create, action_by_id: update)]
    pub num_of_participation: i64,
    #[api_model(summary, one_to_many = public_opinions, foreign_key = institution_id, aggregator = count)]
    pub num_of_projects: i64,
    #[api_model(summary, action = create, action_by_id = update,)]
    pub num_of_vote: i64,
}

#[api_model(base = "/organizations/v2/:org_id/public-opinion-reviews", table = public_opinion_reviews, iter_type=QueryResponse)]
pub struct PublicOpinionReview {
    #[api_model(summary, primary_key, action = delete )]
    pub id: i64,
    #[api_model(summary, auto = insert)]
    pub created_at: i64,
    #[api_model(auto = [insert, update], summary)]
    pub updated_at: i64,

    #[api_model(summary, action = create, action_by_id = update)]
    pub name: String,
    #[api_model(summary, action = create, action_by_id = update)]
    pub image: String,
    #[api_model(summary, one_to_many = public_opinions, foreign_key = institution_id)]
    pub review: String,
}
