use std::{collections::HashMap, sync::Arc};

use crate::utils::{
    error::ApiError, nonce_lab::NonceLabClient, time::convert_rfc3339_to_timestamp_millis,
};
use by_axum::{
    axum::{extract::State, routing::patch, Json, Router},
    log::root,
};
use easy_dynamodb::Client;
use futures::{stream::FuturesUnordered, StreamExt};
use models::prelude::{
    AdminSurveyCompleteRequest, AdminSurveyCompleteResponse, QuestionId, Quota, QuotaId, Survey,
    SurveyQuestion, SurveyResultAnswer, SurveyResultAnswerType, SurveyResultDocument, SurveyStatus,
};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct AxumState {
    db: Arc<Client>,
    nonce_lab_client: Arc<NonceLabClient>,
}
pub fn router(db: Arc<Client>) -> Router {
    let nonce_lab_client = Arc::new(NonceLabClient::new());
    Router::new()
        .route("/", patch(finalize_survey))
        .with_state(AxumState {
            db,
            nonce_lab_client,
        })
}

#[derive(Debug, Serialize, Deserialize)]
enum UpdateField {
    #[serde(untagged)]
    Status(SurveyStatus),
    #[serde(untagged)]
    I64(i64),
    #[serde(untagged)]
    S(String),
}

async fn create_survey_result(
    db: Arc<Client>,
    nonce_lab_client: Arc<NonceLabClient>,
    survey: Survey,
) -> Result<String, ApiError> {
    // Create Survey Result Docuemnt
    let nonce_lab_id = survey.nonce_lab_id.unwrap();
    let response_count_map = nonce_lab_client
        .get_survey(nonce_lab_id)
        .await?
        .response_count_map
        .unwrap_or_default();
    let survey_answers = nonce_lab_client.get_survey_result(nonce_lab_id).await?;
    let mut answers: Vec<SurveyResultAnswer> = vec![];
    let mut quotas: HashMap<QuotaId, Quota> = HashMap::new();
    let mut questions: HashMap<QuestionId, SurveyQuestion> = HashMap::new();

    let mut survey_responses_by_question: HashMap<QuestionId, HashMap<String, u64>> =
        HashMap::new();
    for (i, question) in survey.questions.into_iter().enumerate() {
        questions.insert(i as QuestionId, question);
    }
    for quota in survey_answers.quotas.into_iter() {
        let quota_id = quota.id.unwrap_or_default();
        quotas.insert(quota_id as QuotaId, quota.into());
    }
    for item in survey_answers.response_array.into_iter() {
        let quota_id = item.quota_id;
        let responded_at =
            convert_rfc3339_to_timestamp_millis(&item.responded_at).unwrap_or_default();
        let mut responses: Vec<SurveyResultAnswerType> = vec![];

        for (question_id, answer) in item.answers.into_iter().enumerate() {
            let answer_type = if let Some(v) = answer.text_answer {
                SurveyResultAnswerType::Text(v)
            } else if let Some(v) = answer.choice_answer {
                SurveyResultAnswerType::Select(v)
            } else {
                SurveyResultAnswerType::NotResponded
            };
            let text_list = match answer_type.clone() {
                SurveyResultAnswerType::Text(v) => vec![v],
                SurveyResultAnswerType::Select(v) => v,
                _ => vec![],
            };
            for text in text_list {
                survey_responses_by_question
                    .entry(question_id as QuestionId)
                    .or_insert_with(HashMap::new)
                    .entry(text)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
            responses.push(answer_type);
        }
        let result_answer: SurveyResultAnswer = SurveyResultAnswer {
            responded_at,
            quota_id,
            responses,
        };

        answers.push(result_answer);
    }

    let doc_id = uuid::Uuid::new_v4().to_string();
    let _ = db
        .create(SurveyResultDocument {
            gsi1: SurveyResultDocument::get_gsi1(&survey.creator, &survey.id),
            id: doc_id.clone(),
            user_id: survey.creator.clone(),
            survey_id: survey.id.clone(),
            r#type: SurveyResultDocument::get_type(),
            created_at: chrono::Utc::now().timestamp_millis(),
            expected_responses: survey.total_response_count.unwrap_or_default(),
            actual_responses: survey.response_count.unwrap_or_default(),
            answers,
            quotas,
            questions,
            survey_responses_by_quota: response_count_map,
            survey_responses_by_question,
        })
        .await;

    Ok(doc_id)
}

async fn finalize_survey(
    State(state): State<AxumState>,
    Json(req): Json<AdminSurveyCompleteRequest>,
) -> Result<Json<AdminSurveyCompleteResponse>, ApiError> {
    // Get survey from DB
    slog::debug!(root(), "finalize_survey Date : {}", req.ended_at);
    let mut bookmark: Option<String> = None;
    let mut surveys: Vec<Survey> = vec![];
    let gsi2 = Survey::get_gsi2(req.ended_at);
    loop {
        let result: Result<
            (Option<Vec<Survey>>, Option<String>),
            easy_dynamodb::error::DynamoException,
        > = state
            .db
            .find("gsi2-index", None, Some(10), vec![("gsi2", &gsi2)])
            .await;
        let next_bookmark = match result {
            Ok((Some(v), bookmark)) => {
                surveys.extend(
                    v.into_iter()
                        .filter(|v| v.status == SurveyStatus::InProgress),
                );
                bookmark
            }
            Ok((None, bookmark)) => bookmark,
            Err(e) => return Err(ApiError::DynamoQueryException(e.to_string())),
        };
        if bookmark.is_none() {
            break;
        }
        bookmark = next_bookmark;
    }
    let total_num = surveys.len() as u32;

    let mut futures = FuturesUnordered::new();

    for survey in surveys {
        let state = state.clone();
        futures.push(async move {
            let survey_id = survey.id.clone();
            let doc_id =
                create_survey_result(state.db.clone(), state.nonce_lab_client.clone(), survey)
                    .await
                    .map_err(|_| survey_id.clone())?;
            state
                .db
                .update(
                    &survey_id,
                    vec![
                        ("status", UpdateField::Status(SurveyStatus::Finished)),
                        (
                            "updated_at",
                            UpdateField::I64(chrono::Utc::now().timestamp_millis()),
                        ),
                        ("result_id", UpdateField::S(doc_id)),
                    ],
                )
                .await
                .map_err(|_| survey_id)?;

            Ok::<(), String>(())
        });
    }
    let mut succeed = 0;
    let mut failed = 0;

    while let Some(result) = futures.next().await {
        match result {
            Ok(()) => {
                succeed += 1;
            }
            Err(e) => {
                // 에러 처리
                slog::error!(root(), "Error ID: {}", e);
                failed += 1;
            }
        }
    }

    Ok(Json(AdminSurveyCompleteResponse {
        total: total_num,
        succeed,
        failed,
    }))
}
