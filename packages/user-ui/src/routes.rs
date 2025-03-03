use dioxus::prelude::*;
use dioxus_translate::Language;

#[allow(unused)]
use crate::layout::{RootLayout, RootLayoutWithFooter};

use crate::pages::governance::_id::GovernancePage;
use crate::pages::main::MainPage;
use crate::pages::profile::page::ProfilePage;
use crate::pages::users::login::UserLoginPage;
use crate::pages::NotFoundPage;

#[derive(Clone, Routable)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/:lang")]
        #[layout(RootLayoutWithFooter)]
            #[route("/")]
            MainPage { lang: Language },
            #[nest("/users")]
                #[route("/")]
                UserLoginPage { lang: Language },
            #[end_nest]
        #[end_layout]

        #[layout(RootLayout)]
            #[route("/governance/:governance_id")]
            GovernancePage { lang: Language, governance_id: i64 },
            #[route("/profile")]
            ProfilePage { lang: Language },
        #[end_layout]
        
    #[end_nest]
    #[redirect("/", || Route::MainPage { lang: Language::Ko })]
    #[route("/:..route")]
    NotFoundPage {
        route: Vec<String>,
    },
}
