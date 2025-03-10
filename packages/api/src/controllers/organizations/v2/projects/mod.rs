#![allow(dead_code, unused)]
use by_axum::axum::{
    extract::{Path, Query, State},
    routing::post,
    Json,
};
use by_axum::{
    auth::Authorization,
    axum::{routing::get, Extension},
};
use models::{
    organization_content::{
        OrganizationContent, OrganizationContentGetResponse, OrganizationContentParam,
        OrganizationContentQuery, OrganizationContentRepository, OrganizationContentSummary,
    },
    *,
};

#[derive(Clone, Debug)]
pub struct OrganizationProjectController {
    content_repo: OrganizationContentRepository,
}

impl OrganizationProjectController {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let content_repo = OrganizationContent::get_repository(pool.clone());
        let ctrl = OrganizationProjectController { content_repo };

        Ok(by_axum::axum::Router::new()
            .route("/", get(Self::list_projects))
            .route("/:id", get(Self::get_project))
            .with_state(ctrl.clone()))
    }

    pub async fn get_project(
        State(ctrl): State<OrganizationProjectController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(id): Path<i64>,
    ) -> Result<Json<OrganizationContent>> {
        //TODO: implement get_content
        let _repo = ctrl.content_repo;
        tracing::debug!("get_project: {:?}", id);

        Ok(Json(OrganizationContent {
            id,
            created_at: 1741585389,
            updated_at: 1741585389,
            name: "test organization name".to_string(),
            description: Some("test organization description".to_string()),
            projects: 100,
            votes: 200,
        }))
    }

    pub async fn list_projects(
        State(ctrl): State<OrganizationProjectController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(params): Query<OrganizationContentParam>,
    ) -> Result<Json<OrganizationContentGetResponse>> {
        //TODO: implement list_contents
        let _repo = ctrl.clone().content_repo;
        tracing::debug!("list_projects: {:?}", params);

        match params {
            OrganizationContentParam::Query(params) => ctrl.find(params).await,
        }
    }
}

impl OrganizationProjectController {
    pub async fn find(
        &self,
        OrganizationContentQuery { size, bookmark, .. }: OrganizationContentQuery,
    ) -> Result<Json<OrganizationContentGetResponse>> {
        let _size = size;
        let _bookmark = bookmark;

        tracing::debug!("find query");

        Ok(Json(OrganizationContentGetResponse::Query(QueryResponse {
            items: vec![
                OrganizationContentSummary {
                    id: 1,
                    created_at: 1741585389,
                    updated_at: 1741585389,
                    name: "test organization name".to_string(),
                    description: Some("test organization description".to_string()),
                    projects: 100,
                    votes: 200,
                };
                5
            ],
            total_count: 5,
        })))
    }
}
