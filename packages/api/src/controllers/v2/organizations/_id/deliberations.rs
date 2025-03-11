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
        DeliberationParam, DeliberationQuery, DeliberationRepository, DeliberationSummary,
    },
    step::{Step, StepRepository},
    *,
};
use step::StepCreateRequest;

use crate::controllers::v2::organizations::OrganizationPath;

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
    step: StepRepository,
}

impl DeliberationController {
    pub async fn create(
        &self,
        org_id: i64,
        DeliberationCreateRequest {
            started_at,
            ended_at,
            project_area,
            title,
            description,
            panels,
            resource_ids,
            survey_ids,
            roles,
            steps,
            elearning,
            discussions,
        }: DeliberationCreateRequest,
    ) -> Result<Deliberation> {
        // TODO(api): incompleted creating API
        if started_at >= ended_at {
            return Err(ApiError::ValidationError(
                "started_at should be less than ended_at".to_string(),
            )
            .into());
        }

        let mut tx = self.pool.begin().await?;

        let deliberation = self
            .repo
            .insert_with_tx(
                &mut *tx,
                org_id,
                started_at,
                ended_at,
                project_area,
                title,
                description,
            )
            .await?
            .ok_or(ApiError::AlreadyExists)?;

        for StepCreateRequest {
            ended_at,
            step_type,
            started_at,
            name,
        } in steps
        {
            self.step
                .insert_with_tx(
                    &mut *tx,
                    deliberation.id,
                    step_type,
                    name,
                    started_at,
                    ended_at,
                )
                .await?
                .ok_or(ApiError::AlreadyExists)?;
        }

        tx.commit().await?;

        //TODO: add roles

        Ok(deliberation)
    }

    pub async fn query(
        &self,
        org_id: i64,
        DeliberationQuery { size, bookmark }: DeliberationQuery,
    ) -> Result<QueryResponse<DeliberationSummary>> {
        let mut total_count: i64 = 0;
        let items: Vec<DeliberationSummary> = Deliberation::query_builder()
            .org_id_equals(org_id)
            .limit(size as i32)
            .page(bookmark.unwrap_or("1".to_string()).parse::<i32>().unwrap())
            .with_count()
            .query()
            .map(|r: sqlx::postgres::PgRow| {
                use sqlx::Row;
                total_count = r.get("total_count");
                r.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(QueryResponse { total_count, items })
    }
}

impl DeliberationController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Deliberation::get_repository(pool.clone());
        let step = Step::get_repository(pool.clone());
        Self { pool, repo, step }
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
        State(ctrl): State<DeliberationController>,
        Path(OrganizationPath { org_id }): Path<OrganizationPath>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<DeliberationAction>,
    ) -> Result<Json<Deliberation>> {
        tracing::debug!("act_deliberation {} {:?}", org_id, body);

        match body {
            DeliberationAction::Create(param) => Ok(Json(ctrl.create(org_id, param).await?)),
        }
    }

    pub async fn get_deliberation_by_id(
        State(ctrl): State<DeliberationController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(DeliberationPath { org_id, id }): Path<DeliberationPath>,
    ) -> Result<Json<Deliberation>> {
        tracing::debug!("get_deliberation {} {:?}", org_id, id);
        // FIXME: {"DatabaseQueryError": "error returned from database: relation \"f\" does not exist"
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

    // pub async fn get_deliberation_by_id(
    //     State(ctrl): State<DeliberationController>,
    //     Extension(_auth): Extension<Option<Authorization>>,
    //     Path(DeliberationPath { org_id, id }): Path<DeliberationPath>,
    // ) -> Result<Json<Deliberation>> {
    //     tracing::debug!("get_deliberation {} {:?}", org_id, id);
    //     Ok(Json(
    //         Deliberation::query_builder()
    //             .id_equals(id)
    //             .org_id_equals(org_id)
    //             .query()
    //             .map(Deliberation::from)
    //             .fetch_one(&ctrl.pool)
    //             .await?,
    //     ))
    // }

    pub async fn get_deliberation(
        State(ctrl): State<DeliberationController>,
        Path(OrganizationPath { org_id }): Path<OrganizationPath>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(param): Query<DeliberationParam>,
    ) -> Result<Json<DeliberationGetResponse>> {
        tracing::debug!("list_deliberation {} {:?}", org_id, param);

        match param {
            // "DatabaseQueryError": "error returned from database: relation \"f\" does not exist"
            DeliberationParam::Query(q) => Ok(Json(DeliberationGetResponse::Query(
                ctrl.query(org_id, q).await?,
            ))),
        }
    }
}
