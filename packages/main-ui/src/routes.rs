use dioxus::prelude::*;

use crate::pages::attributes::AttributePage;
use crate::pages::create::CreatePage;
use crate::pages::deliberations::new::page::OpinionCreatePage;
use crate::pages::deliberations::page::DeliberationPage;
use crate::pages::find_email::FindEmailPage;
use crate::pages::groups::_id::page::GroupDetailPage;
use crate::pages::groups::page::GroupPage;
use crate::pages::members::_id::page::MemberDetailPage;
use crate::pages::members::page::MemberPage;
use crate::pages::panels::page::PanelPage;
use crate::pages::reset_password::ResetPasswordPage;
use crate::pages::resources::page::ResourcePage;
use crate::pages::surveys::_id::page::SurveyResultPage;
use crate::pages::surveys::_id::update::page::SurveyUpdatePage;
use crate::pages::surveys::new::page::SurveyCreatePage;
use crate::pages::surveys::page::SurveyPage;
use crate::pages::*;
use crate::prelude::*;
use dioxus_translate::Language;

#[derive(Clone, Routable, Debug, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[nest("/:lang")]
        #[layout(RootLayout)]
            #[route("/groups")]
            GroupPage { lang: Language },
            #[route("/groups/:group_id")]
            GroupDetailPage { lang: Language, group_id: i64 },
            #[route("/deliberations")]
            DeliberationPage { lang: Language },
            #[route("/deliberations/new")]
            OpinionCreatePage { lang: Language },
            #[route("/members")]
            MemberPage { lang: Language },
            #[route("/members/:member_id")]
            MemberDetailPage { lang: Language, member_id: String },
            #[route("/attributes")]
            AttributePage { lang: Language },
            #[route("/panels")]
            PanelPage { lang: Language },
            #[route("/resources")]
            ResourcePage { lang: Language },
            #[route("/surveys")]
            SurveyPage { lang: Language },
            #[route("/surveys/new")]
            SurveyCreatePage { lang: Language },
            #[route("/surveys/:survey_id")]
            SurveyResultPage { lang: Language, survey_id: i64 },
            #[route("/surveys/:survey_id/update")]
            SurveyUpdatePage { lang: Language, survey_id: i64 },
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

    #[redirect("/", || Route::LoginPage { lang: Language::Ko })]
    #[route("/:..route")]
    NotFoundPage { route: Vec<String> },
}

impl Route {
    pub fn to_menu(&self) -> Option<String> {
        match self {
            Route::GroupPage { lang } if lang == &Language::Ko => Some("그룹 관리".to_string()),
            Route::GroupPage { lang } if lang == &Language::En => {
                Some("Group Management".to_string())
            }
            Route::GroupDetailPage { lang, .. } if lang == &Language::Ko => {
                Some("그룹 관리".to_string())
            }
            Route::GroupDetailPage { lang, .. } if lang == &Language::En => {
                Some("Group Management".to_string())
            }
            Route::DeliberationPage { lang } if lang == &Language::Ko => {
                Some("공론 조사".to_string())
            }
            Route::DeliberationPage { lang } if lang == &Language::En => {
                Some("Public Opinion Survey".to_string())
            }
            Route::OpinionCreatePage { lang } if lang == &Language::Ko => {
                Some("공론 조사".to_string())
            }
            Route::OpinionCreatePage { lang } if lang == &Language::En => {
                Some("Public Opinion Survey".to_string())
            }
            Route::MemberPage { lang } if lang == &Language::Ko => Some("팀원 관리".to_string()),
            Route::MemberPage { lang } if lang == &Language::En => {
                Some("Member Management".to_string())
            }
            Route::MemberDetailPage { lang, .. } if lang == &Language::Ko => {
                Some("팀원 관리".to_string())
            }
            Route::MemberDetailPage { lang, .. } if lang == &Language::En => {
                Some("Member Management".to_string())
            }
            Route::AttributePage { lang } if lang == &Language::Ko => {
                Some("속성 & 패널 관리".to_string())
            }
            Route::AttributePage { lang } if lang == &Language::En => {
                Some("Attribute & Panel Management".to_string())
            }
            Route::PanelPage { lang } if lang == &Language::Ko => {
                Some("속성 & 패널 관리".to_string())
            }
            Route::PanelPage { lang } if lang == &Language::En => {
                Some("Attribute & Panel Management".to_string())
            }
            Route::ResourcePage { lang } if lang == &Language::Ko => Some("자료 관리".to_string()),
            Route::ResourcePage { lang } if lang == &Language::En => {
                Some("Resource Management".to_string())
            }
            Route::SurveyPage { lang } if lang == &Language::Ko => Some("여론 조사".to_string()),
            Route::SurveyPage { lang } if lang == &Language::En => {
                Some("Public Opinion Survey".to_string())
            }
            Route::SurveyResultPage { lang, .. } if lang == &Language::Ko => {
                Some("여론 조사".to_string())
            }
            Route::SurveyResultPage { lang, .. } if lang == &Language::En => {
                Some("Public Opinion Survey".to_string())
            }
            Route::SurveyCreatePage { lang } if lang == &Language::Ko => {
                Some("여론 조사".to_string())
            }
            Route::SurveyCreatePage { lang } if lang == &Language::En => {
                Some("Public Opinion Survey".to_string())
            }
            Route::SurveyUpdatePage { lang, .. } if lang == &Language::Ko => {
                Some("여론 조사".to_string())
            }
            Route::SurveyUpdatePage { lang, .. } if lang == &Language::En => {
                Some("Public Opinion Survey".to_string())
            }
            Route::LoginPage { lang } if lang == &Language::Ko => Some("로그인".to_string()),
            Route::LoginPage { lang } if lang == &Language::En => Some("Login".to_string()),
            Route::CreatePage { lang } if lang == &Language::Ko => Some("계정 생성".to_string()),
            Route::CreatePage { lang } if lang == &Language::En => {
                Some("Create Account".to_string())
            }
            Route::FindEmailPage { lang } if lang == &Language::Ko => {
                Some("이메일 찾기".to_string())
            }
            Route::FindEmailPage { lang } if lang == &Language::En => {
                Some("Find Email".to_string())
            }
            Route::ResetPasswordPage { lang } if lang == &Language::Ko => {
                Some("비밀번호 재설정".to_string())
            }
            Route::ResetPasswordPage { lang } if lang == &Language::En => {
                Some("Reset Password".to_string())
            }
            _ => None,
        }
    }
}
