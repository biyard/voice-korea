use core::fmt;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Survey {
    pub id: String,
    pub r#type: String,
    pub gsi1: String,
    pub gsi2: String,
    pub creator: String,
    pub created_at: i64,
    pub updated_at: i64,

    pub status: SurveyStatus,
    pub draft_status: Option<SurveyDraftStatus>,
    pub title: String,
    pub quotas: Vec<Quota>,
    pub questions: Vec<SurveyQuestion>,
    pub started_at: Option<i64>,
    pub ended_at: Option<i64>,
    pub nonce_lab_id: Option<u32>,
    pub result_id: Option<String>,
    pub response_count: Option<u64>,
    pub total_response_count: Option<u64>,
}
impl Survey {
    pub fn new(id: String, user_id: String) -> Self {
        let mut survey = Survey::default();
        let now = chrono::Utc::now().timestamp_millis();
        survey.gsi1 = Survey::get_gsi1(&user_id);
        survey.gsi2 = Survey::get_gsi2("".to_string());
        survey.id = id;
        survey.creator = user_id;
        survey.r#type = Survey::get_type();
        survey.draft_status = Some(SurveyDraftStatus::Init);
        survey.created_at = now;
        survey.updated_at = now;
        survey
    }
    pub fn get_gsi1(user_id: &str) -> String {
        format!("{}#{}", Self::get_type(), user_id)
    }
    pub fn get_gsi2(ended_at: String) -> String {
        // e.g. 2024-12-31
        format!("{}#endedAt#{}", Self::get_type(), ended_at)
    }
    pub fn get_type() -> String {
        "survey".to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum SurveyStatus {
    #[default]
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "finished")]
    Finished,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum SurveyDraftStatus {
    #[default]
    Init,
    Title,
    Quotas,
    Question,
    Complete,
}

impl fmt::Display for SurveyStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SurveyStatus::Draft => write!(f, "draft"),
            SurveyStatus::InProgress {} => {
                write!(f, "in_progress")
            }
            SurveyStatus::Finished {} => write!(f, "finished"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SurveyQuestion {
    #[serde(rename = "id")]
    pub question_id: Option<u64>,
    pub title: String,
    pub description: Option<String>,
    #[serde(rename = "answerType")]
    pub answer_type: SurveyQuestionType,
    pub options: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SurveyQuestionType {
    #[serde(rename = "MULTIPLE_CHOICE")]
    MultipleChoice,
    #[serde(rename = "SINGLE_CHOICE")]
    SingleChoice,
    #[serde(rename = "LONG_ANSWER")]
    LongAnswer,
    #[serde(rename = "SHORT_ANSWER")]
    ShortAnswer,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum QuestionSequence {
    #[default]
    #[serde(rename = "title")]
    Title,
    #[serde(rename = "add_question")]
    AddQuestion,
    #[serde(rename = "select_response")]
    SelectResponse,
    #[serde(rename = "summary")]
    Summary,
}

impl fmt::Display for QuestionSequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QuestionSequence::Title => write!(f, "title"),
            QuestionSequence::AddQuestion => write!(f, "add_question"),
            QuestionSequence::SelectResponse => write!(f, "select_response"),
            QuestionSequence::Summary => write!(f, "summary"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Quota {
    pub attribute: Option<Attribute>,
    pub panel: Option<Panel>,
    pub quota: u64,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Attribute {
    // e.g. 1, 2, 3, 4, 5
    pub salary_tier: Option<SalaryTier>,
    // e.g. 02(Seoul), 051(Busan) and so on.
    pub region_code: Option<RegionCode>,
    pub gender: Option<Gender>,
    pub age: Option<Age>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Panel {
    panel: Vec<ProofId>,
}

// SalaryTier means the annual salary range of the respondent.
// 0: 0 ~ 9,999,999
// 1: 10,000,000 ~ 19,999,999
// 2: 20,000,000 ~ 29,999,999
// ..
pub type SalaryTier = u16;
pub type RegionCode = u16;
pub type ProofId = String;

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Gender {
    Male,
    Female,
    Others,
}

impl<'de> Deserialize<'de> for Gender {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "male" | "남자" => Ok(Gender::Male),
            "female" | "여자" => Ok(Gender::Female),
            "others" | "기타" => Ok(Gender::Others),
            _ => Err(serde::de::Error::custom(format!("Invalid gender: {}", s))),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Age {
    Specific(u8),
    Range {
        #[serde(rename = "inclusiveMin")]
        inclusive_min: Option<u8>,
        #[serde(rename = "inclusiveMax")]
        inclusive_max: Option<u8>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressSurveyResponse {
    pub id: String,
    pub nonce_lab_id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteSurveyResponse {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSurveyRequest {
    pub size: Option<i32>,
    pub bookmark: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSurveyResponse {
    pub bookmark: Option<String>,
    pub survey: Vec<Survey>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CreateSurveyDto {
    pub custom_id: String,
    pub status: SurveyStatus,
    pub started_at: i64,
    pub ended_at: i64,
    pub title: String,
    pub quotas: Vec<Quota>,
    pub questions: Vec<SurveyQuestion>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UpsertSurveyDraftRequest {
    pub id: Option<String>,
    pub status: Option<SurveyDraftStatus>,
    pub title: Option<String>,
    pub quotas: Option<Vec<Quota>>,
    pub questions: Option<Vec<SurveyQuestion>>,
    pub started_at: Option<i64>,
    pub ended_at: Option<i64>,
}

pub type QuotaId = u32;
pub type QuestionId = u32;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SurveyResultDocument {
    pub id: String,
    pub user_id: String,
    pub r#type: String,
    pub gsi1: String,
    pub created_at: i64,
    pub survey_id: String,

    pub expected_responses: u64,
    pub actual_responses: u64,

    pub answers: Vec<SurveyResultAnswer>,

    pub quotas: HashMap<QuotaId, Quota>,
    pub questions: HashMap<QuestionId, SurveyQuestion>,

    pub survey_responses_by_quota: HashMap<QuotaId, u64>,
    pub survey_responses_by_question: HashMap<QuestionId, HashMap<String, u64>>,
}

impl SurveyResultDocument {
    pub fn get_type() -> String {
        "survey_result".to_string()
    }
    pub fn get_gsi1(user_id: &str, survey_id: &str) -> String {
        format!("survey#result#{}#{}", user_id, survey_id)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SurveyResultQuota {
    pub quota_id: String,
    pub total_num: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SurveyResultQuestion {
    pub quesiton_id: String,
    pub total_num: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SurveyResultAnswerType {
    Select(Vec<String>),
    Text(String),
    NotResponded,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SurveyResultAnswer {
    pub responded_at: i64,
    pub quota_id: QuotaId, // 속성 ID
    pub responses: Vec<SurveyResultAnswerType>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdminSurveyCompleteRequest {
    pub ended_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AdminSurveyCompleteResponse {
    pub total: u32,
    pub succeed: u32,
    pub failed: u32,
}
