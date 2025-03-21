#![allow(unused_variables, unused)]
use crate::{
    PanelCountSurveys, PanelCountsV2, PanelV2, ProjectArea, ProjectStatus, ProjectType, Result,
};
use bdk::prelude::*;
use by_types::QueryResponse;
use chrono::{TimeZone, Utc};
use validator::ValidationError;

use super::response::{Answer, SurveyResponse};

// If you want to know how to use Y macro, refer to https://github.com/biyard/rust-sdk/tree/main/packages/by-macros
#[api_model(base = "/v2/organizations/:org-id/surveys", table = surveys, action_by_id = [start_survey, update(panel_ids = Vec<i64>)], iter_type=QueryResponse)]
pub struct SurveyV2 {
    #[api_model(summary, primary_key, action = delete, read_action = find_by_id)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, action = create, action_by_id = update)]
    pub name: String,

    #[api_model(summary, type = INTEGER, action_by_id = update)]
    pub project_type: ProjectType,

    #[api_model(summary, action = create, type = INTEGER, action_by_id = update)]
    pub project_area: ProjectArea,

    #[api_model(summary, type = INTEGER, action_by_id = update)]
    pub status: ProjectStatus,

    #[api_model(summary, action = create, action_by_id = update)]
    pub started_at: i64,

    #[api_model(summary, action = create, action_by_id = update)]
    pub ended_at: i64,

    #[api_model(summary, action = create, action_by_id = update)]
    pub description: String,
    #[api_model(summary, action = create, action_by_id = update)]
    pub quotes: i64,

    #[api_model(summary, many_to_one = organizations)]
    pub org_id: i64,
    #[api_model(summary, action = create, type = JSONB, version = v0.1, action_by_id = update)]
    pub questions: Vec<Question>,

    //FIXME: add action_by_id tag
    #[api_model(summary, action = create, many_to_many = panel_surveys, foreign_table_name = panels, foreign_primary_key = panel_id, foreign_reference_key = survey_id,)]
    #[serde(default)]
    pub panels: Vec<PanelV2>,

    // FIXME: This data may be one_to_many of panel_surveys table
    #[api_model(summary, action = create, type = JSONB, version = v0.1, action_by_id = update)]
    pub panel_counts: Vec<PanelCountsV2>,
    #[api_model(summary)]
    pub noncelab_id: Option<i64>,
    #[api_model(summary, one_to_many = survey_responses, foreign_key = survey_id, aggregator = count)]
    #[serde(default)]
    pub response_count: i64,
    // #[api_model(summary, many_to_many = attrs, foreign_table_name = attributes, foreign_primary_key = attr_id, foreign_reference_key = survey_id)]
    // pub attributes: Vec<Attribute>,
}

impl SurveyV2 {
    pub fn period(&self, lang: Language) -> String {
        let started_at = self.formatted_timestamp(lang, self.started_at);
        let ended_at = self.formatted_timestamp(lang, self.ended_at);

        format!("{} ~ {}", started_at, ended_at)
    }

    pub fn formatted_timestamp(&self, lang: Language, timestamp: i64) -> String {
        let datetime = Utc
            .timestamp_opt(timestamp, 0)
            .single()
            .expect("Invalid timestamp");

        match lang {
            Language::Ko => datetime.format("%-m월 %-d일 %Y년").to_string(),
            Language::En => datetime.format("%-m. %-d. %Y").to_string(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "snake_case", tag = "answer_type")]
pub enum Question {
    SingleChoice(ChoiceQuestion),
    MultipleChoice(ChoiceQuestion),
    ShortAnswer(SubjectiveQuestion),
    Subjective(SubjectiveQuestion),
}

impl Default for Question {
    fn default() -> Self {
        Question::ShortAnswer(SubjectiveQuestion::default())
    }
}

impl Question {
    pub fn new(answer_type: &str) -> Self {
        match answer_type {
            "Single Choice" | "객관식(단일선택)" => {
                Question::SingleChoice(ChoiceQuestion::default())
            }

            "Multiple Choice" | "객관식(다중선택)" => {
                Question::MultipleChoice(ChoiceQuestion::default())
            }

            "Short Answer" | "주관식(단답형)" => {
                Question::ShortAnswer(SubjectiveQuestion::default())
            }

            "Subjective" | "주관식(서술형)" => {
                Question::Subjective(SubjectiveQuestion::default())
            }
            _ => {
                panic!("Invalid answer type: {}", answer_type);
            }
        }
    }

    pub fn set_title(&mut self, title: &str) {
        match self {
            Question::SingleChoice(q) => {
                q.title = title.to_string();
            }
            Question::MultipleChoice(q) => {
                q.title = title.to_string();
            }
            Question::ShortAnswer(q) => {
                q.title = title.to_string();
            }
            Question::Subjective(q) => {
                q.title = title.to_string();
            }
        }
    }

    pub fn title(&self) -> String {
        match self {
            Question::SingleChoice(q) => q.title.clone(),
            Question::MultipleChoice(q) => q.title.clone(),
            Question::ShortAnswer(q) => q.title.clone(),
            Question::Subjective(q) => q.title.clone(),
        }
    }

    pub fn description(&self) -> String {
        match self {
            Question::SingleChoice(q) => q.description.clone().unwrap_or_default(),
            Question::MultipleChoice(q) => q.description.clone().unwrap_or_default(),
            Question::ShortAnswer(q) => q.description.clone(),
            Question::Subjective(q) => q.description.clone(),
        }
    }

    pub fn set_description(&mut self, description: &str) {
        match self {
            Question::SingleChoice(q) => {
                q.description = Some(description.to_string());
            }
            Question::MultipleChoice(q) => {
                q.description = Some(description.to_string());
            }
            Question::ShortAnswer(q) => {
                q.description = description.to_string();
            }
            Question::Subjective(q) => {
                q.description = description.to_string();
            }
        }
    }

    pub fn remove_option(&mut self, index: usize) {
        match self {
            Question::SingleChoice(q) => {
                q.options.remove(index);
            }
            Question::MultipleChoice(q) => {
                q.options.remove(index);
            }
            _ => {
                panic!("Invalid question type for adding option: {:?}", self);
            }
        }
    }

    pub fn add_option(&mut self, option: &str) {
        match self {
            Question::SingleChoice(q) => {
                q.options.push(option.to_string());
            }
            Question::MultipleChoice(q) => {
                q.options.push(option.to_string());
            }
            _ => {
                panic!("Invalid question type for adding option: {:?}", self);
            }
        }
    }

    pub fn change_option(&mut self, index: usize, option: &str) {
        match self {
            Question::SingleChoice(q) => {
                let mut options = q.options.clone();
                options[index] = option.to_string();
                q.options = options;
            }
            Question::MultipleChoice(q) => {
                let mut options = q.options.clone();
                options[index] = option.to_string();
                q.options = options;
            }
            _ => {
                panic!("Invalid question type for adding option: {:?}", self);
            }
        }
    }

    pub fn options(&self) -> Vec<String> {
        match self {
            Question::SingleChoice(q) => q.options.clone(),
            Question::MultipleChoice(q) => q.options.clone(),
            _ => vec![],
        }
    }

    pub fn to_type(&self, lang: &Language) -> String {
        match (self, lang) {
            (&Question::SingleChoice(_), &Language::En) => "Single Choice".to_string(),
            (&Question::SingleChoice(_), &Language::Ko) => "객관식(단일선택)".to_string(),

            (&Question::MultipleChoice(_), &Language::En) => "Multiple Choice".to_string(),
            (&Question::MultipleChoice(_), &Language::Ko) => "객관식(다중선택)".to_string(),

            (&Question::ShortAnswer(_), &Language::En) => "Short Answer".to_string(),
            (&Question::ShortAnswer(_), &Language::Ko) => "주관식(단답형)".to_string(),

            (&Question::Subjective(_), &Language::En) => "Subjective".to_string(),
            (&Question::Subjective(_), &Language::Ko) => "주관식(서술형)".to_string(),
        }
    }

    pub fn types(lang: &Language) -> Vec<String> {
        match lang {
            Language::En => vec![
                "Single Choice".to_string(),
                "Multiple Choice".to_string(),
                "Short Answer".to_string(),
                "Subjective".to_string(),
            ],
            Language::Ko => vec![
                "객관식(단일선택)".to_string(),
                "객관식(다중선택)".to_string(),
                "주관식(단답형)".to_string(),
                "주관식(서술형)".to_string(),
            ],
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SubjectiveQuestion {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct ChoiceQuestion {
    pub title: String,
    pub description: Option<String>,
    pub options: Vec<String>,
}

impl SurveyV2Summary {
    pub fn start_date(&self) -> String {
        let datetime = Utc.timestamp_opt(self.started_at, 0).unwrap();
        let formatted_date = datetime.format("%Y.%m.%d").to_string();
        formatted_date
    }

    pub fn end_date(&self) -> String {
        let datetime = Utc.timestamp_opt(self.ended_at, 0).unwrap();
        let formatted_date = datetime.format("%Y.%m.%d").to_string();
        formatted_date
    }

    pub fn period(&self) -> String {
        format!("{} ~ {}", self.start_date(), self.end_date())
    }

    pub fn response_rate(&self) -> String {
        let responses = self.response_count;

        format!(
            "{}% ({}/{})",
            if self.quotes == 0 {
                0
            } else {
                responses / self.quotes * 100
            },
            responses,
            self.quotes
        )
    }
}
