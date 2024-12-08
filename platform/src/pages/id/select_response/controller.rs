#![allow(non_snake_case)]
#[allow(unused_imports)]
use std::fmt::format;

use crate::api::v2::survey::{get_survey, upsert_survey_draft};
use dioxus::prelude::*;
use models::prelude::{SurveyDraftStatus, UpsertSurveyDraftRequest};

#[derive(Clone, PartialEq, Copy)]
pub struct Controller {
    survey_response: Resource<models::prelude::Survey>,
    id: Signal<String>,
    attributes: Signal<Vec<AttributeInfo>>,
}

#[derive(Clone, PartialEq)]
pub struct AttributeInfo {
    pub region: String,
    pub gender: String,
    pub age: String,
    pub payload: String,
    pub value: u64,
}

impl Controller {
    pub fn init(id: String) -> Self {
        let id_copy = id.clone();
        let survey_response: Resource<models::prelude::Survey> = use_resource(move || {
            let id_value = id.clone();
            async move {
                let survey = get_survey(id_value).await;
                survey.unwrap_or_default()
            }
        });

        let ctrl = Self {
            survey_response,
            id: use_signal(|| id_copy.clone()),
            attributes: use_signal(|| vec![]),
        };
        ctrl
    }

    pub fn get_attributes(&self) -> Vec<AttributeInfo> {
        let attributes = self.get_survey().quotas;
        let mut attrs = vec![];

        for attribute in attributes {
            let mut sal = "".to_string();
            let mut reg = "".to_string();
            let mut gen = "".to_string();
            let mut ag = "".to_string();

            match attribute.attribute {
                Some(attr) => {
                    match attr.salary_tier {
                        Some(salary) => {
                            if salary == 1 {
                                sal = "2000만원 이하".to_string();
                            } else if salary == 2 {
                                sal = "2000만원~4000만원".to_string();
                            } else if salary == 3 {
                                sal = "4000만원~6000만원".to_string();
                            } else if salary == 4 {
                                sal = "6000만원~8000만원".to_string();
                            } else {
                                sal = "8000만원 이상".to_string();
                            }
                        }
                        None => {}
                    };
                    match attr.region_code {
                        Some(region) => {
                            if region == 02 {
                                reg = "서울".to_string();
                            } else if region == 051 {
                                reg = "부산".to_string();
                            } else if region == 053 {
                                reg = "대구".to_string();
                            } else if region == 032 {
                                reg = "인천".to_string();
                            } else if region == 062 {
                                reg = "광주".to_string();
                            } else if region == 042 {
                                reg = "대전".to_string();
                            } else if region == 052 {
                                reg = "울산".to_string();
                            } else if region == 044 {
                                reg = "세종".to_string();
                            } else if region == 031 {
                                reg = "경기".to_string();
                            } else if region == 033 {
                                reg = "강원".to_string();
                            } else if region == 043 {
                                reg = "충북".to_string();
                            } else if region == 041 {
                                reg = "충남".to_string();
                            } else if region == 063 {
                                reg = "전북".to_string();
                            } else if region == 061 {
                                reg = "전남".to_string();
                            } else if region == 054 {
                                reg = "경북".to_string();
                            } else if region == 055 {
                                reg = "경남".to_string();
                            } else if region == 064 {
                                reg = "제주".to_string();
                            }
                        }
                        None => {}
                    };
                    gen = match attr.gender {
                        Some(gender) => {
                            if gender == models::prelude::Gender::Male {
                                "남성".to_string()
                            } else {
                                "여성".to_string()
                            }
                        }
                        None => "".to_string(),
                    };
                    match attr.age {
                        Some(age) => {
                            ag = match age {
                                models::prelude::Age::Specific(age) => format!("{}세", age),
                                models::prelude::Age::Range {
                                    inclusive_min,
                                    inclusive_max,
                                } => {
                                    if inclusive_max.is_none() && inclusive_min.is_none() {
                                        "미정".to_string()
                                    } else if inclusive_max.is_none() {
                                        format!("{}세 이하", inclusive_min.unwrap())
                                    } else if inclusive_min.is_none() {
                                        format!("{}세 이상", inclusive_max.unwrap())
                                    } else {
                                        format!(
                                            "{}~{}세",
                                            inclusive_min.unwrap(),
                                            inclusive_max.unwrap()
                                        )
                                    }
                                }
                            }
                        }
                        None => {}
                    };
                }
                None => {}
            }

            attrs.push(AttributeInfo {
                region: reg,
                gender: gen,
                age: ag,
                payload: sal,
                value: attribute.quota,
            });
        }
        attrs
    }

    pub async fn save_button_clicked(&mut self) {
        let _ = upsert_survey_draft(UpsertSurveyDraftRequest {
            id: Some(self.get_survey_id()),
            status: Some(SurveyDraftStatus::Complete),
            title: None,
            quotas: None,
            questions: None,
            started_at: None,
            ended_at: None,
        })
        .await;
    }

    pub async fn back_button_clicked(&mut self) {
        let _ = upsert_survey_draft(UpsertSurveyDraftRequest {
            id: Some(self.get_survey_id()),
            status: Some(SurveyDraftStatus::Question),
            title: None,
            quotas: None,
            questions: None,
            started_at: None,
            ended_at: None,
        })
        .await;
    }

    pub fn get_survey_id(&self) -> String {
        (self.id)()
    }

    pub fn get_title(&self) -> String {
        self.get_survey().title.clone()
    }

    pub fn get_survey(&self) -> models::prelude::Survey {
        match (self.survey_response.value())() {
            Some(value) => value,
            None => models::prelude::Survey::default(),
        }
    }
}
