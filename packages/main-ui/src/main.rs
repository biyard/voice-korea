#![allow(non_snake_case)]

use by_components::effects::HoverEffects;
use dioxus_logger::tracing;

use dioxus::prelude::*;
use main_ui::service::group_api::GroupApi;
use main_ui::service::member_api::MemberApi;
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
    MemberApi::init();
    GroupApi::init();
    OpinionApi::init();
    MetadataApi::init();

    rsx! {
        document::Link {
            rel: "icon",
            r#type: "image/x-icon",
            href: asset!("/public/favicon.ico"),
        }

        document::Script { src: "https://d3js.org/d3.v7.min.js" }

        head {
            link { rel: "stylesheet", href: asset!("/public/main.css") }
            link { rel: "stylesheet", href: asset!("/public/tailwind.css") }
            load_tailwindcss {}
        }
        HoverEffects {}
        Router::<Route> {}
    }
}

#[cfg(not(feature = "lambda"))]
#[allow(dead_code)]
fn load_tailwindcss() -> Element {
    rsx! {
        script { src: "https://cdn.tailwindcss.com/3.4.5" }
    }
}

#[cfg(feature = "lambda")]
#[allow(dead_code)]
fn load_tailwindcss() -> Element {
    rsx! {}
}
