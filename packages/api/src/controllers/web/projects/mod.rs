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
use deliberation_project::*;
use models::*;
use sqlx::postgres::PgRow;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct DeliberationProjectPath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct DeliberationProjectController {
    repo: DeliberationProjectRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

// TODO: implement APIs
impl DeliberationProjectController {
    async fn query(
        &self,
        _auth: Option<Authorization>,
        param: DeliberationProjectQuery,
    ) -> Result<QueryResponse<DeliberationProjectSummary>> {
        let mut total_count = 0;
        let items: Vec<DeliberationProjectSummary> = DeliberationProjectSummary::query_builder()
            .limit(param.size())
            .page(param.page())
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

impl DeliberationProjectController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = DeliberationProject::get_repository(pool.clone());

        Self { repo, pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/:id", get(Self::get_deliberation_project_by_id))
            .route("/", get(Self::get_deliberation_project))
            .with_state(self.clone()))
    }

    pub async fn get_deliberation_project_by_id(
        State(ctrl): State<DeliberationProjectController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(DeliberationProjectPath { id }): Path<DeliberationProjectPath>,
    ) -> Result<Json<DeliberationProject>> {
        tracing::debug!("get_deliberation_project {:?}", id);

        Ok(Json(
            DeliberationProject::query_builder()
                .id_equals(id)
                .query()
                .map(DeliberationProject::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
    }

    pub async fn get_deliberation_project(
        State(ctrl): State<DeliberationProjectController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<DeliberationProjectParam>,
    ) -> Result<Json<DeliberationProjectGetResponse>> {
        tracing::debug!("list_deliberation_project {:?}", q);

        match q {
            DeliberationProjectParam::Query(param) => Ok(Json(
                DeliberationProjectGetResponse::Query(ctrl.query(auth, param).await?),
            )),
            // DeliberationProjectParam::Read(param)
            //     if param.action == Some(DeliberationProjectReadActionType::ActionType) =>
            // {
            //     let res = ctrl.run_read_action(auth, param).await?;
            //     Ok(Json(DeliberationProjectGetResponse::Read(res)))
            // }
        }
    }
}
