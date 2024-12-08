use std::collections::HashMap;

use models::prelude::{
    Age, Gender, Panel, Quota, RegionCode, SalaryTier, SurveyQuestion, SurveyQuestionType,
    SurveyStatus,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct NonceLabCreateSurveyRequest {
    pub custom_id: String,
    pub status: SurveyStatus,
    pub started_at: i64,
    pub ended_at: i64,
    pub title: String,
    pub quotas: Vec<NonceLabQuota>,
    pub questions: Vec<NonceLabSurveyQuestion>,
    pub description: Option<String>,
    pub expected_responses: u64,
}

#[derive(Serialize)]
pub struct NonceLabSurveyQuestion {
    title: String,
    question: NonceLabSurveyQuestionType,
}
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NonceLabSurveyQuestionType {
    SingleChoice {
        question: String,
        options: Vec<String>,
    },
    MultipleChoice {
        question: String,
        options: Vec<String>,
    },
    LongText(String),
    Text(String),
}

impl From<SurveyQuestion> for NonceLabSurveyQuestion {
    fn from(question: SurveyQuestion) -> Self {
        let question_type = match question.answer_type {
            SurveyQuestionType::MultipleChoice => NonceLabSurveyQuestionType::MultipleChoice {
                question: question.title.clone(),
                options: question.options.unwrap_or_default().clone(),
            },
            SurveyQuestionType::SingleChoice => NonceLabSurveyQuestionType::SingleChoice {
                question: question.title.clone(),
                options: question.options.unwrap_or_default().clone(),
            },
            SurveyQuestionType::LongAnswer => {
                NonceLabSurveyQuestionType::LongText(question.title.clone())
            }
            SurveyQuestionType::ShortAnswer => {
                NonceLabSurveyQuestionType::Text(question.title.clone())
            }
        };
        NonceLabSurveyQuestion {
            title: question.title,
            question: question_type,
        }
    }
}

#[derive(Serialize)]
pub struct NonceLabQuota {
    pub attribute: Option<NonceLabAttribute>,
    pub panel: Option<Panel>,
    pub quota: u64,
}

impl From<Quota> for NonceLabQuota {
    fn from(quota: Quota) -> Self {
        let attribute = if let Some(attribute) = quota.attribute {
            let age = if let Some(age) = attribute.age {
                Some(match age {
                    Age::Range {
                        inclusive_min,
                        inclusive_max,
                    } => NonceLabAge::Range {
                        inclusive_min: inclusive_min.unwrap_or_default(),
                        inclusive_max: inclusive_max.unwrap_or_default(),
                    },
                    Age::Specific(v) => NonceLabAge::Specific(v),
                })
            } else {
                None
            };
            let region_code = if let Some(code) = attribute.region_code {
                convert_area_code_to_region_code(code)
            } else {
                None
            };
            let gender_code = if let Some(gender) = attribute.gender {
                Some(match gender {
                    Gender::Male => 1,
                    Gender::Female => 2,
                    Gender::Others => 3,
                })
            } else {
                None
            };
            Some(NonceLabAttribute {
                salary_tier: attribute.salary_tier,
                region_code,
                gender_code,
                age,
            })
        } else {
            None
        };

        NonceLabQuota {
            panel: quota.panel,
            quota: quota.quota,
            attribute,
        }
    }
}
#[derive(Serialize)]
pub struct NonceLabAttribute {
    // e.g. 1, 2, 3, 4, 5
    pub salary_tier: Option<SalaryTier>,
    // e.g. 02(Seoul), 051(Busan) and so on.
    pub region_code: Option<RegionCode>,
    pub gender_code: Option<u8>,
    pub age: Option<NonceLabAge>,
}

#[derive(Serialize)]
pub enum NonceLabAge {
    Specific(u8),
    Range {
        inclusive_min: u8,
        inclusive_max: u8,
    },
}

fn convert_area_code_to_region_code(area_code: u16) -> Option<u16> {
    let mut area_to_region_map = HashMap::new();
    area_to_region_map.insert(2, 11); // 서울
    area_to_region_map.insert(51, 21); // 부산
    area_to_region_map.insert(53, 22); // 대구
    area_to_region_map.insert(32, 23); // 인천
    area_to_region_map.insert(62, 24); // 광주
    area_to_region_map.insert(42, 25); // 대전
    area_to_region_map.insert(52, 26); // 울산
    area_to_region_map.insert(44, 29); // 세종
    area_to_region_map.insert(31, 31); // 경기
    area_to_region_map.insert(33, 32); // 강원
    area_to_region_map.insert(43, 33); // 충북
    area_to_region_map.insert(41, 34); // 충남
    area_to_region_map.insert(63, 35); // 전북
    area_to_region_map.insert(61, 36); // 전남
    area_to_region_map.insert(54, 37); // 경북
    area_to_region_map.insert(55, 38); // 경남
    area_to_region_map.insert(64, 39); // 제주

    area_to_region_map.get(&area_code).copied()
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct NonceLabCreateSurveyResponse {
    pub id: u32,
    // pub custom_id: Option<String>,
    // pub status: SurveyStatus,
    // pub started_at: String,
    // pub ended_at: String,
    // pub title: String,
    // pub description: String,
    // pub questions: u32,
    // pub responses: Option<bool>,
    // pub estimated_minutes: u32,
    // pub expected_responses: u32,
    // pub created_at: String,
    // pub updated_at: String,
    // pub responders: Vec<String>,
}
