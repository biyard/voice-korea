#![allow(unused)]
use std::{str::FromStr, time::SystemTime};

use crate::{attribute_v2::*, PanelV2, Result};
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::api_model;
use by_types::QueryResponse;
use dioxus_translate::Language;

use crate::attribute_v2::{GenderV2, RegionV2, SalaryV2};

use super::{ChoiceQuestion, Question};
use serde::{Deserialize, Serialize};

#[api_model(base = "/v2/surveys/:survey-id/responses", table = survey_responses)]
pub struct SurveyResponse {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, version = v0.1)]
    pub panel_id: i64,

    #[api_model(action = respond_answer)]
    pub proof_id: String,

    #[api_model(summary, action = respond_answer, type = JSONB)]
    pub attributes: Vec<Attribute>,

    #[api_model(summary, action = respond_answer, type = JSONB)]
    pub answers: Vec<Answer>,

    #[api_model(many_to_one = surveys)]
    pub survey_id: i64,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "snake_case", tag = "answer_type")]
pub enum Answer {
    SingleChoice { answer: i32 },
    MultipleChoice { answer: Vec<i32> },
    ShortAnswer { answer: String },
    Subjective { answer: String },
}

impl Answer {
    pub fn simulate(q: &Question) -> Self {
        let bytes = format!("{:?}", q);
        use sha3::Digest;

        let mut hasher = sha3::Sha3_256::new();
        hasher.update(bytes.as_bytes());
        let result = hasher.finalize();
        let random = result.to_vec();

        let answer_pool = [
            "Lorem ipsum",
            "Dolor sit amet",
            "Consectetur adipiscing elit",
            "Sed do eiusmod tempor",
            "Incididunt ut labore",
            "Et dolore magna aliqua",
            "Ut enim ad minim veniam",
            "Quis nostrud exercitation",
            "Ullamco laboris nisi",
            "Ut aliquip ex ea commodo",
            "Consequ",
        ];

        match q {
            Question::SingleChoice(ChoiceQuestion { options, .. }) => Answer::SingleChoice {
                answer: random[0] as i32 % options.len() as i32,
            },
            Question::MultipleChoice(ChoiceQuestion { options, .. }) => {
                let mut half = options.len() / 2;

                if half == 0 && options.len() > 0 {
                    half = 1;
                }

                let mut answer = vec![];

                for i in 0..half {
                    answer.push(random[i] as i32 % options.len() as i32);
                }

                Answer::MultipleChoice { answer }
            }
            Question::ShortAnswer(_) => Answer::ShortAnswer {
                answer: answer_pool[random[0] as usize % answer_pool.len()].to_string(),
            },
            Question::Subjective(_) => Answer::Subjective {
                answer: answer_pool[random[0] as usize % answer_pool.len()].to_string(),
            },
        }
    }
    pub fn to_answer_string(&self) -> String {
        match self {
            Answer::SingleChoice { answer } => answer.to_string(),
            Answer::MultipleChoice { answer } => answer
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(","),
            Answer::ShortAnswer { answer } => answer.to_string(),
            Answer::Subjective { answer } => answer.to_string(),
        }
    }
}

impl PartialEq<Question> for Answer {
    fn eq(&self, other: &Question) -> bool {
        match (self, other) {
            (Answer::SingleChoice { .. }, Question::SingleChoice(_)) => true,
            (Answer::MultipleChoice { .. }, Question::MultipleChoice(_)) => true,
            (Answer::ShortAnswer { .. }, Question::ShortAnswer(_)) => true,
            (Answer::Subjective { .. }, Question::Subjective(_)) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "snake_case")]
pub enum Attribute {
    #[serde(alias = "age")]
    Age(AgeV3),
    #[serde(alias = "gender")]
    Gender(GenderV2),
    #[serde(alias = "region")]
    Region(RegionV2),
    #[serde(alias = "salary")]
    Salary(SalaryV2),

    #[default]
    None,
}

impl Attribute {
    pub fn from_panel(panel: &PanelV2) -> Vec<Self> {
        let mut attrs = vec![];
        let attributes = panel.attributes.clone();

        for attribute in attributes {
            match attribute {
                Attribute::Age(age_v3) => {
                    let (min, max) = age_v3.to_range();
                    attrs.push(Attribute::Age(AgeV3::Range {
                        inclusive_min: min,
                        inclusive_max: max,
                    }));
                }
                Attribute::Gender(gender_v2) => {
                    attrs.push(Attribute::Gender(gender_v2));
                }
                Attribute::Region(region_v2) => {
                    attrs.push(Attribute::Region(region_v2));
                }
                Attribute::Salary(salary_v2) => {
                    attrs.push(Attribute::Salary(salary_v2));
                }
                Attribute::None => {}
            }
        }

        attrs
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "snake_case")]
pub enum AgeV3 {
    #[serde(alias = "specific")]
    Specific(u8),
    #[serde(alias = "range")]
    Range {
        inclusive_min: u8,
        inclusive_max: u8,
    },
    #[serde(alias = "none")]
    #[default]
    None,
}

impl FromStr for AgeV3 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = s.trim();

        //FIXME: fix to valid value return
        if let Ok(num) = s.parse::<u8>() {
            return Ok(AgeV3::Specific(num));
        }

        match s {
            "Under 17" | "17세 이하" => Ok(AgeV3::Range {
                inclusive_min: 0,
                inclusive_max: 17,
            }),
            "18-29 years old" | "18~29세" => Ok(AgeV3::Range {
                inclusive_min: 18,
                inclusive_max: 29,
            }),
            "30-39 years old" | "30대" => Ok(AgeV3::Range {
                inclusive_min: 30,
                inclusive_max: 39,
            }),
            "40-49 years old" | "40대" => Ok(AgeV3::Range {
                inclusive_min: 40,
                inclusive_max: 49,
            }),
            "50-59 years old" | "50대" => Ok(AgeV3::Range {
                inclusive_min: 50,
                inclusive_max: 59,
            }),
            "60-69 years old" | "60대" => Ok(AgeV3::Range {
                inclusive_min: 60,
                inclusive_max: 69,
            }),
            "Over 70" | "70대 이상" => Ok(AgeV3::Range {
                inclusive_min: 70,
                inclusive_max: 100,
            }),
            "None" | "없음" => Ok(AgeV3::None),
            _ => Err(format!("Invalid age value: {}", s)),
        }
    }
}

impl AgeV3 {
    pub fn translate(&self, lang: &Language) -> String {
        match lang {
            Language::En => match self {
                AgeV3::Specific(v) => format!("{v} years old"),
                AgeV3::Range {
                    inclusive_min,
                    inclusive_max,
                } => {
                    if *inclusive_max == 17 {
                        "Under 17".to_string()
                    } else if *inclusive_min == 18 && *inclusive_max == 29 {
                        "18-29 years old".to_string()
                    } else if *inclusive_min == 70 {
                        format!("Over {inclusive_min}")
                    } else {
                        format!("{inclusive_min}-{inclusive_max} years old")
                    }
                }
                AgeV3::None => format!("None"),
            },
            Language::Ko => match self {
                AgeV3::Specific(v) => format!("{v}세"),
                AgeV3::Range {
                    inclusive_min,
                    inclusive_max,
                } => {
                    if *inclusive_max == 17 {
                        "17세 이하".to_string()
                    } else if *inclusive_min == 18 && *inclusive_max == 29 {
                        "18-29세".to_string()
                    } else if *inclusive_min == 70 {
                        format!("{inclusive_min}대 이상")
                    } else {
                        format!("{inclusive_min}대")
                    }
                }
                AgeV3::None => "없음".to_string(),
            },
        }
    }
    pub fn to_range(&self) -> (u8, u8) {
        match self {
            AgeV3::None => (0, 100),
            AgeV3::Specific(v) => (v.clone(), v.clone()),
            AgeV3::Range {
                inclusive_min: 0,
                inclusive_max: 17,
            } => (0, 17),
            AgeV3::Range {
                inclusive_min: 18,
                inclusive_max: 29,
            } => (18, 29),
            AgeV3::Range {
                inclusive_min: 30,
                inclusive_max: 39,
            } => (30, 39),
            AgeV3::Range {
                inclusive_min: 40,
                inclusive_max: 49,
            } => (40, 49),
            AgeV3::Range {
                inclusive_min: 50,
                inclusive_max: 59,
            } => (50, 59),
            AgeV3::Range {
                inclusive_min: 60,
                inclusive_max: 69,
            } => (60, 69),
            _ => (70, 100),
        }
    }
}
