use dioxus::prelude::*;

pub mod components;
pub mod config;
pub mod layout;
pub mod pages;

pub mod service {
    pub mod popup_service;
    pub mod user_service;
}

pub mod utils {
    pub mod time;
}

pub mod routes;

use routes::Route;
use service::{popup_service::PopupService, user_service::UserService};

const FAVICON: Asset = asset!("/public/favicon.png");
const MAIN_CSS: Asset = asset!("/public/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus_logger::init(config::get().log_level).expect("failed to init logger");

    dioxus_aws::launch(App);
}

#[component]
fn App() -> Element {
    PopupService::init();
    UserService::init();
    rsx! {
        btracing::ToastTracing {
            img {
                src: asset!("/public/logos/logo_symbol_white.png"),
                width: "30px",
            }
        }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        document::Script { src: asset!("/public/dep.js") }
        document::Script { src: "https://d3js.org/d3.v7.min.js" }
        load_tailwindcss {}
        Router::<Route> {}
    }
}

#[cfg(not(feature = "lambda"))]
#[allow(dead_code)]
fn load_tailwindcss() -> Element {
    rsx! {
        script { src: "https://unpkg.com/@tailwindcss/browser@4" }
    }
}

#[cfg(feature = "lambda")]
#[allow(dead_code)]
fn load_tailwindcss() -> Element {
    rsx! {}
}
