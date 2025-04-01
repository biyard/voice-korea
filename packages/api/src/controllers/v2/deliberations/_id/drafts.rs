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
    deliberation_drafts::deliberation_draft::{
        DeliberationDraftGetResponse, DeliberationDraftParam, DeliberationDraftQuery,
        DeliberationDraftSummary,
    },
    *,
};
use sqlx::postgres::PgRow;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct DeliberationDraftParentPath {
    pub deliberation_id: i64,
}

#[derive(Clone, Debug)]
pub struct DeliberationDraftController {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl DeliberationDraftController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            .route("/", get(Self::get_drafts))
            .with_state(self.clone())
    }

    pub async fn get_drafts(
        State(ctrl): State<DeliberationDraftController>,
        Path(DeliberationDraftParentPath { deliberation_id }): Path<DeliberationDraftParentPath>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<DeliberationDraftParam>,
    ) -> Result<Json<DeliberationDraftGetResponse>> {
        tracing::debug!("get_contents: {:?}", q);

        let res = match q {
            DeliberationDraftParam::Query(param) => {
                let res = match param {
                    param => ctrl.query(deliberation_id, auth, param).await?,
                };
                DeliberationDraftGetResponse::Query(res)
            }
        };

        Ok(Json(res))
    }
}

impl DeliberationDraftController {
    async fn query(
        &self,
        deliberation_id: i64,
        _auth: Option<Authorization>,
        param: DeliberationDraftQuery,
    ) -> Result<QueryResponse<DeliberationDraftSummary>> {
        let mut total_count = 0;
        let items: Vec<DeliberationDraftSummary> = DeliberationDraftSummary::query_builder()
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
