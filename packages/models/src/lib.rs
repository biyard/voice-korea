mod attribute;
mod auth;
pub mod error;
mod field;
mod group;
mod metadata;
mod organization;
mod pagination;
mod panel;
mod panel_count;
mod panel_survey;
mod public_opinion;
mod resource;
// mod public_survey;
pub mod deliberation;
mod projects;
mod search;
mod strings;
mod survey;
mod update_field;
mod user;

pub use crate::prelude::*;
pub use by_types::QueryResponse;

pub mod prelude {
    pub use crate::attribute::*;
    pub use crate::auth::*;
    pub use crate::deliberation::*;
    pub use crate::error::*;
    pub use crate::field::*;
    pub use crate::group::*;
    pub use crate::metadata::*;
    pub use crate::organization::*;
    pub use crate::pagination::*;
    pub use crate::panel::*;
    pub use crate::panel_count::*;
    pub use crate::panel_survey::*;
    pub use crate::projects::*;
    pub use crate::public_opinion::*;
    pub use crate::resource::*;
    // pub use crate::public_survey::*;
    pub use crate::search::*;
    pub use crate::strings::*;
    pub use crate::survey::*;
    pub use crate::update_field::*;
    pub use crate::user::*;
}

pub type Result<T> = std::result::Result<T, crate::error::ApiError>;
