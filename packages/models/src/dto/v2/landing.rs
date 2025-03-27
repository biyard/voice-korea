use bdk::prelude::*;

use crate::{
    deliberation_project::DeliberationProject, organization::OrganizationSummary, review::Review,
};

// TODO: implement Read action(find_one) of GET /v2/landing
#[api_model(base = "/v2/landing", database = skip, read_action = find_one)]
pub struct LandingData {
    pub projects: Vec<DeliberationProject>,
    pub organizations: Vec<OrganizationSummary>,
    pub reviews: Vec<Review>,
}
