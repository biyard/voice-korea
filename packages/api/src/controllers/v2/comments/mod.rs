use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Query, State},
        routing::get,
        Extension, Json,
    },
};
use models::{
    comment::{Comment, CommentGetResponse, CommentParam, CommentQuery, CommentSummary},
    *,
};
use sqlx::postgres::PgRow;

#[derive(Clone, Debug)]
pub struct CommentController {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl CommentController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            .route("/", get(Self::get_comments))
            .with_state(self.clone())
    }

    pub async fn get_comments(
        State(ctrl): State<CommentController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(q): Query<CommentParam>,
    ) -> Result<Json<CommentGetResponse>> {
        tracing::debug!("get_comments: {:?}", q);

        let res = match q {
            CommentParam::Query(param) => {
                let res = match param {
                    param => ctrl.query(param).await?,
                };
                CommentGetResponse::Query(res)
            }
        };

        Ok(Json(res))
    }

    async fn query(&self, param: CommentQuery) -> Result<QueryResponse<CommentSummary>> {
        let mut total_count = 0;
        let items: Vec<CommentSummary> = Comment::query_builder()
            .limit(param.size())
            .page(param.page())
            .order_by_created_at_desc()
            .with_count()
            .query()
            .map(|row: PgRow| {
                use sqlx::Row;

                total_count = row.get("total_count");
                row.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(QueryResponse { total_count, items })
    }
}
