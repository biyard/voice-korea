#![allow(non_snake_case)]
use super::*;
use crate::{
    components::icons::Logout,
    prelude::*,
    service::{login_service::LoginService, popup_service::PopupZone},
};
use by_components::loaders::cube_loader::CubeLoader;
use components::*;
use dioxus::prelude::*;
use i18n::RootLayoutTranslate;
use side_bar::{SelectedMenu, SideBar};

use dioxus_translate::{translate, Language};

#[component]
pub fn RootLayout(lang: Language) -> Element {
    let route: Route = use_route();
    let mut selected_menu = use_signal(move || route.to_menu().unwrap_or_default());
    use dioxus_logger::tracing;

    let tr: RootLayoutTranslate = translate(&lang);
    let user: LoginService = use_context();
    let is_logged_in = use_signal(|| (user.email)().clone().is_some());
    let nav = use_navigator();

    use_effect(move || {
        if !is_logged_in() {
            tracing::info!("redirect to login page");

            nav.replace(Route::LoginPage { lang });
        }
    });

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
                PopupZone {}
                div { class: "flex flex-row w-screen h-screen overflow-hidden",
                    SideBar {
                        onselected: move |selected: SelectedMenu| {
                            tracing::info!("selected menu {selected:?}");
                            selected_menu.set(selected.menu);
                        },
                        selected_menu: (selected_menu)(),
                        lang,
                    }
                    div { class: "flex flex-col w-full overflow-y-scroll bg-[#f0f2fc] px-[60px] pt-[25px]",
                        div { class: "flex flex-row w-full justify-end items-end gap-[5px]",
                            Link {
                                class: "flex flex-row justify-start items-start",
                                to: Route::LoginPage { lang },
                                div { class: "w-[20px] h-[20px]",
                                    Logout { width: "20", height: "20" }
                                }
                                div { class: "ml-[5px] font-bold text-[#555462] text-[15px]",
                                    "{tr.logout}"
                                }
                            }
                        }
                        SuspenseBoundary {
                            fallback: |_| rsx! {
                                div { class: "absolute w-screen h-screen top-0 left-0 flex items-center justify-center text-white",
                                    CubeLoader {}
                                }
                            },
                            div { class: "flex flex-col grow w-full bg-[#f0f2fc] px-[60px] pt-[25px]",
                                Outlet::<Route> {}
                            }
                        }
                    }
                }
            }
        }
    }
}
