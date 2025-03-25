#![allow(non_snake_case)]

use by_components::effects::HoverEffects;
use dioxus_logger::tracing;

use dioxus::prelude::*;
use main_ui::service::metadata_api::MetadataApi;
use main_ui::service::opinion_api::OpinionApi;
use main_ui::service::organization_api::OrganizationApi;
use main_ui::service::popup_service::PopupService;

use main_ui::config;
use main_ui::service::theme::Theme;

use main_ui::{
    routes::Route, service::login_service::LoginService, utils::context::use_iitp_context_provider,
};

fn main() {
    dioxus_logger::init(config::get().log_level).expect("failed to init logger");

    tracing::debug!("starting app");
    dioxus_aws::launch(App);
}

fn App() -> Element {
    use_iitp_context_provider();
    Theme::init();
    by_components::theme::BiyardTheme::init();
    LoginService::init();
    PopupService::init();

    OrganizationApi::init();
    OpinionApi::init();
    MetadataApi::init();

    let css = include_str!("../public/input.css");

    rsx! {
        document::Link {
            rel: "icon",
            r#type: "image/x-icon",
            href: asset!("/public/favicon.ico"),
        }
        document::Link { href: "https://fonts.googleapis.com", rel: "preconnect" }
        document::Link {
            crossorigin: "false",
            href: "https://fonts.gstatic.com",
            rel: "preconnect",
        }
        document::Link {
            href: "https://cdn.jsdelivr.net/gh/fonts-archive/Pretendard/Pretendard.css",
            r#type: "text/css",
            rel: "stylesheet",
        }
        document::Script { src: "https://d3js.org/d3.v7.min.js" }
        document::Style { r#type: "text/tailwindcss", {css} }
        document::Script { src: "https://unpkg.com/@tailwindcss/browser@4.0.12/dist/index.global.js" }
        // document::Style { href: asset!("/public/tailwind.css") }
        document::Style { href: asset!("/public/main.css") }

        HoverEffects {}
        Router::<Route> {}
    }
}
