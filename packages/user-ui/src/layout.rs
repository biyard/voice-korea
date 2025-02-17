#![allow(non_snake_case)]
use dioxus::prelude::*;

use dioxus_translate::Language;

use crate::routes::Route;

use crate::components::{Footer, Header};
#[component]
pub fn RootLayout(lang: Language) -> Element {
    rsx! {
        div { class: "w-full flex justify-center",
            div { class: "w-full flex flex-col max-w-[1300px] min-h-lvh",
                Header { lang: lang.clone() }
                Outlet::<Route> {}
                Footer { lang: lang.clone() }
            }
        }
    }
}
