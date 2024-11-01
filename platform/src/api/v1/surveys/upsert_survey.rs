#![allow(unused_imports)]
use dioxus::prelude::{
    server_fn::codec::{GetUrl, Json, PostUrl},
    *,
};
use dioxus_logger::tracing;
use serde::{Deserialize, Serialize};

use crate::{
    api::common::TypeField,
    models::{
        question::QuestionType,
        survey::{QuestionSequence, Quota, SurveySequenceModel, SurveyStatus, SurveySummary},
    },
    service::login_service::use_login_service,
};

#[server(endpoint = "/v1/surveys", input = Json, output = Json)]
pub async fn upsert_survey(survey_id: String, item: SurveyUpdateItem) -> Result<(), ServerFnError> {
    dioxus_logger::tracing::debug!("/v1/surveys: {:?} {:?}", survey_id, item);

    Ok(())
}

#[server(endpoint = "/v1/empty/surveys", input = Json, output = Json)]
pub async fn create_empty_survey(email: String) -> Result<String, ServerFnError> {
    let log = crate::utils::logger::new_api("POST", &format!("/v1/empty/surveys"));
    let cli = crate::utils::db::get(&log);

    let mut sequence: i64 = match cli.get::<SurveySequenceModel>("survey_sequence").await {
        Ok(Some(v)) => v.sequence,
        _ => 0 as i64,
    };

    let id = format!("survey-{:?}", sequence).clone();

    match cli
        .create(SurveySummary {
            id: id.clone(),
            title: "".to_string(),
            status: SurveyStatus::Draft,
            updated_at: chrono::Utc::now().timestamp_millis() as u64,
            created_at: chrono::Utc::now().timestamp_millis() as u64,
            responses: None,
            expected_responses: None,
            questions: 0,
            quotas: None,
            r#type: "survey".to_string(),
            gsi1: email,
            gsi2: QuestionSequence::Title.to_string(),
        })
        .await
    {
        Ok(()) => {
            tracing::debug!("survey create success",);
        }
        Err(e) => {
            return Err(ServerFnError::ServerError(format!("DB create failed {e}")));
        }
    }

    sequence += 1;

    match cli
        .update::<TypeField>(
            "survey_sequence",
            vec![("sequence", TypeField::N(sequence))],
        )
        .await
    {
        Ok(()) => {
            tracing::debug!("DB sequence update success",);
        }
        Err(_e) => {
            match cli
                .create(SurveySequenceModel {
                    id: "survey_sequence".to_string(),
                    sequence: 1,
                })
                .await
            {
                Ok(()) => {
                    tracing::debug!("DB sequence create success",);
                }
                Err(e) => {
                    return Err(ServerFnError::ServerError(format!(
                        "DB sequence create failed {e}"
                    )));
                }
            }
        }
    };

    Ok(id.clone())
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum SurveyUpdateItem {
    Title(String),
    AddQuestion {
        title: String,
        question: QuestionType,
    },
    UpdateQuestion {
        id: String,
        title: Option<String>,
        question: Option<QuestionType>,
    },
    // RemoveRequestion requires question id to be removed.
    RemoveQuestion(String),

    AddResponder(Quota),
    // RemoveResponder requires quota id to be removed.
    RemoveResponder(String),

    // SetPeriod requires epoch timestamp in seconds for start and end time.
    SetPeriod(u64, u64),
}
