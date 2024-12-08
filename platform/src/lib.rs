pub mod prelude {
    pub use crate::layouts::root_layout::*;
    pub use crate::routes::*;
    pub use crate::utils::context::*;

    pub use crate::pages::not_found::NotFoundPage;
    pub use dioxus::document::eval;
}

pub mod pages {
    pub mod attributes;
    pub mod create;
    pub mod dashboard;
    pub mod find_email;
    pub mod id {
        pub mod response_report;
        pub mod select_response;
        pub mod survey_summary;
        pub mod write_question;
        pub mod write_title;
    }
    pub mod login;
    pub mod not_found;
    pub mod reset_password;
}

pub mod service {
    pub mod login_service;
}

pub mod models {
    pub mod pi;
    pub mod question;
    pub mod survey;
    pub mod user;
}

pub mod utils {
    pub mod api;
    pub mod context;
    #[cfg(feature = "server")]
    pub mod db;
    pub mod hash;
    #[cfg(feature = "server")]
    pub mod logger;
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
    pub mod checkbox;
    pub mod icons;
    pub mod input;
    pub mod pi_graph;
    pub mod popup;
    pub mod select;
    pub mod table_row;
}

pub mod api;
pub mod routes;
