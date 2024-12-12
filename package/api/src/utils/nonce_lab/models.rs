use std::collections::HashMap;

use models::prelude::{
    Age, Attribute, Gender, Panel, Quota, RegionCode, SalaryTier, SurveyQuestion,
    SurveyQuestionType, SurveyStatus,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct NonceLabGetSurveyDto {
    // pub id: u32,
    // pub title: String,
    // pub description: Option<String>,
    // pub status: SurveyStatus,
    // #[serde(rename = "startedAt")]
    // pub created_at: String,
    // #[serde(rename = "endedAt")]
    // pub ended_at: String,
    // #[serde(rename = "rewardPoints")]
    // pub reward_points: u32,
    // #[serde(rename = "questionCount")]
    // pub question_count: u32,
    // participated: bool,
    // #[serde(rename = "estimatedMinutes")]
    // pub estimated_minutes: u32,
    // pub quotas: Vec<Quota>,
    // pub questions: Vec<SurveyQuestion>,
    #[serde(rename = "responseCountMap")]
    pub response_count_map: Option<HashMap<u32, u64>>,
}

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

#[derive(Serialize, Deserialize, Debug)]
pub struct NonceLabQuota {
    pub id: Option<u32>,
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
            id: None,
            panel: quota.panel,
            quota: quota.quota,
            attribute,
        }
    }
}

impl Into<Quota> for NonceLabQuota {
    fn into(self) -> Quota {
        let attribute = if let Some(attribute) = self.attribute {
            let region_code = if let Some(code) = attribute.region_code {
                convert_region_code_to_area_code(code)
            } else {
                None
            };

            let gender = if let Some(gender) = attribute.gender_code {
                Some(match gender {
                    1_u8 => Gender::Male,
                    2_u8 => Gender::Female,
                    _ => Gender::Others,
                })
            } else {
                None
            };

            let age = if let Some(age) = attribute.age {
                Some(match age {
                    NonceLabAge::Range {
                        inclusive_min,
                        inclusive_max,
                    } => Age::Range {
                        inclusive_min: Some(inclusive_min),
                        inclusive_max: Some(inclusive_max),
                    },
                    NonceLabAge::Specific(v) => Age::Specific(v),
                })
            } else {
                None
            };

            Some(Attribute {
                salary_tier: attribute.salary_tier,
                region_code,
                gender,
                age,
            })
        } else {
            None
        };

        Quota {
            attribute,
            panel: self.panel,
            quota: self.quota,
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct NonceLabAttribute {
    // e.g. 1, 2, 3, 4, 5
    pub salary_tier: Option<SalaryTier>,
    // e.g. 02(Seoul), 051(Busan) and so on.
    pub region_code: Option<RegionCode>,
    pub gender_code: Option<u8>,
    pub age: Option<NonceLabAge>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NonceLabAge {
    Specific(u8),
    Range {
        inclusive_min: u8,
        inclusive_max: u8,
    },
}

fn convert_area_code_to_region_code(area_code: u16) -> Option<u16> {
    match area_code {
        2 => Some(11),  // 서울
        51 => Some(21), // 부산
        53 => Some(22), // 대구
        32 => Some(23), // 인천
        62 => Some(24), // 광주
        42 => Some(25), // 대전
        52 => Some(26), // 울산
        44 => Some(29), // 세종
        31 => Some(31), // 경기
        33 => Some(32), // 강원
        43 => Some(33), // 충북
        41 => Some(34), // 충남
        63 => Some(35), // 전북
        61 => Some(36), // 전남
        54 => Some(37), // 경북
        55 => Some(38), // 경남
        64 => Some(39), // 제주
        _ => None,
    }
}

fn convert_region_code_to_area_code(region_code: u16) -> Option<u16> {
    match region_code {
        11 => Some(2),  // 서울
        21 => Some(51), // 부산
        22 => Some(53), // 대구
        23 => Some(32), // 인천
        24 => Some(62), // 광주
        25 => Some(42), // 대전
        26 => Some(52), // 울산
        29 => Some(44), // 세종
        31 => Some(31), // 경기
        32 => Some(33), // 강원
        33 => Some(43), // 충북
        34 => Some(41), // 충남
        35 => Some(63), // 전북
        36 => Some(61), // 전남
        37 => Some(54), // 경북
        38 => Some(55), // 경남
        39 => Some(64), // 제주
        _ => None,
    }
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NonceLabSurveyResultResponse {
    pub quotas: Vec<NonceLabQuota>,
    pub response_array: Vec<NonceLabSurveyResponse>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NonceLabSurveyResponse {
    // pub id: u32,
    pub quota_id: u32,
    pub responded_at: String,
    pub answers: Vec<NonceLabSurveyResultAnswer>,
}

// Nonce Lab Responses

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NonceLabSurveyResultAnswer {
    // pub id: u32,
    pub text_answer: Option<String>,
    pub choice_answer: Option<Vec<String>>,
}
