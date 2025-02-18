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
    Resource,
    ResourceAction,
    ResourceByIdAction,
    ResourceCreateRequest,
    ResourceGetResponse,
    ResourceParam,
    ResourceReadAction,
    ResourceRepository,
    ResourceUpdateRequest,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct ResourceControllerV1 {
    repo: ResourceRepository,
}

impl ResourceControllerV1 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> models::Result<Router> {
        let repo = Resource::get_repository(pool.clone());
        let ctrl = Self { repo };

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
        Path(_org_id): Path<i64>,
        Query(params): Query<ResourceParam>,
    ) -> models::Result<Json<ResourceGetResponse>> {
        match params {
            ResourceParam::Query(q) => {
                Ok(Json(ResourceGetResponse::Query(ctrl.repo.find(&q).await?)))
            }
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
        }
    }

    async fn act_resource_by_id(
        State(ctrl): State<ResourceControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path((_org_id, _id)): Path<(i64, i64)>,
        Json(body): Json<ResourceByIdAction>,
    ) -> models::Result<Json<Resource>> {
        // TODO:Check Permission
        match body {
            ResourceByIdAction::Update(req) => {
                let res = ctrl.update(req).await?;
                Ok(Json(res))
            } // ResourceByIdAction::Delete(reqwest) => {
              //     let res = Self::delete(ctrl.repo.clone(), reqwest.id).await?;
              //     Ok(Json(res))
              // }
        }
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
    async fn update(&self, req: ResourceUpdateRequest) -> models::Result<Resource> {
        tracing::debug!("update_resource: {:?}", req);
        // TODO: Update Resource
        Ok(Resource::default())
    }
    #[allow(unused)]
    async fn delete(&self, id: String) -> models::Result<Resource> {
        tracing::debug!("delete_resource: {:?}", id);
        Ok(Resource::default())
    }
}
