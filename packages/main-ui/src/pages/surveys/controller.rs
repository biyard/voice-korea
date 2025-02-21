use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::{
    QueryResponse, SurveyV2, SurveyV2Client, SurveyV2DeleteRequest, SurveyV2Query,
    SurveyV2StartSurveyRequest, SurveyV2Summary,
};

use crate::pages::surveys::page::RemoveSurveyModal;
use crate::service::login_service::LoginService;
use crate::service::popup_service::PopupService;

use super::i18n::SurveyTranslate;

#[derive(Debug, Clone, Copy)]
pub struct Controller {
    client: Signal<SurveyV2Client>,
    lang: Language,
    pub surveys: Resource<QueryResponse<SurveyV2Summary>>,
    popup_service: PopupService,
    page: Signal<usize>,
    pub size: usize,
    org_id: Signal<String>,
    translate: Signal<SurveyTranslate>,
}

impl Controller {
    pub fn new(lang: dioxus_translate::Language) -> std::result::Result<Self, RenderError> {
        let login_service: LoginService = use_context();
        let org_id = match login_service.get_selected_org() {
            Some(v) => v.id.to_string(),
            None => "".to_string(),
        };
        let translate: SurveyTranslate = translate(&lang);
        let page = use_signal(|| 1);
        let size = 10;
        let user: LoginService = use_context();

        // FIXME: it causes screen flickering when navigating to this page
        // let surveys = use_server_future(move || {
        //     let page = page();

        //     async move {
        //         match SurveyV2::get_client(config::get().api_url)
        //             .query(SurveyV2Query::new(size).with_page(page))
        //             .await
        //         {
        //             Ok(res) => res,
        //             Err(e) => {
        //                 tracing::error!("Failed to list surveys: {:?}", e);
        //                 QueryResponse::default()
        //             }
        //         }
        //     }
        // })?;

        let client = SurveyV2::get_client(&crate::config::get().api_url);
        let client_copy = client.clone();

        let surveys = use_server_future(move || {
            let page = page();
            let client = client.clone();

            async move {
                let org_id = user.get_selected_org();
                if org_id.is_none() {
                    tracing::error!("Organization ID is missing");
                    return QueryResponse::default();
                }

                match client
                    .query(org_id.unwrap().id, SurveyV2Query::new(size).with_page(page))
                    .await
                {
                    Ok(res) => res,
                    Err(e) => {
                        tracing::error!("Failed to list surveys: {:?}", e);
                        QueryResponse::default()
                    }
                }
            }
        })?;

        let ctrl = Self {
            client: use_signal(|| client_copy.clone()),
            page,
            size,
            lang,
            surveys,
            popup_service: use_context(),
            translate: use_signal(|| translate),
            org_id: use_signal(|| org_id.clone()),
        };

        Ok(ctrl)
    }

    pub fn set_page(&mut self, page: usize) {
        self.page.set(page);
    }

    pub fn page(&self) -> usize {
        (self.page)()
    }

    pub fn total_pages(&self) -> usize {
        let size = self.size;
        self.surveys.with(|v| {
            if let Some(v) = v {
                if v.total_count != 0 {
                    (v.total_count as usize - 1) / size + 1
                } else {
                    0
                }
            } else {
                0
            }
        }) as usize
    }

    pub fn get_surveys(&self) -> Option<QueryResponse<SurveyV2Summary>> {
        self.surveys.with(|v| v.clone())
    }

    pub async fn start_survey(&mut self, survey_id: i64) {
        let client = (self.client)().clone();
        let org_id = (self.org_id)().parse::<i64>().unwrap_or_default();
        let mut survey_resource = self.surveys;

        match client
            .act_by_id(
                org_id,
                survey_id,
                models::SurveyV2ByIdAction::StartSurvey(SurveyV2StartSurveyRequest {}),
            )
            .await
        {
            Ok(_) => {
                survey_resource.restart();
            }
            Err(e) => {
                tracing::error!("Failed to start survey: {:?}", e);
            }
        }
    }

    pub async fn open_remove_survey_modal(&mut self, survey_id: String) {
        let mut popup_service = self.popup_service;
        let mut public_survey_resource = self.surveys;
        let translate = (self.translate)();
        let client = (self.client)().clone();
        let org_id = (self.org_id)();

        // TODO: implement remove survey
        popup_service
            .open(rsx! {
                RemoveSurveyModal {
                    lang: self.lang,
                    onclose: move |_e: MouseEvent| {
                        popup_service.close();
                    },
                    onremove: {
                        move |_e: MouseEvent| {
                            let survey_id = survey_id.clone();
                            let client = client.clone();
                            let org_id = org_id.clone();
                            async move {
                                let _ = client
                                    .act(
                                        org_id.parse::<i64>().unwrap_or_default(),
                                        models::SurveyV2Action::Delete(SurveyV2DeleteRequest {
                                            id: survey_id.parse::<i64>().unwrap_or_default(),
                                        }),
                                    )
                                    .await;
                                public_survey_resource.restart();
                                popup_service.close();
                            }
                        }
                    },
                }
            })
            .with_id("remove_survey")
            .with_title(translate.remove_modal_title);
    }
}
