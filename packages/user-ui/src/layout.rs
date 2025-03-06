#![allow(non_snake_case)]
use dioxus::prelude::*;

use dioxus_translate::Language;

use crate::routes::Route;

use crate::components::{Footer, Header};
use crate::service::popup_service::PopupZone;
#[component]
pub fn RootLayout(lang: Language) -> Element {
    rsx! {
        div { class: "w-full flex justify-center px-[10px]",
            div { class: "w-full flex flex-col max-w-[1300px] min-h-lvh",
                Header { lang: lang.clone() }
                Outlet::<Route> {}
            }
        }
        PopupZone {}
    }
}

#[component]
pub fn RootLayoutWithFooter(lang: Language) -> Element {
    rsx! {
        div { class: "w-full flex flex-col justify-center items-center",
            div { class: "w-full flex flex-col max-w-[1300px] px-[10px]",
                Header { lang: lang.clone() }
            }
            div { class: "flex flex-col w-full min-h-lvh", Outlet::<Route> {} }
            div { class: "flex flex-row w-full ",
                Footer { lang: lang.clone() }
            }
        }
        PopupZone {}
    }
}
