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
    deliberation_sample_surveys::deliberation_sample_survey::{
        DeliberationSampleSurveyGetResponse, DeliberationSampleSurveyParam,
        DeliberationSampleSurveyQuery, DeliberationSampleSurveySummary,
    },
    *,
};
use sqlx::postgres::PgRow;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct DeliberationSampleSurveyParentPath {
    pub deliberation_id: i64,
}

#[derive(Clone, Debug)]
pub struct DeliberationSampleSurveyController {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl DeliberationSampleSurveyController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            .route("/", get(Self::get_surveys))
            .with_state(self.clone())
    }

    pub async fn get_surveys(
        State(ctrl): State<DeliberationSampleSurveyController>,
        Path(DeliberationSampleSurveyParentPath { deliberation_id }): Path<
            DeliberationSampleSurveyParentPath,
        >,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<DeliberationSampleSurveyParam>,
    ) -> Result<Json<DeliberationSampleSurveyGetResponse>> {
        tracing::debug!("get_contents: {:?}", q);

        let res = match q {
            DeliberationSampleSurveyParam::Query(param) => {
                let res = match param {
                    param => ctrl.query(deliberation_id, auth, param).await?,
                };
                DeliberationSampleSurveyGetResponse::Query(res)
            }
        };

        Ok(Json(res))
    }
}

impl DeliberationSampleSurveyController {
    async fn query(
        &self,
        deliberation_id: i64,
        _auth: Option<Authorization>,
        param: DeliberationSampleSurveyQuery,
    ) -> Result<QueryResponse<DeliberationSampleSurveySummary>> {
        let mut total_count = 0;
        let items: Vec<DeliberationSampleSurveySummary> =
            DeliberationSampleSurveySummary::query_builder()
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
