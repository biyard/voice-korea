use bdk::prelude::*;

use crate::{deliberation::Deliberation, User};

// TODO: implement Read action(find_one) of GET /v2/profile/projects
#[api_model(base = "/v2/profile/projects", database = skip, read_action = find)]
pub struct ProfileData {
    pub designed_projects: Vec<Deliberation>,
    pub participated_projects: Vec<Deliberation>,
    pub user: User,
}
