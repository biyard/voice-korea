use std::str::FromStr;

use crate::{prelude::PanelInfo, projects::ProjectField};
use bdk::prelude::*;
#[cfg(feature = "server")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct CreatePublicSurveyRequest {
    pub introductions: Option<PublicSurveyIntroduction>,
    pub questions: Option<Vec<Question>>,
    pub members: Option<PublicSurveyMemberInfo>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct UpdatePublicSurveyRequest {
    pub introductions: Option<PublicSurveyIntroduction>,
    pub questions: Option<Vec<Question>>,
    pub members: Option<PublicSurveyMemberInfo>,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum SurveyActionRequest {
    Create(CreatePublicSurveyRequest),
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub enum SurveyByIdActionRequest {
    Delete,
    Update(UpdatePublicSurveyRequest),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct PublicSurveySummary {
    pub id: String,
    pub survey_type: SurveyType,
    pub survey_field_type: ProjectField,
    pub title: String,
    pub total_response: u64,
    pub survey_response: u64,
    pub panels: Vec<PanelInfo>,
    pub start_date: i64,
    pub end_date: i64,
    pub status: PublicSurveyStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct PublicSurveyResponse {
    pub id: String,
    pub statistics: PublicSurveyStatistics,
    pub response_participant_rate_totals: PublicSurveyResponseParticipantRateTotals,
    pub response_participant_rates: Vec<PublicSurveyResponseParticipantRates>,
    pub single_choice_statistics: SingleChoiceStatistics,
    pub multiple_choice_statistics: MultipleChoiceStatistics,
    pub text_statistics: TextStatistics,
    pub optional_statistics: OptionalStatistics,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct OptionalStatistics {
    pub totals: OptionalInfo,
    pub panels: Vec<OptionalPanelInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct OptionalPanelInfo {
    pub panel_id: String,
    pub panel_name: String,
    pub statistics: OptionalInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct OptionalInfo {
    pub responses: Vec<i64>,
    pub response_rates: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct TextStatistics {
    pub totals: TextInfo,
    pub panels: Vec<TextPanelInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct TextPanelInfo {
    pub panel_id: String,
    pub panel_name: String,
    pub statistics: TextInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct TextInfo {
    pub most_used_keyword: Vec<String>,
    pub include_keyword_answer: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct MultipleChoiceStatistics {
    pub totals: Vec<MultipleChoiceInfo>,
    pub panels: Vec<MultipleChoicePanelInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct MultipleChoicePanelInfo {
    pub panel_id: String,
    pub panel_name: String,
    pub statistics: Vec<MultipleChoiceInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct MultipleChoiceInfo {
    pub answer_name: String,
    pub response_count: u64,
    pub response_rate: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct SingleChoiceStatistics {
    pub totals: Vec<SingleChoiceInfo>,
    pub panels: Vec<SingleChoicePanelInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct SingleChoicePanelInfo {
    pub panel_id: String,
    pub panel_name: String,
    pub statistics: Vec<SingleChoiceInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct SingleChoiceInfo {
    pub answer_name: String,
    pub response_count: u64,
    pub response_rate: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct PublicSurveyResponseParticipantRateTotals {
    pub panels: Vec<PublicSurveyResponsePanelInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct PublicSurveyResponseParticipantRates {
    pub question_id: String,
    pub question_name: String,
    pub panels: Vec<PublicSurveyResponsePanelInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct PublicSurveyResponsePanelInfo {
    pub id: String,
    pub name: String,
    pub members: i64,
    pub percents: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct PublicSurveyStatistics {
    pub total_members: i64,
    pub response_members: i64,
    pub participants_rate: i64,
    pub time_taken: String,    //etc. 00:02:00
    pub remained_time: String, //etc. 20일
    pub start_date: i64,
    pub end_date: i64,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct PublicSurveyMemberInfo {
    pub total_members: u64,
    pub members: Vec<PublicSurveyMember>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct PublicSurveyMember {
    pub member_id: String,
    pub panel_ids: Vec<String>,
    pub attribute_ids: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct PublicSurveyIntroduction {
    pub field: ProjectField,
    pub title: String,
    pub start_date: u64,
    pub end_date: u64,
    pub description: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
pub struct PublicQuestion {
    #[serde(rename = "id")]
    pub id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    #[serde(rename = "answer_type")]
    pub question_type: PublicSurveyQuestionType,
    pub image_url: Option<String>,

    pub answer_start_range: Option<i64>, //1~10 까지 그렇다~아니다일 경우 다음 필드 활용
    pub answer_end_range: Option<i64>,
    pub options: Option<Vec<String>>, //체크박스, 드롭다운인 경우 선택지 입력, 평가척도의 경우 1, 10이 어느 구간에 해당하는지 입력

    pub multiple_choice_enable: Option<bool>, //복수 응답 유무
    pub necessary_answer_enable: Option<bool>, //필수 입력 유무
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PublicSurveyQuestionType {
    #[default]
    Subjective, //주관식 답변
    Dropdown, //드롭다운
    Checkbox, //체크박스
    Optional, //평가 척도
}

impl PublicSurveyQuestionType {
    pub fn translate(&self, lang: &Language) -> &'static str {
        match lang {
            Language::En => match self {
                PublicSurveyQuestionType::Dropdown => "Dropdown",
                PublicSurveyQuestionType::Checkbox => "Checkbox",
                PublicSurveyQuestionType::Subjective => "Subjective",
                PublicSurveyQuestionType::Optional => "Rating",
            },
            Language::Ko => match self {
                PublicSurveyQuestionType::Dropdown => "드랍다운 선택",
                PublicSurveyQuestionType::Checkbox => "체크박스 선택",
                PublicSurveyQuestionType::Subjective => "주관식 답변",
                PublicSurveyQuestionType::Optional => "평가 척도",
            },
        }
    }
}

impl FromStr for PublicSurveyQuestionType {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Dropdown" | "드랍다운 선택" => Ok(PublicSurveyQuestionType::Dropdown),
            "Checkbox" | "체크박스 선택" => Ok(PublicSurveyQuestionType::Checkbox),
            "Subjective" | "주관식 답변" => Ok(PublicSurveyQuestionType::Subjective),
            "Rating" | "평가 척도" => Ok(PublicSurveyQuestionType::Optional),
            _ => Err(format!("invalid field")),
        }
    }

    type Err = String;
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
#[serde(rename_all = "snake_case")]
pub enum PublicSurveyStatus {
    #[default]
    Ready,
    InProgress,
    Finish,
}

impl PublicSurveyStatus {
    pub fn translate(&self, lang: &Language) -> &'static str {
        match lang {
            Language::En => match self {
                PublicSurveyStatus::Ready => "Ready",
                PublicSurveyStatus::InProgress => "InProgress",
                PublicSurveyStatus::Finish => "Finish",
            },
            Language::Ko => match self {
                PublicSurveyStatus::Ready => "준비",
                PublicSurveyStatus::InProgress => "진행",
                PublicSurveyStatus::Finish => "마감",
            },
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(JsonSchema, aide::OperationIo))]
#[serde(rename_all = "snake_case")]
pub enum SurveyType {
    #[default]
    Survey,
    PublicPoll,
    Satisfaction,
}

impl SurveyType {
    pub fn translate(&self, lang: &Language) -> &'static str {
        match lang {
            Language::En => match self {
                SurveyType::Survey => "Survey",
                SurveyType::PublicPoll => "Opinion Poll",
                SurveyType::Satisfaction => "Satisfaction Survey",
            },
            Language::Ko => match self {
                SurveyType::Survey => "설문 조사",
                SurveyType::PublicPoll => "여론 조사",
                SurveyType::Satisfaction => "만족도 조사",
            },
        }
    }
}
