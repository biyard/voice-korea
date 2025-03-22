use bdk::prelude::btracing;
use dioxus::prelude::*;
use dioxus_logger::tracing;
use dioxus_translate::{translate, Language};
use models::{
    QueryResponse, SurveyV2, SurveyV2Client, SurveyV2DeleteRequest, SurveyV2Query,
    SurveyV2StartSurveyRequest, SurveyV2Summary,
};

use crate::pages::surveys::components::setting_reward_modal::{
    SettingRewardModal, SettingRewardModalTranslate,
};
use crate::pages::surveys::page::{ErrorModal, RemoveSurveyModal};
use crate::service::login_service::LoginService;
use crate::service::popup_service::PopupService;

use super::i18n::{ErrorModalTranslate, SurveyTranslate};

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
        let mut popup_service = self.popup_service;
        let client = (self.client)().clone();
        let org_id = (self.org_id)().parse::<i64>().unwrap_or_default();
        let mut survey_resource = self.surveys;

        let surveys = self.get_surveys().unwrap().items;
        let surveys: Vec<SurveyV2Summary> = surveys
            .iter()
            .filter(|v| v.id == survey_id)
            .map(|v| v.clone())
            .collect();

        let survey = if surveys.is_empty() {
            SurveyV2Summary::default()
        } else {
            surveys[0].clone()
        };

        let tr: ErrorModalTranslate = translate(&self.lang);

        if survey.name.trim().is_empty()
            || survey.description.trim().is_empty()
            || survey.questions.is_empty()
            || survey.panels.is_empty()
            || survey.panel_counts.is_empty()
        {
            popup_service
                .open(rsx! {
                    ErrorModal {
                        lang: self.lang,
                        onclose: move |_e: MouseEvent| {
                            popup_service.close();
                        },
                    }
                })
                .with_id("survey error")
                .with_title(tr.title);
        } else {
            match client
                .act_by_id(
                    org_id,
                    survey_id,
                    models::SurveyV2ByIdAction::StartSurvey(SurveyV2StartSurveyRequest {}),
                )
                .await
            {
                Ok(_) => {
                    btracing::debug!("success to start survey");
                    survey_resource.restart();
                }
                Err(e) => {
                    btracing::error!("Failed to start survey with error: {:?}", e);
                }
            }
        }
    }

    pub async fn open_setting_reward_modal(
        &mut self,
        id: i64,
        lang: Language,
        estimate_time: i64,
        point: i64,
        questions: i64,
    ) {
        let mut surveys = self.surveys;
        let mut popup_service = self.popup_service;
        let cli = SurveyV2::get_client(crate::config::get().api_url);
        let org_id = (self.org_id)().parse::<i64>().unwrap_or_default();

        let tr: SettingRewardModalTranslate = translate(&lang);

        popup_service
            .open(rsx! {
                SettingRewardModal {
                    lang,
                    questions,
                    estimate_time,
                    point,

                    change_estimate_time: move |_| {},
                    change_point: move |_| {},
                    onsend: {
                        move |(estimate_time, point): (i64, i64)| {
                            let cli = cli.clone();
                            async move {
                                tracing::debug!("estimate time: {:?} point: {:?}", estimate_time, point);
                                match cli.update_setting(org_id, id, estimate_time, point).await {
                                    Ok(_) => {
                                        btracing::debug!("success to update setting");
                                        surveys.restart();
                                        popup_service.close();
                                    }
                                    Err(e) => {
                                        btracing::error!("Failed to update setting with error: {:?}", e);
                                        popup_service.close();
                                    }
                                }
                            }
                        }
                    },
                    oncancel: move |_| {
                        popup_service.close();
                    },
                }
            })
            .with_id("setting reward in main")
            .with_title(tr.title);
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
                                match client
                                    .act(
                                        org_id.parse::<i64>().unwrap_or_default(),
                                        models::SurveyV2Action::Delete(SurveyV2DeleteRequest {
                                            id: survey_id.parse::<i64>().unwrap_or_default(),
                                        }),
                                    )
                                    .await
                                {
                                    Ok(_) => {
                                        btracing::debug!("success to remove survey");
                                        public_survey_resource.restart();
                                        popup_service.close();
                                    }
                                    Err(e) => {
                                        btracing::error!("Failed to remove survey with error: {:?}", e);
                                        popup_service.close();
                                    }
                                }
                            }
                        }
                    },
                }
            })
            .with_id("remove_survey")
            .with_title(translate.remove_modal_title);
    }
}
