#![allow(unused)]

use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Extension, Json, Router,
    },
};

pub mod bucket;

use models::{
    // ResourceDeleteRequest,
    ApiError,
    GetObjectUriRequest,
    GetObjectUriResponse,
    QueryResponse,
    ResourceFile,
    ResourceFileAction,
    ResourceFileByIdAction,
    ResourceFileCreateRequest,
    ResourceFileGetResponse,
    ResourceFileParam,
    ResourceFileQuery,
    ResourceFileQueryActionType,
    ResourceFileReadAction,
    ResourceFileRepository,
    ResourceFileRepositoryUpdateRequest,
    ResourceFileSummary,
    ResourceFileUpdateRequest,
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;

#[derive(Clone, Debug)]
pub struct ResourceControllerV1 {
    repo: ResourceFileRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl ResourceControllerV1 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> models::Result<Router> {
        let repo = ResourceFile::get_repository(pool.clone());
        let ctrl = Self { repo, pool };

        Ok(Router::new()
            .route("/", get(Self::list_resources).post(Self::act_resource))
            .route(
                "/:id",
                get(Self::get_resource).post(Self::act_resource_by_id),
            )
            .with_state(ctrl))
    }

    async fn get_resource(
        State(ctrl): State<ResourceControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path((_org_id, id)): Path<(i64, i64)>,
    ) -> models::Result<Json<ResourceFile>> {
        let resource = ctrl
            .repo
            .find_one(&ResourceFileReadAction::new().find_by_id(id))
            .await?;
        Ok(Json(resource))
    }

    async fn list_resources(
        State(ctrl): State<ResourceControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(org_id): Path<i64>,
        Query(params): Query<ResourceFileParam>,
    ) -> models::Result<Json<ResourceFileGetResponse>> {
        match params {
            ResourceFileParam::Query(q) => match q.action {
                Some(ResourceFileQueryActionType::SearchBy) => {
                    Ok(ctrl.search_by(org_id, q).await?)
                }
                None => Ok(Json(ResourceFileGetResponse::Query(
                    ctrl.repo.find(&q).await?,
                ))),
            },
            ResourceFileParam::Read(q) => Ok(Json(ResourceFileGetResponse::Read(
                ctrl.repo.find_one(&q).await?,
            ))),
        }
    }

    async fn act_resource(
        State(ctrl): State<ResourceControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(org_id): Path<i64>,
        Json(body): Json<ResourceFileAction>,
    ) -> models::Result<Json<ResourceFile>> {
        match body {
            ResourceFileAction::Create(req) => {
                let res = ctrl.create(org_id, req).await?;
                Ok(Json(res))
            }
            ResourceFileAction::Delete(req) => {
                let res = ctrl.delete(req.id).await?;
                Ok(Json(res))
            }
        }
    }

    async fn act_resource_by_id(
        State(ctrl): State<ResourceControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path((org_id, id)): Path<(i64, i64)>,
        Json(body): Json<ResourceFileByIdAction>,
    ) -> models::Result<Json<ResourceFile>> {
        match body {
            ResourceFileByIdAction::Update(req) => {
                let res = ctrl.update(org_id, id, req).await?;
                Ok(Json(res))
            }
        }
    }
}

impl ResourceControllerV1 {
    pub async fn search_by(
        &self,
        org_id: i64,
        ResourceFileQuery {
            size,
            bookmark,
            title,
            ..
        }: ResourceFileQuery,
    ) -> models::Result<Json<ResourceFileGetResponse>> {
        let mut total_count: i64 = 0;

        let query = ResourceFileSummary::base_sql_with(
            "where org_id = $1 and title ilike $2 limit $3 offset $4",
        );

        let items: Vec<ResourceFileSummary> = sqlx::query(&query)
            .bind(org_id)
            .bind(format!("%{}%", title.unwrap()))
            .bind(size as i64)
            .bind(size as i64 * (bookmark.unwrap_or("1".to_string()).parse::<i64>().unwrap() - 1))
            .map(|r: PgRow| {
                use sqlx::Row;

                total_count = r.get("total_count");
                r.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(Json(ResourceFileGetResponse::Query(QueryResponse {
            items,
            total_count,
        })))
    }
}

impl ResourceControllerV1 {
    async fn create(
        &self,
        org_id: i64,
        req: ResourceFileCreateRequest,
    ) -> models::Result<ResourceFile> {
        tracing::debug!("create_resource: {:?}", req);
        let resource = self
            .repo
            .insert(
                req.title,
                req.resource_type,
                req.project_area,
                req.usage_purpose,
                req.source,
                req.access_level,
                org_id,
                req.files,
            )
            .await?;
        Ok(resource)
    }
    async fn update(
        &self,
        org_id: i64,
        id: i64,
        req: ResourceFileUpdateRequest,
    ) -> models::Result<ResourceFile> {
        tracing::debug!("update_resource: {:?}", req);

        let resource = self
            .repo
            .update(
                id,
                ResourceFileRepositoryUpdateRequest {
                    title: Some(req.title),
                    resource_type: req.resource_type,
                    project_area: req.project_area,
                    usage_purpose: req.usage_purpose,
                    source: req.source,
                    access_level: req.access_level,
                    org_id: Some(org_id),
                    files: Some(req.files),
                },
            )
            .await?;
        Ok(resource)
    }
    #[allow(unused)]
    async fn delete(&self, id: i64) -> models::Result<ResourceFile> {
        tracing::debug!("delete_resource: {:?}", id);

        let _ = self.repo.delete(id).await?;

        Ok(ResourceFile::default())
    }
}
