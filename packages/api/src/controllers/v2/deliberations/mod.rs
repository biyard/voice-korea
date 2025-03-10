pub mod _id {
    pub mod responses;
}

#[allow(unused)]
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
    deliberation_content::{
        DeliberationContent, DeliberationContentGetResponse, DeliberationContentParam,
        DeliberationContentQuery, DeliberationContentRepository, DeliberationContentSummary,
    },
    *,
};

#[derive(Clone, Debug)]
pub struct DeliberationControllerV2 {
    deliberation_repo: DeliberationContentRepository,
}

impl DeliberationControllerV2 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let deliberation_repo = DeliberationContent::get_repository(pool.clone());
        let ctrl = DeliberationControllerV2 { deliberation_repo };

        Ok(by_axum::axum::Router::new()
            .route("/", get(Self::list_deliberations))
            .route("/:id", get(Self::get_deliberation))
            .with_state(ctrl.clone()))
    }

    pub async fn get_deliberation(
        State(ctrl): State<DeliberationControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(id): Path<i64>,
    ) -> Result<Json<DeliberationContent>> {
        //TODO: implement get deliberation
        let _repo = ctrl.deliberation_repo;
        tracing::debug!("get_deliberation: {:?}", id);

        Ok(Json(DeliberationContent {
            id,
            created_at: 1741585389,
            updated_at: 1741585389,
            title: "deliberation title".to_string(),
            description: "deliberation description".to_string(),
            project_area: ProjectArea::Society,
            org_id: 1,
            participants: 100,
            votes: 150,
        }))
    }

    pub async fn list_deliberations(
        State(ctrl): State<DeliberationControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(params): Query<DeliberationContentParam>,
    ) -> Result<Json<DeliberationContentGetResponse>> {
        //TODO: implement list_deliberations
        let _repo = ctrl.clone().deliberation_repo;
        tracing::debug!("list_deliberations: {:?}", params);

        match params {
            DeliberationContentParam::Query(params) => ctrl.find(params).await,
        }
    }
}

impl DeliberationControllerV2 {
    pub async fn find(
        &self,
        DeliberationContentQuery { size, bookmark, .. }: DeliberationContentQuery,
    ) -> Result<Json<DeliberationContentGetResponse>> {
        let _size = size;
        let _bookmark = bookmark;

        tracing::debug!("find query");

        Ok(Json(DeliberationContentGetResponse::Query(QueryResponse {
            items: vec![
                DeliberationContentSummary {
                    id: 1,
                    created_at: 1741585389,
                    updated_at: 1741585389,
                    title: "deliberation title".to_string(),
                    description: "deliberation description".to_string(),
                    project_area: ProjectArea::Society,
                    org_id: 1,
                    participants: 100,
                    votes: 150,
                };
                5
            ],
            total_count: 5,
        })))
    }
}
