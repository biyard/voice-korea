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
        let mut age: Vec<NonceLabAge> = vec![];
        let mut gender: Vec<u8> = vec![];
        let mut region: Vec<RegionCode> = vec![];
        let mut salary: Vec<SalaryTier> = vec![];

        for attribute in attributes.clone() {
            match attribute {
                Attribute::Age(age_v3) => {
                    match age_v3 {
                        AgeV3::Specific(v) => {
                            let a = NonceLabAge::Specific(v);
                            age.push(a);
                        }
                        AgeV3::Range {
                            inclusive_min,
                            inclusive_max,
                        } => {
                            let a = NonceLabAge::Range {
                                inclusive_min,
                                inclusive_max,
                            };
                            age.push(a);
                        }
                        AgeV3::None => {}
                    };
                }
                Attribute::Gender(gender_v2) => {
                    let g = gender_v2 as u8;
                    gender.push(g);
                }
                Attribute::Region(region_v2) => {
                    let r = region_v2.into();
                    region.push(r);
                }
                Attribute::Salary(salary_v2) => {
                    let s = salary_v2 as SalaryTier;
                    salary.push(s);
                }
                Attribute::None => {}
            }
        }

        NonceLabQuota {
            id: None,
            attribute: Some(NonceLabAttribute {
                salary_tier: salary,
                region_code: region,
                gender_code: gender,
                age,
            }),
            panel: None,
            quota: user_count,
        }
    }
}

#[derive(Serialize, serde::Deserialize, Debug)]
pub struct NonceLabAttribute {
    // e.g. 1, 2, 3, 4, 5
    pub salary_tier: Vec<SalaryTier>,
    // e.g. 02(Seoul), 051(Busan) and so on.
    pub region_code: Vec<RegionCode>,
    pub gender_code: Vec<u8>,
    pub age: Vec<NonceLabAge>,
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
