mod controller;
mod i18n;
mod page;

pub(self) use super::controller::Controller as ParentController;
pub use page::*;
