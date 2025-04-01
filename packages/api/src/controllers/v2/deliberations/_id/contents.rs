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
    deliberation_contents::deliberation_content::{
        DeliberationContentGetResponse, DeliberationContentParam, DeliberationContentQuery,
        DeliberationContentSummary,
    },
    *,
};
use sqlx::postgres::PgRow;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct DeliberationContentParentPath {
    pub deliberation_id: i64,
}

#[derive(Clone, Debug)]
pub struct DeliberationContentController {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl DeliberationContentController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            .route("/", get(Self::get_contents))
            .with_state(self.clone())
    }

    pub async fn get_contents(
        State(ctrl): State<DeliberationContentController>,
        Path(DeliberationContentParentPath { deliberation_id }): Path<
            DeliberationContentParentPath,
        >,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<DeliberationContentParam>,
    ) -> Result<Json<DeliberationContentGetResponse>> {
        tracing::debug!("get_contents: {:?}", q);

        let res = match q {
            DeliberationContentParam::Query(param) => {
                let res = match param {
                    param => ctrl.query(deliberation_id, auth, param).await?,
                };
                DeliberationContentGetResponse::Query(res)
            }
        };

        Ok(Json(res))
    }
}

impl DeliberationContentController {
    async fn query(
        &self,
        deliberation_id: i64,
        _auth: Option<Authorization>,
        param: DeliberationContentQuery,
    ) -> Result<QueryResponse<DeliberationContentSummary>> {
        let mut total_count = 0;
        let items: Vec<DeliberationContentSummary> = DeliberationContentSummary::query_builder()
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
