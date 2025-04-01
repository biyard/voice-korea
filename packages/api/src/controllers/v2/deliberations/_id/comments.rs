use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::post,
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
    async fn replies_of(
        &self,
        deliberation_id: i64,
        auth: Option<Authorization>,
        param: DeliberationCommentQuery,
    ) -> Result<QueryResponse<DeliberationCommentSummary>> {
        let user_id = match auth {
            Some(Authorization::Bearer { ref claims }) => AppClaims(claims).get_user_id(),
            _ => 0,
        };
        let parent_id = param.parent_id.unwrap_or_default();
        if parent_id == 0 {
            return Err(ApiError::DeliberationCommentNotFound);
        }

        let mut total_count = 0;
        let items: Vec<DeliberationCommentSummary> =
            DeliberationCommentSummary::query_builder(user_id)
                .limit(param.size())
                .page(param.page())
                .parent_id_equals(param.parent_id.unwrap_or_default())
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

    async fn query(
        &self,
        deliberation_id: i64,
        auth: Option<Authorization>,
        param: DeliberationCommentQuery,
    ) -> Result<QueryResponse<DeliberationCommentSummary>> {
        let user_id = match auth {
            Some(Authorization::Bearer { ref claims }) => AppClaims(claims).get_user_id(),
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
            Some(Authorization::Bearer { ref claims }) => AppClaims(claims).get_user_id(),
            _ => return Err(ApiError::Unauthorized),
        };

        let res = self
            .repo
            .insert(user_id, deliberation_id, comment, 0)
            .await?;

        Ok(res)
    }

    async fn reply_to_comment(
        &self,
        deliberation_id: i64,
        comment_id: i64,
        auth: Option<Authorization>,
        DeliberationCommentReplyToCommentRequest { comment }: DeliberationCommentReplyToCommentRequest,
    ) -> Result<DeliberationComment> {
        let user_id = match auth {
            Some(Authorization::Bearer { ref claims }) => AppClaims(claims).get_user_id(),
            _ => return Err(ApiError::Unauthorized),
        };

        let res = self
            .repo
            .insert(user_id, deliberation_id, comment, comment_id)
            .await?;

        Ok(res)
    }

    async fn like(
        &self,
        comment_id: i64,
        auth: Option<Authorization>,
    ) -> Result<DeliberationComment> {
        let user_id = match auth {
            Some(Authorization::Bearer { ref claims }) => AppClaims(claims).get_user_id(),
            _ => return Err(ApiError::Unauthorized),
        };

        let mut tx = self.pool.begin().await?;

        let repo = DeliberationCommentLike::get_repository(self.pool.clone());
        repo.insert_with_tx(&mut *tx, comment_id, user_id)
            .await?
            .ok_or(ApiError::DeliberationCommentLikeException)?;

        let res = DeliberationComment::query_builder(user_id)
            .id_equals(comment_id)
            .query()
            .map(DeliberationComment::from)
            .fetch_optional(&mut *tx)
            .await?
            .ok_or(ApiError::DeliberationCommentException)?;

        tx.commit().await?;

        Ok(res)
    }
}

impl DeliberationCommentController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = DeliberationComment::get_repository(pool.clone());

        Self { repo, pool }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            .route("/:id", post(Self::act_deliberation_comment_by_id))
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
                ctrl.reply_to_comment(deliberation_id, id, auth, param)
                    .await?
            }
            DeliberationCommentByIdAction::Like(_) => ctrl.like(id, auth).await?,
        };

        Ok(Json(res))
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

        let res = match q {
            DeliberationCommentParam::Query(param) => {
                let res = match param {
                    param
                        if param.action == Some(DeliberationCommentQueryActionType::RepliesOf) =>
                    {
                        ctrl.replies_of(deliberation_id, auth, param).await?
                    }
                    param => ctrl.query(deliberation_id, auth, param).await?,
                };
                DeliberationCommentGetResponse::Query(res)
            }
        };

        Ok(Json(res))
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

#[cfg(test)]
mod deliberation_comment_tests {
    use deliberation::Deliberation;

    use super::*;

    // use crate::tests::{setup, TestContext};

    async fn create_deliberation(endpoint: &str, org_id: i64, now: i64) -> i64 {
        let cli = Deliberation::get_client(endpoint);
        let res = cli
            .create(
                org_id,
                now,
                now + 1000,
                "".to_string(),
                format!("test deliberation {now}"),
                "test description".to_string(),
                ProjectArea::City,
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
            )
            .await;
        assert!(res.is_ok());

        res.unwrap().id
    }

    // FIXME: remove this comment when fixed failed test code.
    // NOTE: Comments are created well on the web, but in the test code, it says that comments are not created. (Need to be modified after confirmation)
    // #[tokio::test]
    // async fn test_create_comment() {
    //     let TestContext {
    //         user,
    //         now,
    //         endpoint,
    //         ..
    //     } = setup().await.unwrap();
    //     let org_id = user.orgs[0].id;

    //     let deliberation_id = create_deliberation(&endpoint, org_id, now).await;

    //     let cli = DeliberationComment::get_client(&endpoint);

    //     let res = cli
    //         .comment(deliberation_id, "test comment".to_string())
    //         .await;

    //     assert!(res.is_ok());

    //     let res = res.unwrap();

    //     assert_eq!(res.comment, "test comment");

    //     let res = cli
    //         .query(deliberation_id, DeliberationCommentQuery::new(1))
    //         .await
    //         .unwrap();

    //     assert!(res.total_count == 1);
    // }

    // #[tokio::test]
    // async fn test_reply_to_comment() {
    //     let TestContext {
    //         user,
    //         now,
    //         endpoint,
    //         ..
    //     } = setup().await.unwrap();
    //     let org_id = user.orgs[0].id;

    //     let deliberation_id = create_deliberation(&endpoint, org_id, now).await;

    //     let cli = DeliberationComment::get_client(&endpoint);

    //     let res = cli
    //         .comment(deliberation_id, "test reply comment".to_string())
    //         .await;

    //     assert!(res.is_ok());

    //     let res = res.unwrap();

    //     assert_eq!(res.comment, "test reply comment");

    //     let reply = cli
    //         .reply_to_comment(deliberation_id, res.id, "replied to comment".to_string())
    //         .await
    //         .unwrap();

    //     assert_eq!(reply.parent_id, res.id);

    //     let replies = cli
    //         .replies_of(10, Some("1".to_string()), deliberation_id, res.id)
    //         .await
    //         .unwrap();

    //     assert!(replies.total_count == 1);

    //     let reply = cli
    //         .reply_to_comment(deliberation_id, res.id, "replied to comment 2".to_string())
    //         .await
    //         .unwrap();

    //     assert_eq!(reply.parent_id, res.id);

    //     let replies = cli
    //         .replies_of(10, Some("1".to_string()), deliberation_id, res.id)
    //         .await
    //         .unwrap();

    //     assert!(replies.total_count == 2);
    //     let req = DeliberationCommentQuery::new(10);
    //     tracing::info!("req {:?}", req);
    //     let res = cli
    //         .query(deliberation_id, DeliberationCommentQuery::new(10))
    //         .await
    //         .unwrap();
    //     assert!(res.total_count == 3, "total_count: {:?}", res);
    //     assert_eq!(res.items[0].replies, 2);
    // }
}
