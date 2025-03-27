use dioxus::prelude::*;
use dioxus_translate::Language;

#[allow(unused)]
use crate::layout::{RootLayout, RootLayoutWithFooter};

use crate::pages::{
    layout::{MainRootLayout, MainRootLayoutWithoutFooter},
    *,
};

#[derive(Clone, Routable)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/:lang")]
        #[layout(MainRootLayout)]
            #[route("/")]
            MainPage { lang: Language },
        #[end_layout]

        #[layout(MainRootLayoutWithoutFooter)]
            #[route("/coming-soon")]
            ComingSoonPage { lang: Language },
            #[route("/profile")]
            ProfilePage { lang: Language },
            #[route("/projects")]
            ProjectListPage { lang: Language },
        #[end_layout]

        #[layout(ProjectLayout)]
            #[route("/projects/:project_id")]
            ProjectPage { lang: Language, project_id: i64 },
        #[end_layout]

        #[layout(GovernanceLayout)]
            #[route("/governance/:governance_id")]
            GovernancePage { lang: Language, governance_id: i64 },
        #[end_layout]
    #[end_nest]

    #[nest("/:lang")]
        #[route("/education/:resource_id")]
        EducationPage { lang: Language, resource_id: i64 },
    #[end_nest]
    
    #[redirect("/", || Route::MainPage { lang: Language::Ko })]
    #[route("/:..route")]
    NotFoundPage {
        route: Vec<String>,
    },
}
