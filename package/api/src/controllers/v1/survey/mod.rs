use crate::{
    middleware::auth::authorization_middleware,
    utils::{error::ApiError, jwt::Claims, nonce_lab, time::convert_utc_timestamp_to_datetime},
};

use by_axum::{
    axum::{
        extract::{Path, Query, State},
        middleware,
        routing::get,
        Extension, Json, Router,
    },
    log::root,
};

use easy_dynamodb::Client;
use nonce_lab::models::{NonceLabCreateSurveyRequest, NonceLabQuota, NonceLabSurveyQuestion};
use nonce_lab::NonceLabClient;

use models::prelude::{
    ListSurveyResponse, ProgressSurveyResponse, Survey, SurveyDraftStatus, SurveyResultDocument,
    SurveyStatus, UpsertSurveyDraftRequest,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Clone)]
pub struct AxumState {
    db: Arc<Client>,
    nonce_lab_client: Arc<NonceLabClient>,
}
pub fn router(db: Arc<Client>) -> Router {
    let nonce_lab_client = Arc::new(NonceLabClient::new());

    Router::new()
        .route("/", get(list_survey).patch(upsert_survey_draft))
        .route("/:id", get(get_survey).post(progress_survey))
        .route("/:id/result", get(get_survey_result))
        .with_state(AxumState {
            db,
            nonce_lab_client,
        })
        .layer(middleware::from_fn(authorization_middleware))
}

const UPDATE_DELAY_MS: i64 = 300_000; // 300 seconds (5 minutes)

#[derive(Deserialize)]
pub struct GetParams {
    pub size: Option<i32>,
    pub bookmark: Option<String>,
}
async fn list_survey(
    Extension(claims): Extension<Claims>,
    State(state): State<AxumState>,
    Query(params): Query<GetParams>,
) -> Result<Json<ListSurveyResponse>, ApiError> {
    let result: Result<
        (Option<Vec<Survey>>, Option<String>),
        easy_dynamodb::error::DynamoException,
    > = state
        .db
        .find(
            "gsi1-index",
            params.bookmark,
            Some(params.size.unwrap_or(10)),
            vec![("gsi1", Survey::get_gsi1(&claims.id))],
        )
        .await;
    let (mut surveys, bookmark) = match result {
        Ok((Some(docs), bookmark)) => (docs, bookmark),
        _ => return Err(ApiError::NotFound),
    };
    let now = chrono::Utc::now().timestamp_millis();
    for survey in surveys.iter_mut() {
        if let Some(nonce_lab_id) = survey.nonce_lab_id {
            if survey.status == SurveyStatus::InProgress
                && survey.updated_at + UPDATE_DELAY_MS <= now
            {
                let current_responses = state
                    .nonce_lab_client
                    .get_survey_responses(nonce_lab_id)
                    .await?;
                let _ = state
                    .db
                    .update(
                        &survey.id,
                        vec![
                            ("updated_at", UpdateField::I64(now)),
                            ("response_count", UpdateField::OU64(current_responses)),
                        ],
                    )
                    .await;
                survey.response_count = current_responses;
            }
        }
    }

    Ok(Json(ListSurveyResponse {
        bookmark,
        survey: surveys,
    }))
}

async fn get_survey(
    Extension(claims): Extension<Claims>,
    State(state): State<AxumState>,
    Path(id): Path<String>,
) -> Result<Json<Survey>, ApiError> {
    let now = chrono::Utc::now().timestamp_millis();
    let result: Result<Option<Survey>, easy_dynamodb::error::DynamoException> =
        state.db.get(&id).await;
    let mut survey = match result {
        Ok(Some(v)) => {
            if v.creator != claims.id {
                return Err(ApiError::InvalidCredentials("Not Creator".to_string()));
            } else {
                v
            }
        }
        Ok(None) => return Err(ApiError::SurveyNotFound(id)),
        Err(e) => return Err(ApiError::DynamoQueryException(e.to_string())),
    };
    if survey.status == SurveyStatus::InProgress
        && survey.nonce_lab_id.is_some()
        && survey.updated_at + UPDATE_DELAY_MS <= now
    {
        let current_responses = state
            .nonce_lab_client
            .get_survey_responses(survey.nonce_lab_id.unwrap())
            .await?;
        let _ = state
            .db
            .update(
                &survey.id,
                vec![
                    ("updated_at", UpdateField::I64(now)),
                    ("response_count", UpdateField::OU64(current_responses)),
                ],
            )
            .await;
        survey.response_count = current_responses;
    }

    Ok(Json(survey))
}

async fn upsert_survey_draft(
    Extension(claims): Extension<Claims>,
    State(state): State<AxumState>,
    Json(req): Json<UpsertSurveyDraftRequest>,
) -> Result<Json<String>, ApiError> {
    slog::debug!(root(), "CLAIM: {}", claims.id.clone());
    let mut survey = if let Some(id) = req.id {
        let result: Result<Option<Survey>, easy_dynamodb::error::DynamoException> =
            state.db.get(&id).await;
        match result {
            Ok(Some(v)) => {
                if v.status != SurveyStatus::Draft {
                    return Err(ApiError::NotDraftSurvey);
                }
                if v.creator != claims.id {
                    return Err(ApiError::InvalidCredentials("Not Creator".to_string()));
                }
                v
            }
            Ok(None) => return Err(ApiError::SurveyNotFound(id)),
            Err(e) => return Err(ApiError::DynamoQueryException(e.to_string())),
        }
    } else {
        Survey::new(uuid::Uuid::new_v4().to_string(), claims.id)
    };
    let draft_id = survey.id.clone();
    if let Some(title) = req.title {
        survey.title = title;
    }
    if let Some(questions) = req.questions {
        survey.questions = questions;
    }
    if let Some(quotas) = req.quotas {
        survey.quotas = quotas;
    }
    if let Some(started_at) = req.started_at {
        survey.started_at = Some(started_at);
    }
    if let Some(ended_at) = req.ended_at {
        survey.ended_at = Some(ended_at);
    }
    if let Some(status) = req.status {
        survey.draft_status = Some(status);
    }

    survey.updated_at = chrono::Utc::now().timestamp_millis();
    state.db.upsert(survey).await.map_err(|e| {
        slog::error!(root(), "UPSERT ERROR: {:?}", e);
        ApiError::DynamoCreateException(e.to_string())
    })?;
    Ok(Json(draft_id))
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
enum UpdateField {
    #[serde(untagged)]
    U32(u32),
    #[serde(untagged)]
    Status(SurveyStatus),
    #[serde(untagged)]
    Null(Option<bool>),
    #[serde(untagged)]
    OU64(Option<u64>),
    #[serde(untagged)]
    I64(i64),
    #[serde(untagged)]
    S(String),
}

//Update Survey Status from Draft to in_progress
async fn progress_survey(
    Extension(claims): Extension<Claims>,
    State(state): State<AxumState>,
    Path(id): Path<String>,
) -> Result<Json<ProgressSurveyResponse>, ApiError> {
    let result: Result<Option<Survey>, easy_dynamodb::error::DynamoException> =
        state.db.get(&id).await;
    let survey = match result {
        Ok(Some(v)) => {
            if v.creator != claims.id {
                return Err(ApiError::InvalidCredentials("Not Owner".to_string()));
            }
            if v.status != SurveyStatus::Draft {
                return Err(ApiError::NotDraftSurvey);
            }
            if !matches!(v.draft_status, Some(SurveyDraftStatus::Complete))
                || v.title == String::default()
                || v.questions.is_empty()
                || v.quotas.is_empty()
                || v.started_at.is_none()
                || v.ended_at.is_none()
            {
                return Err(ApiError::InCompleteDraft);
            }
            v
        }
        Ok(None) => return Err(ApiError::SurveyNotFound(id)),
        Err(e) => return Err(ApiError::DynamoQueryException(e.to_string())),
    };
    let expected_responses: u64 = survey.quotas.iter().map(|v| v.quota).sum();
    let quotas: Vec<NonceLabQuota> = survey.quotas.into_iter().map(|v| v.into()).collect();
    let questions: Vec<NonceLabSurveyQuestion> =
        survey.questions.into_iter().map(|v| v.into()).collect();
    let survey_dto = NonceLabCreateSurveyRequest {
        custom_id: survey.id.clone(),
        status: SurveyStatus::InProgress,
        started_at: survey.started_at.unwrap() / 100,
        ended_at: survey.ended_at.unwrap() / 100,
        title: survey.title,
        quotas,
        questions,
        description: None,
        expected_responses,
    };
    let now = chrono::Utc::now().timestamp_millis();
    let end_date = convert_utc_timestamp_to_datetime(survey.ended_at.unwrap(), Some(1))
        .unwrap_or("INVALID_END_DATE".to_string());
    let nonce_lab_id = state.nonce_lab_client.create_survey(survey_dto).await?;
    match state
        .db
        .update(
            &survey.id,
            vec![
                ("gsi2", UpdateField::S(Survey::get_gsi2(end_date))),
                ("nonce_lab_id", UpdateField::U32(nonce_lab_id)),
                ("status", UpdateField::Status(SurveyStatus::InProgress)),
                ("draft_status", UpdateField::Null(None)),
                ("updated_at", UpdateField::I64(now)),
                ("response_count", UpdateField::OU64(Some(0))),
                (
                    "total_response_count",
                    UpdateField::OU64(Some(expected_responses)),
                ),
            ],
        )
        .await
    {
        Ok(_) => Ok(Json(ProgressSurveyResponse {
            id: uuid::Uuid::new_v4().to_string(),
            nonce_lab_id,
        })),
        Err(e) => Err(ApiError::DynamoUpdateException(e.to_string())),
    }
}

async fn get_survey_result(
    Extension(claims): Extension<Claims>,
    State(state): State<AxumState>,
    Path(id): Path<String>,
) -> Result<Json<SurveyResultDocument>, ApiError> {
    let query_result = state
        .db
        .find(
            "gsi1-index",
            None,
            Some(1),
            vec![("gsi1", SurveyResultDocument::get_gsi1(&claims.id, &id))],
        )
        .await;

    let docs = match query_result {
        Ok((Some(docs), _)) if !docs.is_empty() => docs,
        _ => return Err(ApiError::NotFound),
    };

    Ok(Json(docs.into_iter().next().unwrap()))
}
