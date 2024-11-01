use dioxus::prelude::*;

use crate::prelude::*;
use crate::presentations::create::CreatePage;
use crate::presentations::dashboard::DashboardPage;
use crate::presentations::find_email::FindEmailPage;
use crate::presentations::login::LoginPage;
use crate::presentations::reset_password::ResetPasswordPage;
use crate::presentations::select_response::SelectResponsePage;
use crate::presentations::select_response_detail::SelectResponseDetailPage;
use crate::presentations::survey_summary::SurveySummaryPage;
use crate::presentations::write_question::WriteQuestionPage;
use crate::presentations::write_title::WriteTitlePage;
use crate::utils::context::{default_lang, Language};

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/:lang")]
        #[layout(RootLayout)]
            #[route("/dashboard")]
            DashboardPage { lang: Language },
            #[route("/id/:id/write-title")]
            WriteTitlePage { lang: Language, id: String },
            #[route("/:title/write-question")]
            WriteQuestionPage { lang: Language, title: String },
            #[route("/:title/select-response")]
            SelectResponsePage { lang: Language, title: String },
            #[route("/:title/select-response/type/:select_type")]
            SelectResponseDetailPage { lang: Language, title: String, select_type: String },
            #[route("/:title/survey-summary")]
            SurveySummaryPage { lang: Language, title: String },
        #[end_layout]

        #[route("/")]
        LoginPage { lang: Language },
        #[route("/create")]
        CreatePage { lang: Language },
        #[route("/find-email")]
        FindEmailPage { lang: Language },
        #[route("/reset-password")]
        ResetPasswordPage { lang: Language },
    #[end_nest]

    #[redirect("/", || Route::LoginPage { lang: default_lang() })]
    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}
