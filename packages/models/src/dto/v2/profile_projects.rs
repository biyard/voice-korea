use bdk::prelude::*;

use crate::deliberation::Deliberation;

// TODO: implement Read action(find_one) of GET /v2/profile/projects
#[api_model(base = "/v2/profile/projects", database = skip, read_action = find_one)]
pub struct ProfileProjectsData {
    pub designed_projects: Vec<Deliberation>,
    pub participated_projects: Vec<Deliberation>,
}
