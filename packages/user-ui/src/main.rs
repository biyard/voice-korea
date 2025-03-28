use bdk::prelude::*;

pub mod components;
pub mod config;
pub mod pages;

pub mod service {
    pub mod popup_service;
    pub mod user_service;
}

pub mod utils {
    pub mod time;
}

pub mod routes;

use dioxus_oauth::prelude::FirebaseProvider;
use routes::Route;
use service::{popup_service::PopupService, user_service::UserService};

const FAVICON: Asset = asset!("/public/favicon.png");
const MAIN_CSS: Asset = asset!("/public/main.css");
const TAILWIND_CSS: Asset = asset!("/public/tailwind.css");

fn main() {
    dioxus_logger::init(config::get().log_level).expect("failed to init logger");

    dioxus_aws::launch(App);
}

#[component]
fn App() -> Element {
    PopupService::init();
    UserService::init();

    let conf = config::get();
    let css = include_str!("../public/input.css");

    rsx! {
        FirebaseProvider {
            api_key: conf.firebase.api_key.clone(),
            auth_domain: conf.firebase.auth_domain.clone(),
            project_id: conf.firebase.project_id.clone(),
            storage_bucket: conf.firebase.storage_bucket.clone(),
            messaging_sender_id: conf.firebase.messaging_sender_id.clone(),
            app_id: conf.firebase.app_id.clone(),
            measurement_id: conf.firebase.measurement_id.clone(),
        }
        btracing::ToastTracing {
            img {
                src: asset!("/public/logos/logo_symbol_white.png"),
                width: "30px",
            }
        }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        // document::Link {
        //     rel: "stylesheet",
        //     href: "https://cdn.quilljs.com/1.3.6/quill.snow.css",
        // }

        // document::Script { src: "https://cdn.quilljs.com/1.3.6/quill.min.js" }
        document::Style { r#type: "text/tailwindcss", {css} }
        document::Script { src: "https://d3js.org/d3.v7.min.js" }
        document::Script { src: "https://unpkg.com/@tailwindcss/browser@4.0.12/dist/index.global.js" }
        document::Script { src: "https://cdn.jsdelivr.net/npm/amazon-chime-sdk-js@3.27.1/build/index.min.js" }
        Router::<Route> {}
    }
}

#[cfg(feature = "server")]
mod api {
    use bdk::prelude::*;
    use server_fn::codec::{GetUrl, Json};

    #[server(endpoint = "/version", input=GetUrl, output=Json)]
    pub async fn version() -> Result<String, ServerFnError> {
        Ok(match option_env!("VERSION") {
            Some(version) => match option_env!("COMMIT") {
                Some(commit) => format!("{}-{}", version, commit),
                None => format!("{}", version),
            },
            None => match option_env!("DATE") {
                Some(date) => date.to_string(),
                None => "unknown".to_string(),
            },
        }
        .to_string())
    }
}
