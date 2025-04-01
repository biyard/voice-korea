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
    deliberation_basic_infos::deliberation_basic_info::{
        DeliberationBasicInfoGetResponse, DeliberationBasicInfoParam, DeliberationBasicInfoQuery,
        DeliberationBasicInfoSummary,
    },
    *,
};
use sqlx::postgres::PgRow;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct DeliberationInfoParentPath {
    pub deliberation_id: i64,
}

#[derive(Clone, Debug)]
pub struct DeliberationBasicInfoController {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl DeliberationBasicInfoController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            .route("/", get(Self::get_basic_infos))
            .with_state(self.clone())
    }

    pub async fn get_basic_infos(
        State(ctrl): State<DeliberationBasicInfoController>,
        Path(DeliberationInfoParentPath { deliberation_id }): Path<DeliberationInfoParentPath>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<DeliberationBasicInfoParam>,
    ) -> Result<Json<DeliberationBasicInfoGetResponse>> {
        tracing::debug!("get_contents: {:?}", q);

        let res = match q {
            DeliberationBasicInfoParam::Query(param) => {
                let res = match param {
                    param => ctrl.query(deliberation_id, auth, param).await?,
                };
                DeliberationBasicInfoGetResponse::Query(res)
            }
        };

        Ok(Json(res))
    }
}

impl DeliberationBasicInfoController {
    async fn query(
        &self,
        deliberation_id: i64,
        _auth: Option<Authorization>,
        param: DeliberationBasicInfoQuery,
    ) -> Result<QueryResponse<DeliberationBasicInfoSummary>> {
        let mut total_count = 0;
        let items: Vec<DeliberationBasicInfoSummary> =
            DeliberationBasicInfoSummary::query_builder()
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
