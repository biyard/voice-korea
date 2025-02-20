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
    Resource,
    ResourceAction,
    ResourceByIdAction,
    ResourceCreateRequest,
    ResourceGetResponse,
    ResourceParam,
    ResourceQuery,
    ResourceQueryActionType,
    ResourceReadAction,
    ResourceRepository,
    ResourceRepositoryUpdateRequest,
    ResourceSummary,
    ResourceUpdateRequest,
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;

#[derive(Clone, Debug)]
pub struct ResourceControllerV1 {
    repo: ResourceRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl ResourceControllerV1 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> models::Result<Router> {
        let repo = Resource::get_repository(pool.clone());
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
    ) -> models::Result<Json<Resource>> {
        let resource = ctrl
            .repo
            .find_one(&ResourceReadAction::new().find_by_id(id))
            .await?;
        Ok(Json(resource))
    }

    async fn list_resources(
        State(ctrl): State<ResourceControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(org_id): Path<i64>,
        Query(params): Query<ResourceParam>,
    ) -> models::Result<Json<ResourceGetResponse>> {
        match params {
            ResourceParam::Query(q) => match q.action {
                Some(ResourceQueryActionType::SearchBy) => Ok(ctrl.search_by(org_id, q).await?),
                None => Ok(Json(ResourceGetResponse::Query(ctrl.repo.find(&q).await?))),
            },
            ResourceParam::Read(q) => Ok(Json(ResourceGetResponse::Read(
                ctrl.repo.find_one(&q).await?,
            ))),
        }
    }

    async fn act_resource(
        State(ctrl): State<ResourceControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(org_id): Path<i64>,
        Json(body): Json<ResourceAction>,
    ) -> models::Result<Json<Resource>> {
        match body {
            ResourceAction::Create(req) => {
                let res = ctrl.create(org_id, req).await?;
                Ok(Json(res))
            }
            ResourceAction::Delete(req) => {
                let res = ctrl.delete(req.id).await?;
                Ok(Json(res))
            }
        }
    }

    async fn act_resource_by_id(
        State(ctrl): State<ResourceControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path((org_id, id)): Path<(i64, i64)>,
        Json(body): Json<ResourceByIdAction>,
    ) -> models::Result<Json<Resource>> {
        match body {
            ResourceByIdAction::Update(req) => {
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
        ResourceQuery {
            size,
            bookmark,
            title,
            ..
        }: ResourceQuery,
    ) -> models::Result<Json<ResourceGetResponse>> {
        let mut total_count: i64 = 0;

        let query = ResourceSummary::base_sql_with(
            "where org_id = $1 and title ilike $2 limit $3 offset $4",
        );

        let items: Vec<ResourceSummary> = sqlx::query(&query)
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

        Ok(Json(ResourceGetResponse::Query(QueryResponse {
            items,
            total_count,
        })))
    }
}

impl ResourceControllerV1 {
    async fn create(&self, org_id: i64, req: ResourceCreateRequest) -> models::Result<Resource> {
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
        req: ResourceUpdateRequest,
    ) -> models::Result<Resource> {
        tracing::debug!("update_resource: {:?}", req);

        let resource = self
            .repo
            .update(
                id,
                ResourceRepositoryUpdateRequest {
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
    async fn delete(&self, id: i64) -> models::Result<Resource> {
        tracing::debug!("delete_resource: {:?}", id);

        let _ = self.repo.delete(id).await?;

        Ok(Resource::default())
    }
}
