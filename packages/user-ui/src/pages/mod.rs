mod components;
mod controller;
mod i18n;
pub mod layout;
pub mod main_footer;
pub mod main_header;
pub mod page;

mod coming_soon;
mod education;
mod governance;
mod not_found;
mod profile;
mod projects;
mod users;

pub use coming_soon::*;
pub use education::page::EducationPage;
pub use governance::*;
pub use not_found::*;
pub use page::*;
pub use profile::*;
pub use projects::*;
pub use users::*;
