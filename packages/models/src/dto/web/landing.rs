#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use validator::Validate;

use crate::{
    deliberation_project::DeliberationProject, organization::OrganizationSummary, review::Review,
};

#[derive(Validate)]
#[api_model(base = "/web", database = skip, read_action = find_one)]
pub struct LandingData {
    pub projects: Vec<DeliberationProject>,
    pub organizations: Vec<OrganizationSummary>,
    pub reviews: Vec<Review>,
}
