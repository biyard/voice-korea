use attribute_v2::{GenderV2, RegionV2, SalaryV2};
use models::{
    response::{AgeV3, Attribute},
    *,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
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
    pub estimated_minutes: u64,
    pub reward_points: u64,
}

impl From<SurveyV2> for NonceLabCreateSurveyRequest {
    fn from(survey: SurveyV2) -> Self {
        let panel_counts = survey.panel_counts;
        let quotas = survey
            .panels
            .into_iter()
            .map(|q| {
                let mut nq: NonceLabQuota = q.clone().into();

                let d: Vec<PanelCountsV2> = panel_counts
                    .iter()
                    .filter(|v| v.panel_id == q.id.clone() as i64)
                    .map(|v| v.clone())
                    .collect();

                let v = match d.get(0) {
                    Some(v) => v.user_count,
                    None => 0,
                };

                nq.quota = v as u64;
                nq
            })
            .collect();
        let questions = survey.questions.into_iter().map(|q| q.into()).collect();
        NonceLabCreateSurveyRequest {
            custom_id: survey.id.to_string(),
            status: survey.status.into(),
            started_at: survey.started_at,
            ended_at: survey.ended_at,
            title: survey.name,
            quotas,
            questions,
            description: if !survey.description.is_empty() {
                Some(survey.description)
            } else {
                None
            },
            expected_responses: survey.quotes as u64,
            estimated_minutes: survey.estimate_time as u64,
            reward_points: survey.point as u64,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NonceLabQuota {
    pub id: Option<u32>,
    pub attribute: Option<NonceLabAttribute>,
    pub panel: Option<SurveyPanel>,
    pub quota: u64,
}

impl From<PanelV2> for NonceLabQuota {
    fn from(
        PanelV2 {
            user_count,
            attributes,
            ..
        }: PanelV2,
    ) -> Self {
        let mut age = None;
        let mut gender = None;
        let mut region = None;
        let mut salary = None;

        for attribute in attributes.clone() {
            match attribute {
                Attribute::Age(age_v3) => {
                    age = Some(age_v3);
                }
                Attribute::Gender(gender_v2) => {
                    gender = Some(gender_v2);
                }
                Attribute::Region(region_v2) => {
                    region = Some(region_v2);
                }
                Attribute::Salary(salary_v2) => {
                    salary = Some(salary_v2);
                }
                Attribute::None => {}
            }
        }

        NonceLabQuota {
            id: None,
            attribute: Some(NonceLabAttribute {
                salary_tier: match salary {
                    None => None,
                    Some(v) => match v {
                        SalaryV2::None => None,
                        t => Some(t as SalaryTier),
                    },
                },
                region_code: match region {
                    None => None,
                    Some(c) => match c {
                        RegionV2::None => None,
                        c => match c {
                            RegionV2::Seoul => Some(02 as RegionCode),
                            RegionV2::Busan => Some(051 as RegionCode),
                            RegionV2::Daegu => Some(053 as RegionCode),
                            RegionV2::Incheon => Some(032 as RegionCode),
                            RegionV2::Gwangju => Some(062 as RegionCode),
                            RegionV2::Daejeon => Some(042 as RegionCode),
                            RegionV2::Ulsan => Some(052 as RegionCode),
                            RegionV2::Sejong => Some(044 as RegionCode),
                            RegionV2::Gyeonggi => Some(031 as RegionCode),
                            RegionV2::Gangwon => Some(033 as RegionCode),
                            RegionV2::Chungbuk => Some(043 as RegionCode),
                            RegionV2::Chungnam => Some(041 as RegionCode),
                            RegionV2::Jeonbuk => Some(063 as RegionCode),
                            RegionV2::Jeonnam => Some(061 as RegionCode),
                            RegionV2::Gyeongbuk => Some(054 as RegionCode),
                            RegionV2::Gyeongnam => Some(055 as RegionCode),
                            RegionV2::Jeju => Some(064 as RegionCode),
                            _ => Some(0 as RegionCode),
                        },
                    },
                },
                gender_code: match gender {
                    None => None,
                    Some(c) => match c {
                        GenderV2::None => None,
                        c => Some(c as u8),
                    },
                },
                age: match age {
                    None => None,
                    Some(a) => match a {
                        AgeV3::None => None,
                        a => Some(a.try_into().expect("Invalid Age")),
                    },
                },
            }),
            panel: None,
            quota: user_count,
        }
    }
}

#[derive(Serialize, serde::Deserialize, Debug)]
pub struct NonceLabAttribute {
    // e.g. 1, 2, 3, 4, 5
    pub salary_tier: Option<SalaryTier>,
    // e.g. 02(Seoul), 051(Busan) and so on.
    pub region_code: Option<RegionCode>,
    pub gender_code: Option<u8>,
    pub age: Option<NonceLabAge>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum SurveyStatus {
    #[default]
    #[serde(rename = "draft")]
    Draft = 1,
    #[serde(rename = "in_progress")]
    InProgress = 2,
    #[serde(rename = "finished")]
    Finished = 3,
}

impl From<ProjectStatus> for SurveyStatus {
    fn from(status: ProjectStatus) -> Self {
        match status {
            ProjectStatus::Ready => SurveyStatus::Draft,
            ProjectStatus::InProgress => SurveyStatus::InProgress,
            ProjectStatus::Finish => SurveyStatus::Finished,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct NonceLabSurveyQuestion {
    title: String,
    question: NonceLabSurveyQuestionType,
}

impl From<Question> for NonceLabSurveyQuestion {
    fn from(question: Question) -> Self {
        // NOTE: Noncelab API does not support description field for each question.
        match question {
            Question::SingleChoice(ChoiceQuestion {
                title,
                description,
                options,
                ..
            }) => NonceLabSurveyQuestion {
                title: title.clone(),
                question: NonceLabSurveyQuestionType::SingleChoice {
                    question: description.unwrap_or_default(),
                    options,
                },
            },
            Question::MultipleChoice(ChoiceQuestion {
                title,
                description,
                options,
                ..
            }) => NonceLabSurveyQuestion {
                title: title.clone(),
                question: NonceLabSurveyQuestionType::MultipleChoice {
                    question: description.unwrap_or_default(),
                    options,
                },
            },
            Question::ShortAnswer(SubjectiveQuestion { title, description }) => {
                NonceLabSurveyQuestion {
                    title: title.clone(),
                    question: NonceLabSurveyQuestionType::Text(description),
                }
            }
            Question::Subjective(SubjectiveQuestion { title, description }) => {
                NonceLabSurveyQuestion {
                    title: title.clone(),
                    question: NonceLabSurveyQuestionType::LongText(description),
                }
            }
        }
    }
}

#[derive(Serialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum NonceLabAge {
    Specific(u8),
    Range {
        inclusive_min: u8,
        inclusive_max: u8,
    },
}

impl TryFrom<AgeV3> for NonceLabAge {
    type Error = ();

    fn try_from(value: AgeV3) -> std::result::Result<Self, Self::Error> {
        match value {
            AgeV3::None => Err(()),
            AgeV3::Specific(v) => Ok(NonceLabAge::Specific(v)),
            AgeV3::Range {
                inclusive_min,
                inclusive_max,
            } => Ok(NonceLabAge::Range {
                inclusive_min,
                inclusive_max,
            }),
        }
    }
}

// SalaryTier means the annual salary range of the respondent.
// 0: 0 ~ 9,999,999
// 1: 10,000,000 ~ 19,999,999
// 2: 20,000,000 ~ 29,999,999
// ..
pub type SalaryTier = u16;

pub type RegionCode = u16;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NonceLabCreateSurveyResponse {
    pub id: u32,
}
