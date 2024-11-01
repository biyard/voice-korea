#![allow(non_snake_case)]
use std::time::{Duration, UNIX_EPOCH};

use chrono::{self, DateTime, Local};
use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::{
    api::v1::surveys::{list_surveys, upsert_survey::create_empty_survey},
    service::login_service::use_login_service,
};

use super::{Language, Route};

#[derive(Debug, Clone, PartialEq)]
pub struct Survey {
    pub survey_type: String,
    pub title: String,
    pub update_date: String,
    pub response_count: u64,
    pub total_response_count: u64,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    pub surveys: Signal<Vec<Survey>>,
    pub clicked_type: Signal<u64>, //0: type-1, 1: type-2
    pub is_error: Signal<bool>,
}

impl Controller {
    pub fn init(lang: Language) -> Self {
        let navigator = use_navigator();
        let email: String = use_login_service().get_email().clone();

        if email.is_empty() {
            navigator.push(Route::LoginPage { lang });
        };

        let mut ctrl = Self {
            surveys: use_signal(|| vec![]),
            clicked_type: use_signal(|| 0),
            is_error: use_signal(|| false),
        };

        let _ = use_effect(move || {
            let value = email.clone();
            spawn(async move {
                match list_surveys(value, None, None, None).await {
                    Ok(res) => {
                        let surveys = res.items;

                        let total_surveys: Vec<Survey> = surveys
                            .into_iter()
                            .map(|survey| Survey {
                                survey_type: survey.status.to_string(),
                                title: survey.title,
                                update_date: Self::format_date(survey.updated_at),
                                response_count: survey.responses.unwrap_or_default(),
                                total_response_count: survey.expected_responses.unwrap_or_default(),
                            })
                            .collect();
                        ctrl.surveys.set(total_surveys);
                    }
                    Err(e) => {
                        tracing::error!("Error: {:?}", e);
                    }
                }
            });
        });

        ctrl
    }

    pub fn format_date(timestamp: u64) -> String {
        let d = UNIX_EPOCH + Duration::from_secs(timestamp);
        let datetime = DateTime::<Local>::from(d);
        datetime.format("%Y.%m.%d").to_string()
    }

    pub fn get_clicked_type(&self) -> u64 {
        (self.clicked_type)()
    }

    pub fn set_clicked_type(&mut self, clicked_type: u64) {
        self.clicked_type.set(clicked_type);
    }

    pub fn get_total_surveys(&self) -> Vec<Survey> {
        (self.surveys)()
    }

    pub fn get_error(&self) -> bool {
        (self.is_error)()
    }

    pub async fn clicked_create_survey(&mut self, lang: Language) {
        let navigator = use_navigator();
        let email = use_login_service().get_email().clone();
        let res = create_empty_survey(email).await;

        match res {
            Ok(v) => {
                self.is_error.set(false);
                navigator.push(Route::WriteTitlePage {
                    lang: lang.clone(),
                    id: v,
                });
            }
            Err(e) => {
                tracing::error!("create survey err: {:?}", e);
                self.is_error.set(true);
            }
        }
    }
}
