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
use models::*;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct DeliberationPath {
    pub parent_id: i64,
    pub id: i64,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct DeliberationParentPath {
    pub parent_id: i64,
}

#[derive(Clone, Debug)]
pub struct DeliberationController {
    repo: DeliberationRepository,
}

impl DeliberationController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Deliberation::get_repository(pool);

        Self { repo }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route(
                "/:id",
                get(Self::get_deliberation_by_id), // .post(Self::act_deliberation_by_id)
            )
            .with_state(self.clone())
            .route(
                "/",
                post(Self::act_deliberation).get(Self::get_deliberation),
            )
            .with_state(self.clone()))
    }

    pub async fn act_deliberation(
        State(_ctrl): State<DeliberationController>,
        Path(DeliberationParentPath { parent_id }): Path<DeliberationParentPath>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<DeliberationAction>,
    ) -> Result<Json<Deliberation>> {
        tracing::debug!("act_deliberation {} {:?}", parent_id, body);
        Ok(Json(Deliberation::default()))
    }

    // pub async fn act_deliberation_by_id(
    //     State(_ctrl): State<DeliberationController>,
    //     Extension(_auth): Extension<Option<Authorization>>,
    //     Path(DeliberationPath { parent_id, id }): Path<DeliberationPath>,
    //     Json(body): Json<DeliberationByIdAction>,
    // ) -> Result<Json<Deliberation>> {
    //     tracing::debug!("act_deliberation_by_id {} {:?} {:?}", parent_id, id, body);
    //     Ok(Json(Deliberation::default()))
    // }

    pub async fn get_deliberation_by_id(
        State(_ctrl): State<DeliberationController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(DeliberationPath { parent_id, id }): Path<DeliberationPath>,
    ) -> Result<Json<Deliberation>> {
        tracing::debug!("get_deliberation {} {:?}", parent_id, id);
        Ok(Json(Deliberation::default()))
    }

    pub async fn get_deliberation(
        State(_ctrl): State<DeliberationController>,
        Path(DeliberationParentPath { parent_id }): Path<DeliberationParentPath>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(q): Query<DeliberationParam>,
    ) -> Result<Json<DeliberationGetResponse>> {
        tracing::debug!("list_deliberation {} {:?}", parent_id, q);

        Ok(Json(DeliberationGetResponse::Query(
            QueryResponse::default(),
        )))
    }
}
