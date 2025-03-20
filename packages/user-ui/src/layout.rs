#![allow(non_snake_case)]
use by_components::loaders::cube_loader::CubeLoader;
use dioxus::prelude::*;

use dioxus_logger::tracing;
use dioxus_translate::Language;

use crate::routes::Route;

use crate::components::{Footer, Header};
use crate::service::popup_service::PopupZone;
#[component]
pub fn RootLayout(lang: Language) -> Element {
    let path: Route = use_route();

    let path = path.to_string();

    rsx! {
        ErrorBoundary {
            handle_error: move |e| {
                tracing::error!("error: {:?}", e);
                rsx! { "error : " }
            },
            div { class: "flex flex-col w-screen min-h-screen bg-white text-black",
                // Header {
                //     logout: translates.logout,
                //     lang,
                // }

                if path.contains("projects") {
                    div { class: "w-full flex flex-col justify-center items-center",
                        div { class: "w-full flex flex-col max-w-[1300px] px-[10px]",
                            Header { lang: lang.clone() }
                        }
                        div { class: "flex flex-col w-full min-h-lvh", Outlet::<Route> {} }
                    }
                } else {
                    div { class: "w-full flex flex-col justify-center items-center",
                        div { class: "w-full flex flex-col max-w-[1300px] px-[10px]",
                            Header { lang: lang.clone() }
                        }
                        SuspenseBoundary {
                            fallback: |_| rsx! {
                                div { class: "absolute w-screen h-screen top-0 left-0 flex items-center justify-center text-white",
                                    CubeLoader {}
                                }
                            },
                            div { class: "flex flex-col w-full min-h-lvh", Outlet::<Route> {} }
                        }
                    }
                }
                PopupZone {}
            }
        }
    }
}

#[component]
pub fn RootLayoutWithFooter(lang: Language) -> Element {
    rsx! {
        ErrorBoundary {
            handle_error: move |e| {
                tracing::error!("error: {:?}", e);
                rsx! { "error : " }
            },
            div { class: "w-full flex flex-col justify-center items-center",
                div { class: "w-full flex flex-col max-w-[1300px] px-[10px]",
                    Header { lang: lang.clone() }
                }
                SuspenseBoundary {
                    fallback: |_| rsx! {
                        div { class: "absolute w-screen h-screen top-0 left-0 flex items-center justify-center text-white",
                            CubeLoader {}
                        }
                    },
                    div { class: "flex flex-col w-full min-h-lvh", Outlet::<Route> {} }
                    div { class: "flex flex-row w-full ",
                        Footer { lang: lang.clone() }
                    }
                }
            }
            PopupZone {}
        }
    }
}
