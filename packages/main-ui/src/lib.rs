pub mod prelude {
    pub use crate::layouts::root_layout::*;
    pub use crate::routes::*;
    pub use crate::utils::context::*;

    pub use crate::pages::not_found::NotFoundPage;
    pub use dioxus::document::eval;
}

pub mod config;
pub mod pages;

pub mod service {
    pub mod attribute_api;
    pub mod group_api;
    pub mod login_service;
    pub mod member_api;
    pub mod metadata_api;
    pub mod opinion_api;
    pub mod organization_api;
    pub mod popup_service;
    pub mod theme;
}

pub mod models {
    pub mod pi;
    pub mod question;
    pub mod role_field;
    pub mod survey;
    pub mod user;
}

pub mod utils {
    pub mod api;
    pub mod context;
    pub mod hash;
    pub mod metadata;
    pub mod time;
}

pub mod layouts {
    pub mod root_layout;
}

pub mod components {
    pub mod alert;
    pub mod bar_graph;
    pub mod bottom;
    pub mod button;
    pub mod calendar;
    pub mod checkbox;
    pub mod icons;
    pub mod input;
    pub mod label;
    pub mod pagination;
    pub mod pi_graph;
    pub mod popup;
    pub mod select;
    pub mod stepper;
    pub mod table_row;
    pub mod upload_button;
}

pub mod api;
pub mod routes;
