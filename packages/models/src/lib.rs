mod attribute;
mod auth;
pub mod error;
mod field;
mod group;
mod groups;
mod metadata;
mod organizations;
mod pagination;
mod panel;
mod panel_count;
mod panel_survey;
mod public_opinion;
mod resource;
// mod public_survey;
pub mod deliberation_comments;
pub mod deliberation_responses;
pub mod deliberation_users;
pub mod deliberation_votes;
pub mod deliberations;
pub mod dto;
mod group_members;
pub mod invitations;
mod organization_members;
mod projects;
mod reviews;
mod search;
mod strings;
mod survey;
mod update_field;
mod users;
mod verifications;

pub use crate::prelude::*;
pub use by_types::QueryResponse;

pub mod prelude {
    pub use crate::attribute::*;
    pub use crate::auth::*;
    pub use crate::deliberation_comments::*;
    pub use crate::deliberation_responses::*;
    pub use crate::deliberation_users::*;
    pub use crate::deliberation_votes::*;
    pub use crate::deliberations::*;
    pub use crate::error::*;
    pub use crate::field::*;
    pub use crate::group::*;
    pub use crate::groups::*;
    pub use crate::invitations::*;
    pub use crate::metadata::*;
    pub use crate::organizations::*;
    pub use crate::pagination::*;
    pub use crate::panel::*;
    pub use crate::panel_count::*;
    pub use crate::panel_survey::*;
    pub use crate::projects::*;
    pub use crate::public_opinion::*;
    pub use crate::resource::*;
    pub use crate::reviews::*;
    // pub use crate::public_survey::*;

    pub use crate::group_members::*;
    pub use crate::organization_members::*;
    pub use crate::search::*;
    pub use crate::strings::*;
    pub use crate::survey::*;
    pub use crate::update_field::*;
    pub use crate::users::*;
    pub use crate::verifications::*;
}

pub type Result<T> = std::result::Result<T, crate::error::ApiError>;
