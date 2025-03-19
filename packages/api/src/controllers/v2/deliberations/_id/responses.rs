use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Extension, Json,
    },
};
use by_types::QueryResponse;
use models::{
    deliberation::{Deliberation, DeliberationRepository},
    deliberation_response::{
        DeliberationResponse, DeliberationResponseAction, DeliberationResponseByIdAction,
        DeliberationResponseGetResponse, DeliberationResponseParam, DeliberationResponseRepository,
        DeliberationResponseRepositoryUpdateRequest, DeliberationResponseRespondAnswerRequest,
        DeliberationResponseUpdateRespondAnswerRequest, DeliberationType,
    },
    *,
};

use crate::utils::app_claims::AppClaims;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DeliberationResponseController {
    repo: DeliberationResponseRepository,
    deliberation: DeliberationRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl DeliberationResponseController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = DeliberationResponse::get_repository(pool.clone());
        let deliberation = Deliberation::get_repository(pool.clone());

        Self {
            repo,
            pool,
            deliberation,
        }
    }

    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let ctrl = Self::new(pool);

        Ok(by_axum::axum::Router::new()
            .route(
                "/:id",
                get(Self::get_deliberation_response).post(Self::act_deliberation_response_by_id),
            )
            .with_state(ctrl.clone())
            .route(
                "/",
                post(Self::act_deliberation_response).get(Self::list_deliberation_response),
            )
            .with_state(ctrl.clone()))
    }

    pub async fn act_deliberation_response_by_id(
        State(ctrl): State<DeliberationResponseController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(DeliberationResponsePath {
            deliberation_id,
            id,
        }): Path<DeliberationResponsePath>,
        Json(body): Json<DeliberationResponseByIdAction>,
    ) -> Result<Json<DeliberationResponse>> {
        tracing::debug!(
            "act_deliberation_response_by_id {} {:?} {:?}",
            deliberation_id,
            id,
            body
        );

        let res = match body {
            DeliberationResponseByIdAction::UpdateRespondAnswer(params) => {
                ctrl.update_respond_answer(id, auth, params).await?
            }
            DeliberationResponseByIdAction::RemoveRespondAnswer(_) => {
                ctrl.remove_respond_answer(id, auth).await?
            }
        };

        Ok(res)
    }

    pub async fn get_deliberation_response(
        State(_ctrl): State<DeliberationResponseController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(DeliberationResponsePath {
            deliberation_id,
            id,
        }): Path<DeliberationResponsePath>,
    ) -> Result<Json<DeliberationResponse>> {
        //TODO: implement get_deliberation_response
        tracing::debug!("get_deliberation_response {} {}", deliberation_id, id);
        Ok(Json(DeliberationResponse::default()))
    }

    pub async fn list_deliberation_response(
        State(_ctrl): State<DeliberationResponseController>,
        Path(DeliberationResponseParentPath { deliberation_id }): Path<
            DeliberationResponseParentPath,
        >,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(q): Query<DeliberationResponseParam>,
    ) -> Result<Json<DeliberationResponseGetResponse>> {
        //TODO(api): implement list_deliberation_response
        tracing::debug!("list_deliberation_response {} {:?}", deliberation_id, q);

        match q {
            DeliberationResponseParam::Query(_q) => Ok(Json(
                DeliberationResponseGetResponse::Query(QueryResponse {
                    total_count: 0,
                    items: vec![],
                }),
            )),
        }
    }

    pub async fn act_deliberation_response(
        State(ctrl): State<DeliberationResponseController>,
        Path(DeliberationResponseParentPath { deliberation_id }): Path<
            DeliberationResponseParentPath,
        >,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<DeliberationResponseAction>,
    ) -> Result<Json<DeliberationResponse>> {
        //TODO(api): implement act_deliberation_response
        tracing::debug!("act_deliberation_response {} {:?}", deliberation_id, body);

        match body {
            DeliberationResponseAction::RespondAnswer(req) => {
                ctrl.respond_answer(deliberation_id, auth, req).await
            }
        }
    }
}

impl DeliberationResponseController {
    pub async fn remove_respond_answer(
        &self,
        response_id: i64,
        auth: Option<Authorization>,
    ) -> Result<Json<DeliberationResponse>> {
        let _ = match auth {
            Some(Authorization::Bearer { ref claims }) => AppClaims(claims).get_user_id(),
            _ => return Err(ApiError::Unauthorized),
        };

        let respond = DeliberationResponse::query_builder()
            .id_equals(response_id)
            .query()
            .map(DeliberationResponse::from)
            .fetch_one(&self.pool)
            .await?;

        if respond.deliberation_type == DeliberationType::Survey {
            return Err(ApiError::UpdateNotAllowed);
        }

        let res = self.repo.delete(response_id).await?;

        Ok(Json(res))
    }

    pub async fn update_respond_answer(
        &self,
        response_id: i64,
        auth: Option<Authorization>,
        DeliberationResponseUpdateRespondAnswerRequest {
            answers,
        }: DeliberationResponseUpdateRespondAnswerRequest,
    ) -> Result<Json<DeliberationResponse>> {
        let _ = match auth {
            Some(Authorization::Bearer { ref claims }) => AppClaims(claims).get_user_id(),
            _ => return Err(ApiError::Unauthorized),
        };

        let respond = DeliberationResponse::query_builder()
            .id_equals(response_id)
            .query()
            .map(DeliberationResponse::from)
            .fetch_one(&self.pool)
            .await?;

        if respond.deliberation_type == DeliberationType::Survey {
            return Err(ApiError::UpdateNotAllowed);
        }

        let res = self
            .repo
            .update(
                response_id,
                DeliberationResponseRepositoryUpdateRequest {
                    deliberation_id: None,
                    user_id: None,
                    answers: Some(answers),
                    deliberation_type: None,
                },
            )
            .await?;

        Ok(Json(res))
    }

    pub async fn respond_answer(
        &self,
        deliberation_id: i64,
        auth: Option<Authorization>,
        DeliberationResponseRespondAnswerRequest {
            answers,
            deliberation_type,
        }: DeliberationResponseRespondAnswerRequest,
    ) -> Result<Json<DeliberationResponse>> {
        let user_id: i64 = match auth {
            Some(Authorization::Bearer { ref claims }) => AppClaims(claims).get_user_id(),
            _ => return Err(ApiError::Unauthorized),
        };

        let res = self
            .repo
            .insert(deliberation_id, user_id, answers, deliberation_type)
            .await?;
        Ok(Json(res))
    }
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct DeliberationResponsePath {
    pub deliberation_id: i64,
    pub id: i64,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct DeliberationResponseParentPath {
    pub deliberation_id: i64,
}
