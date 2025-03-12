#![allow(unused)]
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
use models::{
    deliberation::{
        Deliberation, DeliberationAction, DeliberationCreateRequest, DeliberationGetResponse,
        DeliberationParam, DeliberationQuery, DeliberationRepository,
    },
    *,
};

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct DeliberationPath {
    pub org_id: i64,
    pub id: i64,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct DeliberationParentPath {
    pub org_id: i64,
}

#[derive(Clone, Debug)]
pub struct DeliberationController {
    repo: DeliberationRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl DeliberationController {
    pub async fn create(
        &self,
        DeliberationCreateRequest {
            started_at,
            ended_at,
            steps,
            project_area,
            title,
            description,
            panels,
            resource_ids,
            survey_ids,
            roles,
        }: DeliberationCreateRequest,
    ) -> Result<Deliberation> {
        // TODO: implement temporary and create
        todo!()
    }

    pub async fn query(
        &self,
        org_id: i64,
        DeliberationQuery { size, bookmark }: DeliberationQuery,
    ) -> Result<QueryResponse<Deliberation>> {
        // TODO: impelement query
        todo!()
    }
}

impl DeliberationController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Deliberation::get_repository(pool.clone());

        Self { repo, pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/:id/contents", get(Self::get_deliberation_content))
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

    pub async fn get_deliberation_content(
        State(ctrl): State<DeliberationController>,
        Path(DeliberationPath { org_id, id }): Path<DeliberationPath>,
        Extension(_auth): Extension<Option<Authorization>>,
    ) -> Result<Json<Deliberation>> {
        tracing::debug!("get_deliberation_content {} {:?}", org_id, id);
        Ok(Json(
            Deliberation::query_builder()
                .id_equals(id)
                .org_id_equals(org_id)
                .query()
                .map(Deliberation::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
    }

    pub async fn act_deliberation(
        State(_ctrl): State<DeliberationController>,
        Path(DeliberationParentPath { org_id }): Path<DeliberationParentPath>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<DeliberationAction>,
    ) -> Result<Json<Deliberation>> {
        tracing::debug!("act_deliberation {} {:?}", org_id, body);

        match body {
            DeliberationAction::Create(param) => Ok(Json(_ctrl.create(param).await?)),
        }
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
        State(ctrl): State<DeliberationController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(DeliberationPath { org_id, id }): Path<DeliberationPath>,
    ) -> Result<Json<Deliberation>> {
        tracing::debug!("get_deliberation {} {:?}", org_id, id);
        Ok(Json(
            Deliberation::query_builder()
                .id_equals(id)
                .org_id_equals(org_id)
                .query()
                .map(Deliberation::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
    }

    pub async fn get_deliberation(
        State(_ctrl): State<DeliberationController>,
        Path(DeliberationParentPath { org_id }): Path<DeliberationParentPath>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(q): Query<DeliberationParam>,
    ) -> Result<Json<DeliberationGetResponse>> {
        tracing::debug!("list_deliberation {} {:?}", org_id, q);

        Ok(Json(DeliberationGetResponse::Query(
            QueryResponse::default(),
        )))
    }
}
