use crate::{
    middleware::auth::authorization_middleware,
    utils::{error::ApiError, jwt::Claims, nonce_lab},
};
use by_axum::axum::{
    extract::{Path, Query, State},
    middleware,
    routing::get,
    Extension, Json, Router,
};
mod local_models;

use easy_dynamodb::Client;
use local_models::{
    NonceLabCreateSurveyRequest, NonceLabCreateSurveyResponse, NonceLabQuota,
    NonceLabSurveyQuestion,
};

use models::prelude::{
    ListSurveyResponse, ProgressSurveyResponse, Survey, SurveyDraftStatus, SurveyStatus,
    UpsertSurveyDraftRequest,
};
use serde::Deserialize;
use std::sync::Arc;

pub fn router(db: Arc<Client>) -> Router {
    Router::new()
        .route("/", get(list_survey).patch(upsert_survey_draft))
        .route("/:id", get(get_survey).post(progress_survey))
        .with_state(db)
        .layer(middleware::from_fn(authorization_middleware))
}

#[derive(Deserialize)]
pub struct GetParams {
    pub size: Option<i32>,
    pub bookmark: Option<String>,
}

async fn list_survey(
    Extension(claims): Extension<Claims>,
    State(db): State<Arc<Client>>,
    Query(params): Query<GetParams>,
) -> Result<Json<ListSurveyResponse>, ApiError> {
    let result: Result<
        (Option<Vec<Survey>>, Option<String>),
        easy_dynamodb::error::DynamoException,
    > = db
        .find(
            "gsi1-index",
            params.bookmark,
            Some(params.size.unwrap_or(10)),
            vec![("gsi1", Survey::get_gsi1(&claims.id))],
        )
        .await;
    let (survey, bookmark) = match result {
        Ok((Some(docs), bookmark)) => (docs, bookmark),
        _ => return Err(ApiError::NotFound),
    };
    Ok(Json(ListSurveyResponse { bookmark, survey }))
}

async fn get_survey(
    Extension(claims): Extension<Claims>,
    State(db): State<Arc<Client>>,
    Path(id): Path<String>,
) -> Result<Json<Survey>, ApiError> {
    let result: Result<Option<Survey>, easy_dynamodb::error::DynamoException> = db.get(&id).await;
    match result {
        Ok(Some(v)) => {
            if v.creator != claims.id {
                Err(ApiError::InvalidCredentials("Not Creator".to_string()))
            } else {
                Ok(Json(v))
            }
        }
        Ok(None) => Err(ApiError::SurveyNotFound(id)),
        Err(e) => Err(ApiError::DynamoQueryException(e.to_string())),
    }
}

async fn upsert_survey_draft(
    Extension(claims): Extension<Claims>,
    State(db): State<Arc<Client>>,
    Json(req): Json<UpsertSurveyDraftRequest>,
) -> Result<Json<String>, ApiError> {
    let mut survey = if let Some(id) = req.id {
        let result: Result<Option<Survey>, easy_dynamodb::error::DynamoException> =
            db.get(&id).await;
        match result {
            Ok(Some(v)) => {
                if v.status != SurveyStatus::Draft {
                    return Err(ApiError::SurveyInProgress);
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
    let _ = db.upsert(survey).await;
    Ok(Json(draft_id))
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
enum UpdateField {
    #[serde(untagged)]
    Id(u32),
    #[serde(untagged)]
    S(SurveyStatus),
    #[serde(untagged)]
    Null(Option<bool>),
}

//Update Survey Status from Draft to in_progress
async fn progress_survey(
    Extension(claims): Extension<Claims>,
    State(db): State<Arc<Client>>,
    Path(id): Path<String>,
) -> Result<Json<ProgressSurveyResponse>, ApiError> {
    let result: Result<Option<Survey>, easy_dynamodb::error::DynamoException> = db.get(&id).await;
    let survey = match result {
        Ok(Some(v)) => {
            if v.creator != claims.id {
                return Err(ApiError::InvalidCredentials("Not Ownder".to_string()));
            }
            if v.status != SurveyStatus::Draft {
                return Err(ApiError::SurveyInProgress);
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

    let nonce_lab_client = nonce_lab::NonceLabClient::new()
        .map_err(|e| ApiError::ReqwestClientFailed(e.to_string()))?;
    let res = nonce_lab_client
        .post("/v1/vendor/survey")
        .json(&survey_dto)
        .send()
        .await
        .map_err(|v| ApiError::ReqwestFailed(v.to_string()))?;
    let res: NonceLabCreateSurveyResponse = match res.error_for_status() {
        Ok(v) => match v.json().await {
            Ok(v) => v,
            Err(e) => return Err(ApiError::JSONSerdeError(e.to_string())),
        },
        Err(e) => {
            return Err(ApiError::ReqwestFailed(e.to_string()));
        }
    };
    let nonce_lab_id = res.id;
    match db
        .update(
            &survey.id,
            vec![
                ("nonce_lab_id", UpdateField::Id(nonce_lab_id)),
                ("status", UpdateField::S(SurveyStatus::InProgress)),
                ("draft_status", UpdateField::Null(None)),
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
