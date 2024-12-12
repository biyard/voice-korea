#![allow(non_snake_case)]
#[allow(unused_imports)]
use std::collections::HashMap;

use chrono::{TimeZone, Utc};
use dioxus::prelude::*;
#[allow(unused_imports)]
use dioxus_logger::tracing;
use models::prelude::SurveyResultDocument;

use crate::{api::v2::survey::get_survey_result, models::pi::PiChart, prelude::Language};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    pub select_page: Signal<SelectPage>,
    pub panels: Signal<Vec<Response>>,
    pub clicked_index: Signal<usize>,
    pub attributes: Signal<Vec<Attributes>>,
    pub surveys: Signal<Vec<Surveys>>,

    pub survey_response: Signal<Resource<models::prelude::SurveyResultDocument>>,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum SelectPage {
    Summary,
    Response,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum ResponseType {
    AttributeResponse,
    NoneResponse,
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum ResponseStatus {
    NotProgress,
    InProgress,
    Finished,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attributes {
    pub label: String,
    pub chart_datas: Vec<PiChart>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Surveys {
    pub title: String,
    pub answer: u64,
    pub skipped_answer: u64,
    pub labels: Vec<String>,
    pub value_percents: Vec<f32>,
    pub colors: Vec<String>,
    pub value_counts: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Response {
    pub response_type: ResponseType,
    pub response_status: Option<ResponseStatus>,
    pub final_update_date: String,
    pub inprogress_time: String,
    pub attribute: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Region {
    pub label: String,
    pub value: i32,
    pub percent: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Payload {
    pub label: String,
    pub value: i32,
    pub percent: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Age {
    pub label: String,
    pub value: i32,
    pub percent: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Gender {
    pub label: String,
    pub value: i32,
    pub percent: f64,
}

impl Controller {
    #[allow(unused_variables)]
    pub fn init(lang: Language, survey_id: String) -> Self {
        #[cfg(feature = "web")]
        {
            use crate::routes::Route;
            use crate::service::login_service::use_login_service;

            let navigator = use_navigator();
            let token = use_login_service().get_cookie_value();
            if token.is_none() {
                navigator.push(Route::LoginPage { lang });
            }
        }

        let res = use_resource(move || {
            let value = survey_id.clone();
            async move {
                match get_survey_result(value).await {
                    Ok(res) => {
                        tracing::debug!("this line come: {:?}", res);
                        res
                    }
                    Err(e) => {
                        tracing::error!("get survey failed: {:?}", e);
                        SurveyResultDocument::default()
                    }
                }
            }
        });

        let mut ctrl = Self {
            select_page: use_signal(|| SelectPage::Summary),
            clicked_index: use_signal(|| 0),
            attributes: use_signal(|| vec![]),
            surveys: use_signal(|| vec![]),
            panels: use_signal(|| vec![]),
            survey_response: use_signal(|| res),
        };

        ctrl.survey_response.set(res);

        let attributes = ctrl.get_attributes_chart();
        let surveys = ctrl.get_surveys_chart();
        let panels = ctrl.get_panels_chart();
        ctrl.attributes.set(attributes);
        ctrl.surveys.set(surveys);
        ctrl.panels.set(panels);
        ctrl.clicked_index.set(ctrl.get_panels().len());

        use_context_provider(|| ctrl);
        ctrl
    }

    pub fn get_total_response(&self) -> u64 {
        let survey = self.get_survey_response().clone();

        if survey.is_none() {
            return 0;
        }

        survey.unwrap().actual_responses
    }

    pub fn get_survey_response(&self) -> Option<SurveyResultDocument> {
        ((self.survey_response)().value())()
    }

    pub fn get_panels_chart(&self) -> Vec<Response> {
        let survey = self.get_survey_response().clone();

        if survey.is_none() {
            return vec![];
        }

        let answers = survey.clone().unwrap().answers;
        let quotas = survey.unwrap().quotas;
        let mut responses: Vec<Response> = vec![];

        for answer in answers {
            let responded_at = answer.responded_at;
            let timestamp_secs = responded_at / 1000;
            let timestamp_nanos = (responded_at % 1000) * 1_000_000;

            let datetime = Utc
                .timestamp_opt(timestamp_secs, timestamp_nanos as u32)
                .single()
                .expect("Invalid timestamp");

            let formatted_date = datetime.format("%Y-%m-%d").to_string();

            let quote_id = answer.quota_id;
            let quota: models::prelude::Quota = quotas
                .get(&quote_id)
                .unwrap_or(&models::prelude::Quota {
                    attribute: None,
                    panel: None,
                    quota: 0,
                })
                .clone();
            let attribute = quota.attribute;

            let mut salary = "-".to_string();
            let mut region = "-".to_string();
            let mut gender = "-".to_string();
            let mut age = "-".to_string();

            match attribute {
                Some(attr) => {
                    salary = self.get_salary(attr.salary_tier);
                    region = self.get_region(attr.region_code);
                    gender = self.get_gender(attr.gender);
                    age = self.get_age(attr.age);
                }
                None => {}
            };

            responses.push(Response {
                response_type: ResponseType::AttributeResponse,
                response_status: None,
                final_update_date: formatted_date,
                inprogress_time: "-".to_string(),
                attribute: vec![salary, region, gender, age],
            });
        }

        responses
    }

    pub fn get_surveys_chart(&self) -> Vec<Surveys> {
        let default_colors = vec![
            "#34D399".to_string(),
            "#3B82F6".to_string(),
            "#FBBF24".to_string(),
            "#F87171".to_string(),
            "#EF4444".to_string(),
        ];
        let mut surveys: Vec<Surveys> = vec![];
        let survey = self.get_survey_response();

        if survey.is_none() {
            return vec![];
        }

        let questions = survey.clone().unwrap().questions;
        let mut sorted_questions: Vec<_> = questions.iter().collect();
        sorted_questions.sort_by(|a, b| a.0.cmp(b.0));

        for (key, value) in sorted_questions {
            let s = survey.clone().unwrap().clone();
            let survey_response_by_question = s.survey_responses_by_question.get(&key);
            let mut values = vec![];
            let mut value_percent: Vec<f32> = vec![];
            let mut colors: Vec<String> = vec![];

            for _i in 0..value.options.clone().unwrap_or(vec![]).len() {
                values.push(0);
                value_percent.push(0.0);
            }

            let mut total_response = 0;

            if let Some(survey_response) = survey_response_by_question {
                for (i, option) in value.options.clone().unwrap_or(vec![]).iter().enumerate() {
                    if !survey_response_by_question.clone().is_none() {
                        let option_count = survey_response.get(option).copied().unwrap_or_default();

                        if let Some(count) = (option_count).try_into().ok() {
                            values[i] = count;
                            total_response += count;
                        }
                    }
                }

                let question_keys = survey_response_by_question
                    .map(|map| map.keys().cloned().collect::<Vec<_>>())
                    .unwrap_or_else(Vec::new);

                if total_response == 0 {
                    let survey_response = survey_response_by_question.unwrap();
                    for (i, key) in question_keys.clone().iter().enumerate() {
                        let v = survey_response.get(key).unwrap().clone();
                        total_response += v;

                        if values.len() > i {
                            values[i] = v;
                        } else {
                            values.push(v);
                            value_percent.push(0.0);
                        }
                    }
                }

                for (i, v) in values.iter().enumerate() {
                    if total_response == 0 {
                        continue;
                    }

                    value_percent[i] = (*v as f32) / (total_response as f32) * 100.0;

                    if !value.options.clone().is_none() {
                        let ind = i % 5;
                        let color = default_colors[ind].clone();
                        colors.push(color);
                    }
                }

                surveys.push(Surveys {
                    title: value.title.clone(),
                    answer: if survey_response_by_question.is_some() {
                        if total_response == 0 {
                            survey_response_by_question.unwrap().keys().len() as u64
                        } else {
                            total_response
                        }
                    } else {
                        0
                    },
                    skipped_answer: 0,
                    labels: if !value.options.clone().is_none() {
                        value.options.clone().unwrap()
                    } else {
                        question_keys
                    },
                    value_percents: value_percent,
                    colors,
                    value_counts: values,
                });
            } else {
                surveys.push(Surveys {
                    title: value.title.clone(),
                    answer: 0,
                    skipped_answer: 0,
                    labels: if !value.options.clone().is_none() {
                        value.options.clone().unwrap()
                    } else {
                        vec![]
                    },
                    value_percents: value_percent,
                    colors,
                    value_counts: values,
                });
            }
        }

        surveys
    }

    pub fn get_attributes_chart(&self) -> Vec<Attributes> {
        let mut salary_to_index_map = HashMap::new();
        let mut salary_index = 0;
        let mut total_payload = 0;
        let mut payload_data = vec![];

        let mut age_to_index_map = HashMap::new();
        let mut age_index = 0;
        let mut total_age = 0;
        let mut age_data = vec![];

        let mut region_to_index_map = HashMap::new();
        let mut region_index = 0;
        let mut total_region = 0;
        let mut region_data = vec![];

        let mut gender_to_index_map = HashMap::new();
        let mut gender_index = 0;
        let mut total_gender = 0;
        let mut gender_data = vec![];

        let colors = vec![
            "#4285f4", "#db4437", "#f4b400", "#0F9D58", "#AB47BC", "#E67E22", "#27AE60", "#3498DB",
            "#8E44AD", "#F39C12", "#2ECC71", "#1ABC9C", "#C0392B", "#2980B9", "#D35400", "#16A085",
            "#34495E",
        ];

        let survey = self.get_survey_response().clone();

        if survey.is_none() {
            return vec![];
        }

        let answers = survey.clone().unwrap().answers;
        let quotas = survey.unwrap().quotas;

        for answer in answers {
            let quote_id = answer.quota_id;
            let quota: models::prelude::Quota = quotas
                .get(&quote_id)
                .unwrap_or(&models::prelude::Quota {
                    attribute: None,
                    panel: None,
                    quota: 0,
                })
                .clone();
            let attribute = quota.attribute;

            match attribute {
                Some(attr) => {
                    let sal = self.get_salary(attr.salary_tier);

                    if sal != "-" {
                        if salary_to_index_map.get(&sal).is_none() {
                            salary_to_index_map.insert(sal.clone(), salary_index);
                            salary_index += 1;
                        }

                        let ind = salary_to_index_map.get(&sal).unwrap();

                        if *ind >= payload_data.len() {
                            payload_data.push(Payload {
                                label: sal.clone(),
                                value: 0,
                                percent: 0.0,
                            });
                        }

                        let payload = payload_data.get(*ind as usize).unwrap();
                        let payload_value = payload.value + 1;
                        payload_data[*ind as usize] = Payload {
                            value: payload_value,
                            ..payload.clone()
                        };
                        total_payload += 1;
                    }

                    let age = self.get_age(attr.age);

                    if age != "-" {
                        if age_to_index_map.get(&age).is_none() {
                            age_to_index_map.insert(age.clone(), age_index);
                            age_index += 1;
                        }

                        let ind = age_to_index_map.get(&age).unwrap();

                        if *ind >= age_data.len() {
                            age_data.push(Age {
                                label: age.clone(),
                                value: 0,
                                percent: 0.0,
                            });
                        }

                        let age = age_data.get(*ind as usize).unwrap();
                        let age_value = age.value + 1;
                        age_data[*ind as usize] = Age {
                            value: age_value,
                            ..age.clone()
                        };
                        total_age += 1;
                    }

                    let region = self.get_region(attr.region_code);

                    if region != "-" {
                        if region_to_index_map.get(&region).is_none() {
                            region_to_index_map.insert(region.clone(), region_index);
                            region_index += 1;
                        }

                        let ind = region_to_index_map.get(&region).unwrap();

                        if *ind >= region_data.len() {
                            region_data.push(Region {
                                label: region.clone(),
                                value: 0,
                                percent: 0.0,
                            });
                        }

                        let region = region_data.get(*ind as usize).unwrap();
                        let region_value = region.value + 1;
                        region_data[*ind as usize] = Region {
                            value: region_value,
                            ..region.clone()
                        };
                        total_region += 1;
                    }

                    let gender = self.get_gender(attr.gender);

                    if gender != "-" {
                        if gender_to_index_map.get(&gender).is_none() {
                            gender_to_index_map.insert(gender.clone(), gender_index);
                            gender_index += 1;
                        }

                        let ind = gender_to_index_map.get(&gender).unwrap();

                        if *ind >= gender_data.len() {
                            gender_data.push(Gender {
                                label: gender.clone(),
                                value: 0,
                                percent: 0.0,
                            });
                        }

                        let gender = gender_data.get(*ind as usize).unwrap();
                        let gender_value = gender.value + 1;
                        gender_data[*ind as usize] = Gender {
                            value: gender_value,
                            ..gender.clone()
                        };
                        total_gender += 1;
                    }
                }
                None => {}
            }
        }

        let mut payload_pi_chart = vec![];
        let mut age_pi_chart = vec![];
        let mut region_pi_chart = vec![];
        let mut gender_pi_chart = vec![];

        if total_payload != 0 {
            for (i, payload) in payload_data.iter().enumerate() {
                payload_pi_chart.push(PiChart {
                    label: payload.label.clone(),
                    percentage: ((payload.value as f64) / (total_payload as f64)),
                    color: colors.get(i).unwrap(),
                });
            }
        }

        if total_age != 0 {
            for (i, age) in age_data.iter().enumerate() {
                age_pi_chart.push(PiChart {
                    label: age.label.clone(),
                    percentage: ((age.value as f64) / (total_age as f64)),
                    color: colors.get(i).unwrap(),
                });
            }
        }

        if total_region != 0 {
            for (i, region) in region_data.iter().enumerate() {
                region_pi_chart.push(PiChart {
                    label: region.label.clone(),
                    percentage: ((region.value as f64) / (total_region as f64)),
                    color: colors.get(i).unwrap(),
                });
            }
        }

        if total_gender != 0 {
            for (i, gender) in gender_data.iter().enumerate() {
                gender_pi_chart.push(PiChart {
                    label: gender.label.clone(),
                    percentage: ((gender.value as f64) / (total_gender as f64)),
                    color: colors.get(i).unwrap(),
                });
            }
        }

        let mut attributes = vec![];

        if total_payload != 0 {
            attributes.push(Attributes {
                label: "연봉".to_string(),
                chart_datas: payload_pi_chart,
            });
        }

        if total_gender != 0 {
            attributes.push(Attributes {
                label: "성별".to_string(),
                chart_datas: gender_pi_chart,
            });
        }

        if total_region != 0 {
            attributes.push(Attributes {
                label: "지역".to_string(),
                chart_datas: region_pi_chart,
            });
        }

        if total_age != 0 {
            attributes.push(Attributes {
                label: "연령".to_string(),
                chart_datas: age_pi_chart,
            })
        }

        attributes
    }

    pub fn get_attributes(&self) -> Vec<Attributes> {
        (self.attributes)()
    }

    pub fn get_surveys(&self) -> Vec<Surveys> {
        (self.surveys)()
    }

    pub fn get_clicked_index(&self) -> usize {
        (self.clicked_index)()
    }

    pub fn change_clicked_index(&mut self, index: usize) {
        self.clicked_index.set(index);
    }

    pub fn get_panels(&self) -> Vec<Response> {
        (self.panels)()
    }

    pub fn change_select_page(&mut self, select_page: SelectPage) {
        self.select_page.set(select_page);
    }

    pub fn get_select_page(&self) -> SelectPage {
        (self.select_page)()
    }

    pub fn get_age(&self, age: Option<models::prelude::Age>) -> String {
        match age {
            Some(a) => match a {
                models::prelude::Age::Specific(a) => format!("{}세", a),
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
                        format!("{}~{}세", inclusive_min.unwrap(), inclusive_max.unwrap())
                    }
                }
            },
            None => "-".to_string(),
        }
    }

    pub fn get_gender(&self, gender: Option<models::prelude::Gender>) -> String {
        match gender {
            Some(g) => {
                if g == models::prelude::Gender::Male {
                    "남성".to_string()
                } else {
                    "여성".to_string()
                }
            }
            None => "-".to_string(),
        }
    }

    pub fn get_region(&self, region_code: Option<u16>) -> String {
        match region_code {
            Some(r) => {
                if r == 02 {
                    "서울".to_string()
                } else if r == 051 {
                    "부산".to_string()
                } else if r == 053 {
                    "대구".to_string()
                } else if r == 032 {
                    "인천".to_string()
                } else if r == 062 {
                    "광주".to_string()
                } else if r == 042 {
                    "대전".to_string()
                } else if r == 052 {
                    "울산".to_string()
                } else if r == 044 {
                    "세종".to_string()
                } else if r == 031 {
                    "경기".to_string()
                } else if r == 033 {
                    "강원".to_string()
                } else if r == 043 {
                    "충북".to_string()
                } else if r == 041 {
                    "충남".to_string()
                } else if r == 063 {
                    "전북".to_string()
                } else if r == 061 {
                    "전남".to_string()
                } else if r == 054 {
                    "경북".to_string()
                } else if r == 055 {
                    "경남".to_string()
                } else {
                    "제주".to_string()
                }
            }
            None => "-".to_string(),
        }
    }

    pub fn get_salary(&self, salary_tier: Option<u16>) -> String {
        match salary_tier {
            Some(tier) => {
                if tier == 1 {
                    "2400만원 이하".to_string()
                } else if tier == 2 {
                    "2400만원~5000만원".to_string()
                } else if tier == 3 {
                    "5000만원~8000만원".to_string()
                } else if tier == 4 {
                    "8000만원~10000만원".to_string()
                } else {
                    "10000만원 이상".to_string()
                }
            }
            None => "-".to_string(),
        }
    }
}

#[allow(dead_code)]
pub fn use_controller() -> Controller {
    use_context()
}
