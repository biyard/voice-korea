use dioxus::prelude::*;
use dioxus_translate::Language;

#[allow(unused)]
use crate::layout::RootLayout;

use crate::pages::main::MainPage;
use crate::pages::users::login::UserLoginPage;
use crate::pages::NotFoundPage;

#[derive(Clone, Routable)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/:lang")]
        #[layout(RootLayout)]
            #[route("/")]
            MainPage { lang: Language },
            #[nest("/users")]
                #[route("/")]
                UserLoginPage { lang: Language },
            #[end_nest]
        #[end_layout]
        
    #[end_nest]
    #[redirect("/", || Route::MainPage { lang: Language::Ko })]
    #[route("/:..route")]
    NotFoundPage {
        route: Vec<String>,
    },
}
