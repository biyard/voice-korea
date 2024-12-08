#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing;
use models::prelude::{SurveyDraftStatus, UpsertSurveyDraftRequest};

use crate::{
    api::v2::survey::{create_survey, get_survey, upsert_survey_draft},
    routes::Route,
    utils::time::convert_timestamp_to_separate_string,
};

use chrono::{Local, NaiveDate, TimeZone, Utc};

use super::Language;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    pub survey: Resource<models::prelude::Survey>,
    pub lang: Language,
    summary_clicked: Signal<bool>,
    panel_clicked: Signal<bool>,
    survey_list_clicked: Signal<bool>,
    pub survey_id: Signal<String>,
    pub start_date: Signal<String>,
    pub end_date: Signal<String>,
    pub object_count: Signal<u64>,
    pub subject_count: Signal<u64>,
    pub total_count: Signal<u64>,

    pub total_panels: Signal<u64>,
    pub total_attributes: Signal<u64>,

    pub start_year: Signal<String>,
    pub start_month: Signal<String>,
    pub start_day: Signal<String>,

    pub end_year: Signal<String>,
    pub end_month: Signal<String>,
    pub end_day: Signal<String>,

    pub invalid_date: Signal<bool>,
}

impl Controller {
    pub fn init(lang: Language, id: String) -> Self {
        let id_copy = id.clone();

        let survey_response: Resource<models::prelude::Survey> = use_resource(move || {
            let id_value = id.clone();
            async move { get_survey(id_value).await.unwrap_or_default() }
        });

        let mut ctrl = Self {
            lang,
            survey: survey_response,
            summary_clicked: use_signal(|| false),
            panel_clicked: use_signal(|| false),
            survey_list_clicked: use_signal(|| false),
            survey_id: use_signal(|| "".to_string()),
            start_date: use_signal(|| "".to_string()),
            end_date: use_signal(|| "".to_string()),
            object_count: use_signal(|| 0),
            subject_count: use_signal(|| 0),
            total_count: use_signal(|| 0),

            total_attributes: use_signal(|| 0),
            total_panels: use_signal(|| 0),

            start_year: use_signal(|| "".to_string()),
            start_month: use_signal(|| "".to_string()),
            start_day: use_signal(|| "".to_string()),
            end_year: use_signal(|| "".to_string()),
            end_month: use_signal(|| "".to_string()),
            end_day: use_signal(|| "".to_string()),

            invalid_date: use_signal(|| false),
        };

        match survey_response.value()() {
            Some(v) => {
                if let Some(v) = v.started_at {
                    let ymd = convert_timestamp_to_separate_string(v);
                    ctrl.start_year.set(ymd.0.to_string());
                    ctrl.start_month.set(ymd.1.to_string());
                    ctrl.start_day.set(ymd.2.to_string());
                }
                if let Some(v) = v.ended_at {
                    let ymd = convert_timestamp_to_separate_string(v);
                    ctrl.end_year.set(ymd.0.to_string());
                    ctrl.end_month.set(ymd.1.to_string());
                    ctrl.end_day.set(ymd.2.to_string());
                }
            }
            _ => {}
        }

        let (object_count, subject_count, total_count) = ctrl.get_survey_count();
        let _ = ctrl.get_panel_info();
        ctrl.survey_id.set(id_copy.clone());
        ctrl.object_count.set(object_count);
        ctrl.subject_count.set(subject_count);
        ctrl.total_count.set(total_count);

        use_context_provider(|| ctrl);

        ctrl
    }

    pub fn get_survey_id(&self) -> String {
        (self.survey_id)()
    }

    pub async fn clicked_start_survey(&mut self, lang: Language) {
        let navigator = use_navigator();
        let start_year: i32 = (self.start_year)().parse().unwrap();
        let start_month: u32 = (self.start_month)().parse().unwrap();
        let start_day: u32 = (self.start_day)().parse().unwrap();

        let end_year: i32 = (self.end_year)().parse().unwrap();
        let end_month: u32 = (self.end_month)().parse().unwrap();
        let end_day: u32 = (self.end_day)().parse().unwrap();
        tracing::debug!(
            "{} {} {} / {} {} {}",
            start_year,
            start_month,
            start_day,
            end_year,
            end_month,
            end_day
        );
        let naive_start_date = NaiveDate::from_ymd_opt(start_year, start_month, start_day)
            .expect("Invalid date components");
        let naive_start_datetime = naive_start_date
            .and_hms_opt(0, 0, 0)
            .expect("Invalid time components");
        let local_datetime = Local
            .from_local_datetime(&naive_start_datetime)
            .single()
            .expect("Invalid local datetime");
        let utc_datetime = local_datetime.with_timezone(&Utc);
        let start_timestamp = utc_datetime.timestamp_millis();

        let naive_end_date =
            NaiveDate::from_ymd_opt(end_year, end_month, end_day).expect("Invalid date components");
        let naive_end_datetime = naive_end_date
            .and_hms_opt(23, 59, 59)
            .expect("Invalid time components");
        let local_datetime = Local
            .from_local_datetime(&naive_end_datetime)
            .single()
            .expect("Invalid local datetime");
        let utc_datetime = local_datetime.with_timezone(&Utc);
        // UTC DateTime 생성
        let end_timestamp = utc_datetime.timestamp_millis();

        tracing::debug!("datetime: {} {}", start_timestamp, end_timestamp);

        let _ = upsert_survey_draft(UpsertSurveyDraftRequest {
            id: Some(self.get_survey_id()),
            status: Some(SurveyDraftStatus::Complete),
            started_at: Some(start_timestamp),
            ended_at: Some(end_timestamp),
            title: None,
            quotas: None,
            questions: None,
        })
        .await;
        let _ = create_survey(self.get_survey_id()).await;

        navigator.push(Route::DashboardPage { lang });
    }

    pub fn set_date(&mut self, start_date: Option<String>, end_date: Option<String>) {
        match start_date {
            Some(s) => {
                let start: Vec<&str> = s.split("-").collect();
                self.start_year.set(start.get(0).unwrap().to_string());
                self.start_month.set(start.get(1).unwrap().to_string());
                self.start_day.set(start.get(2).unwrap().to_string());
            }
            None => {}
        };

        match end_date {
            Some(e) => {
                let end: Vec<&str> = e.split("-").collect();

                self.end_year.set(end.get(0).unwrap().to_string());
                self.end_month.set(end.get(1).unwrap().to_string());
                self.end_day.set(end.get(2).unwrap().to_string());
            }
            None => {}
        }
    }

    pub fn get_total_panels(&self) -> u64 {
        (self.total_panels)()
    }

    pub fn get_total_attributes(&self) -> u64 {
        (self.total_attributes)()
    }

    pub fn get_survey_count(&self) -> (u64, u64, u64) {
        let questions = self.get_survey().questions;
        let mut object_count: u64 = 0;
        let mut subject_count: u64 = 0;
        let mut total_count: u64 = 0;

        for question in questions {
            match question.options {
                Some(_) => {
                    object_count += 1;
                }
                None => {
                    subject_count += 1;
                }
            }
            total_count += 1;
        }
        (object_count, subject_count, total_count)
    }

    pub fn get_panel_info(&mut self) {
        let mut panels = 0;
        let mut attributes = 0;
        let mut attribute_vec = vec![0, 0, 0, 0];

        for quota in self.get_quotas() {
            let attribute = quota.attribute;

            match attribute {
                Some(a) => {
                    match a.salary_tier {
                        Some(_s) => {
                            attribute_vec[0] = attribute_vec[0] + 1;
                        }
                        None => {}
                    }

                    match a.region_code {
                        Some(_r) => {
                            attribute_vec[1] = attribute_vec[1] + 1;
                        }
                        None => {}
                    };

                    match a.gender {
                        Some(_g) => {
                            attribute_vec[2] = attribute_vec[2] + 1;
                        }
                        None => {}
                    };

                    match a.age {
                        Some(_a) => {
                            attribute_vec[3] = attribute_vec[3] + 1;
                        }
                        None => {}
                    };
                }
                None => {}
            };

            panels += quota.quota;
        }

        for i in attribute_vec {
            if i != 0 {
                attributes += 1;
            }
        }

        self.total_panels.set(panels);
        self.total_attributes.set(attributes);
    }

    pub fn get_quotas(&self) -> Vec<models::prelude::Quota> {
        let quotas = self.get_survey().quotas;
        quotas
    }

    pub fn change_start_date(&mut self, start_date: String) {
        self.start_date.set(start_date.clone());
        self.set_date(Some(start_date.clone()), None);
    }

    pub fn change_end_date(&mut self, end_date: String) {
        self.end_date.set(end_date.clone());
        self.set_date(None, Some(end_date.clone()));
    }

    pub fn get_questions(&self) -> Vec<models::prelude::SurveyQuestion> {
        self.get_survey().questions
    }

    pub fn get_survey(&self) -> models::prelude::Survey {
        match (self.survey.value())() {
            Some(value) => value,
            None => models::prelude::Survey::default(),
        }
    }

    pub fn get_summary_clicked(&self) -> bool {
        (self.summary_clicked)()
    }

    pub fn get_panel_clicked(&self) -> bool {
        (self.panel_clicked)()
    }

    pub fn get_survey_list_clicked(&self) -> bool {
        (self.survey_list_clicked)()
    }

    pub fn change_summary_clicked(&mut self) {
        self.summary_clicked.set(!self.get_summary_clicked());
    }

    pub fn change_panel_clicked(&mut self) {
        self.panel_clicked.set(!self.get_panel_clicked());
    }

    pub fn change_survey_list_clicked(&mut self) {
        self.survey_list_clicked
            .set(!self.get_survey_list_clicked());
    }

    pub async fn back_button_clicked(&mut self) {
        tracing::debug!("back button clicked");
    }
}

#[allow(dead_code)]
pub fn use_controller() -> Controller {
    use_context()
}
