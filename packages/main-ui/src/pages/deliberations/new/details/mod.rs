mod controller;
mod i18n;
mod layout;
mod models;

pub(self) use super::controller::Controller as DeliberationNewController;
pub use layout::*;
pub(self) use models::*;

// children pages
mod basic_info;
mod deliberation;
mod discussions;
mod sample_survey;
mod votes;

pub use basic_info::*;
pub use deliberation::*;
pub use discussions::*;
pub use sample_survey::*;
pub use votes::*;
