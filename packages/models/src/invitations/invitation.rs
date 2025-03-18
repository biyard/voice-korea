#![allow(unused_variables)]
#[allow(unused)]
use crate::Result;
use crate::Role;
use bdk::prelude::*;
use by_types::QueryResponse;

#[derive(validator::Validate)]
#[api_model(base = "invitations/v2/:org-id", table = invitations, action = [invite(email = String)], iter_type=QueryResponse)]
pub struct Invitation {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, many_to_one=organizations)] // TODO: using composite key with email
    pub org_id: i64,
    // #[api_model(summary, many_to_one=groups, action = [invite], nullable)]
    #[api_model(summary, action = [invite])]
    pub group_id: Option<i64>,
    // #[api_model(summary, many_to_one=projects)]
    // pub project_id: i64,
    #[api_model(summary, query_action = query_by_email)] // TODO: using composite key with org_id
    #[validate(email)]
    pub email: String,
    #[api_model(summary, action = [invite])]
    pub name: Option<String>,
    #[api_model(summary, type = INTEGER, action = [invite])]
    pub role: Option<Role>,
}
