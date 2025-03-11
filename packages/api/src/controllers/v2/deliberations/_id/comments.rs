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
use deliberation_comment::*;
use deliberation_comments_likes::DeliberationCommentLike;
use models::*;
use sqlx::postgres::PgRow;

use crate::utils::app_claims::AppClaims;

#[derive(Clone, Debug)]
pub struct DeliberationCommentController {
    repo: DeliberationCommentRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl DeliberationCommentController {
    async fn query(
        &self,
        deliberation_id: i64,
        auth: Option<Authorization>,
        param: DeliberationCommentQuery,
    ) -> Result<QueryResponse<DeliberationCommentSummary>> {
        let user_id = match auth {
            Some(Authorization::Bearer { claims }) => AppClaims(claims).get_user_id(),
            _ => 0,
        };

        let mut total_count = 0;
        let items: Vec<DeliberationCommentSummary> =
            DeliberationCommentSummary::query_builder(user_id)
                .limit(param.size())
                .page(param.page())
                .deliberation_id_equals(deliberation_id)
                .query()
                .map(|row: PgRow| {
                    use sqlx::Row;

                    total_count = row.try_get("total_count").unwrap_or_default();
                    row.into()
                })
                .fetch_all(&self.pool)
                .await?;

        Ok(QueryResponse { total_count, items })
    }

    async fn create(
        &self,
        deliberation_id: i64,
        auth: Option<Authorization>,
        DeliberationCommentCommentRequest { comment }: DeliberationCommentCommentRequest,
    ) -> Result<DeliberationComment> {
        let user_id = match auth {
            Some(Authorization::Bearer { claims }) => AppClaims(claims).get_user_id(),
            _ => return Err(ApiError::Unauthorized),
        };

        let res = self
            .repo
            .insert(user_id, deliberation_id, comment, 0)
            .await?;

        Ok(res)
    }

    async fn update(
        &self,
        id: i64,
        auth: Option<Authorization>,
        param: DeliberationCommentReplyToCommentRequest,
    ) -> Result<DeliberationComment> {
        if auth.is_none() {
            return Err(ApiError::Unauthorized);
        }

        let res = self.repo.update(id, param.into()).await?;

        Ok(res)
    }

    async fn like(&self, id: i64, auth: Option<Authorization>) -> Result<DeliberationComment> {
        if auth.is_none() {
            return Err(ApiError::Unauthorized);
        }

        let repo = DeliberationCommentLike::get_repository(self.pool.clone());

        Ok(res)
    }

    // async fn run_read_action(
    //     &self,
    //     _auth: Option<Authorization>,
    //     DeliberationCommentReadAction { action, .. }: DeliberationCommentReadAction,
    // ) -> Result<DeliberationComment> {
    //     todo!()
    // }
}

impl DeliberationCommentController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = DeliberationComment::get_repository(pool.clone());

        Self { repo, pool }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            .route(
                "/:id",
                get(Self::get_deliberation_comment_by_id)
                    .post(Self::act_deliberation_comment_by_id),
            )
            .with_state(self.clone())
            .route(
                "/",
                post(Self::act_deliberation_comment).get(Self::get_deliberation_comment),
            )
            .with_state(self.clone())
    }

    pub async fn act_deliberation_comment(
        State(ctrl): State<DeliberationCommentController>,
        Path(DeliberationCommentParentPath { deliberation_id }): Path<
            DeliberationCommentParentPath,
        >,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<DeliberationCommentAction>,
    ) -> Result<Json<DeliberationComment>> {
        tracing::debug!("act_deliberation_comment {} {:?}", deliberation_id, body);
        match body {
            DeliberationCommentAction::Comment(param) => {
                let res = ctrl.create(deliberation_id, auth, param).await?;
                Ok(Json(res))
            }
        }
    }

    pub async fn act_deliberation_comment_by_id(
        State(ctrl): State<DeliberationCommentController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(DeliberationCommentPath {
            deliberation_id,
            id,
        }): Path<DeliberationCommentPath>,
        Json(body): Json<DeliberationCommentByIdAction>,
    ) -> Result<Json<DeliberationComment>> {
        tracing::debug!(
            "act_deliberation_comment_by_id {} {:?} {:?}",
            deliberation_id,
            id,
            body
        );

        let res = match body {
            DeliberationCommentByIdAction::ReplyToComment(param) => {
                ctrl.update(id, auth, param).await?
            }
            DeliberationCommentByIdAction::Like(_) => ctrl.like(id, auth).await?,
        };

        Ok(Json(res))
    }

    pub async fn get_deliberation_comment_by_id(
        State(ctrl): State<DeliberationCommentController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(DeliberationCommentPath {
            deliberation_id,
            id,
        }): Path<DeliberationCommentPath>,
    ) -> Result<Json<DeliberationComment>> {
        tracing::debug!("get_deliberation_comment {} {:?}", deliberation_id, id);

        let user_id = match auth {
            Some(Authorization::Bearer { claims }) => AppClaims(claims).get_user_id(),
            _ => 0,
        };

        Ok(Json(
            DeliberationComment::query_builder(user_id)
                .id_equals(id)
                .deliberation_id_equals(deliberation_id)
                .query()
                .map(DeliberationComment::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
    }

    pub async fn get_deliberation_comment(
        State(ctrl): State<DeliberationCommentController>,
        Path(DeliberationCommentParentPath { deliberation_id }): Path<
            DeliberationCommentParentPath,
        >,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<DeliberationCommentParam>,
    ) -> Result<Json<DeliberationCommentGetResponse>> {
        tracing::debug!("list_deliberation_comment {} {:?}", deliberation_id, q);

        match q {
            DeliberationCommentParam::Query(param) => {
                Ok(Json(DeliberationCommentGetResponse::Query(
                    ctrl.query(deliberation_id, auth, param).await?,
                )))
            } // DeliberationCommentParam::Read(param)
              //     if param.action == Some(DeliberationCommentReadActionType::ActionType) =>
              // {
              //     let res = ctrl.run_read_action(auth, param).await?;
              //     Ok(Json(DeliberationCommentGetResponse::Read(res)))
              // }
        }
    }
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct DeliberationCommentPath {
    pub deliberation_id: i64,
    pub id: i64,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct DeliberationCommentParentPath {
    pub deliberation_id: i64,
}
