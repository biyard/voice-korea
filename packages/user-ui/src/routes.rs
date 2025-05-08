use dioxus::prelude::*;
use dioxus_translate::Language;

use crate::pages::{layout::MainLayout, *};

#[derive(Clone, PartialEq, Eq, Routable)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/:lang")]
        #[layout(MainLayout)]
            #[route("/")]
            MainPage { lang: Language },
            #[route("/coming-soon")]
            ComingSoonPage { lang: Language },
            #[route("/profile")]
            ProfilePage { lang: Language },
            #[route("/projects")]
            ProjectListPage { lang: Language },
            #[route("/governances")]
            GovernanceListPage { lang: Language },
        #[end_layout]

        #[nest("/projects")]
            #[route("/:project_id/discussion/:discussion_id")]
                DiscussionVideoPage { lang: Language, project_id: i64, discussion_id: i64 },
            #[layout(ProjectLayout)]
                #[route("/:project_id")]
                ProjectPage { lang: Language, project_id: i64 },
            #[end_layout]
        #[end_nest]


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
