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
    deliberation_final_surveys::deliberation_final_survey::{
        DeliberationFinalSurveyGetResponse, DeliberationFinalSurveyParam,
        DeliberationFinalSurveyQuery, DeliberationFinalSurveySummary,
    },
    *,
};
use sqlx::postgres::PgRow;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct DeliberationFinalSurveyParentPath {
    pub deliberation_id: i64,
}

#[derive(Clone, Debug)]
pub struct DeliberationFinalSurveyController {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl DeliberationFinalSurveyController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            .route("/", get(Self::get_surveys))
            .with_state(self.clone())
    }

    pub async fn get_surveys(
        State(ctrl): State<DeliberationFinalSurveyController>,
        Path(DeliberationFinalSurveyParentPath { deliberation_id }): Path<
            DeliberationFinalSurveyParentPath,
        >,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<DeliberationFinalSurveyParam>,
    ) -> Result<Json<DeliberationFinalSurveyGetResponse>> {
        tracing::debug!("get_contents: {:?}", q);

        let res = match q {
            DeliberationFinalSurveyParam::Query(param) => {
                let res = match param {
                    param => ctrl.query(deliberation_id, auth, param).await?,
                };
                DeliberationFinalSurveyGetResponse::Query(res)
            }
        };

        Ok(Json(res))
    }
}

impl DeliberationFinalSurveyController {
    async fn query(
        &self,
        deliberation_id: i64,
        _auth: Option<Authorization>,
        param: DeliberationFinalSurveyQuery,
    ) -> Result<QueryResponse<DeliberationFinalSurveySummary>> {
        let mut total_count = 0;
        let items: Vec<DeliberationFinalSurveySummary> =
            DeliberationFinalSurveySummary::query_builder()
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
