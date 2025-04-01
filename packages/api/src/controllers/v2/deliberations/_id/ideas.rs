use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::get,
        Extension, Json,
    },
};
use by_types::QueryResponse;
use models::{
    deliberation_discussions::deliberation_discussion::{
        DeliberationDiscussionGetResponse, DeliberationDiscussionParam,
        DeliberationDiscussionQuery, DeliberationDiscussionSummary,
    },
    *,
};
use sqlx::postgres::PgRow;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct DeliberationIdeaParentPath {
    pub deliberation_id: i64,
}

#[derive(Clone, Debug)]
pub struct DeliberationIdeaController {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl DeliberationIdeaController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            .route("/", get(Self::get_discussions))
            .with_state(self.clone())
    }

    pub async fn get_discussions(
        State(ctrl): State<DeliberationIdeaController>,
        Path(DeliberationIdeaParentPath { deliberation_id }): Path<DeliberationIdeaParentPath>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<DeliberationDiscussionParam>,
    ) -> Result<Json<DeliberationDiscussionGetResponse>> {
        tracing::debug!("get_discussions: {:?}", q);

        let res = match q {
            DeliberationDiscussionParam::Query(param) => {
                let res = match param {
                    param => ctrl.query(deliberation_id, auth, param).await?,
                };
                DeliberationDiscussionGetResponse::Query(res)
            }
        };

        Ok(Json(res))
    }
}

impl DeliberationIdeaController {
    async fn query(
        &self,
        deliberation_id: i64,
        _auth: Option<Authorization>,
        param: DeliberationDiscussionQuery,
    ) -> Result<QueryResponse<DeliberationDiscussionSummary>> {
        let mut total_count = 0;
        let items: Vec<DeliberationDiscussionSummary> =
            DeliberationDiscussionSummary::query_builder()
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
}
