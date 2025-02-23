#[allow(unused)]
use by_axum::axum::{
    extract::{Path, Query, State},
    routing::post,
    Json,
};
use models::{
    v2::{
        Institution, InstitutionRepository, PublicOpinionProject, PublicOpinionProjectAction,
        PublicOpinionProjectByIdAction, PublicOpinionProjectCreateRequest,
        PublicOpinionProjectGetResponse, PublicOpinionProjectParam, PublicOpinionProjectQuery,
        PublicOpinionProjectQueryActionType, PublicOpinionProjectRepository,
        PublicOpinionProjectUpdateRequest,
    },
    *,
};

#[derive(Clone, Debug)]
pub struct OpinionControllerV2 {
    opinion_repo: PublicOpinionProjectRepository,
    institution_repo: InstitutionRepository,
}

impl OpinionControllerV2 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let opinion_repo = PublicOpinionProject::get_repository(pool.clone());
        let institution_repo = Institution::get_repository(pool.clone());
        let ctrl = OpinionControllerV2 {
            opinion_repo,
            institution_repo,
        };

        // FIXME: checking condition by organization id
        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::act_opinion).get(Self::list_opinions))
            .route("/:id", post(Self::act_by_id).get(Self::get_opinion))
            .with_state(ctrl.clone()))
    }

    pub async fn get_opinion(
        State(ctrl): State<OpinionControllerV2>,
        Path(id): Path<i64>,
    ) -> Result<Json<PublicOpinionProject>> {
        //TODO: implement get opinion
        let _repo = ctrl.clone().opinion_repo;
        let _institution_repo = ctrl.clone().institution_repo;
        tracing::debug!("get_opinion: {:?}", id);

        Ok(Json(PublicOpinionProject::default()))
    }

    pub async fn act_by_id(
        State(ctrl): State<OpinionControllerV2>,
        Path(id): Path<i64>,
        Json(body): Json<PublicOpinionProjectByIdAction>,
    ) -> Result<Json<PublicOpinionProject>> {
        //TODO: implement act_by_id
        let _repo = ctrl.clone().opinion_repo;
        let _institution_repo = ctrl.clone().institution_repo;
        tracing::debug!("act_by_id: {:?} {:?}", id, body);

        match body {
            PublicOpinionProjectByIdAction::Update(params) => ctrl.update(id, params).await,
        }
    }

    pub async fn list_opinions(
        State(ctrl): State<OpinionControllerV2>,
        Query(params): Query<PublicOpinionProjectParam>,
    ) -> Result<Json<PublicOpinionProjectGetResponse>> {
        //TODO: implement list_opinions
        let _repo = ctrl.clone().opinion_repo;
        let _institution_repo = ctrl.clone().institution_repo;
        tracing::debug!("list_opinions: {:?}", params);

        match params {
            PublicOpinionProjectParam::Query(params) => match params.action {
                Some(PublicOpinionProjectQueryActionType::SearchBy) => ctrl.search_by(params).await,
                _ => ctrl.find(params).await,
            },
        }
    }

    pub async fn act_opinion(
        State(ctrl): State<OpinionControllerV2>,
        Json(body): Json<PublicOpinionProjectAction>,
    ) -> Result<Json<PublicOpinionProject>> {
        //TODO: implement act_opinion
        let _repo = ctrl.clone().opinion_repo;
        let _institution_repo = ctrl.clone().institution_repo;
        tracing::debug!("act opinion {:?}", body);

        match body {
            PublicOpinionProjectAction::Delete(params) => ctrl.delete(params.id).await,
            PublicOpinionProjectAction::Create(params) => ctrl.create(params).await,
        }
    }
}

impl OpinionControllerV2 {
    pub async fn update(
        &self,
        id: i64,
        params: PublicOpinionProjectUpdateRequest,
    ) -> Result<Json<PublicOpinionProject>> {
        tracing::debug!("update opinion: {:?} {:?}", id, params);

        Ok(Json(PublicOpinionProject::default()))
    }

    pub async fn find(
        &self,
        PublicOpinionProjectQuery { size, bookmark, .. }: PublicOpinionProjectQuery,
    ) -> Result<Json<PublicOpinionProjectGetResponse>> {
        let _size = size;
        let _bookmark = bookmark;

        tracing::debug!("find query");

        Ok(Json(PublicOpinionProjectGetResponse::Query(
            QueryResponse {
                items: vec![],
                total_count: 0,
            },
        )))
    }

    pub async fn search_by(
        &self,
        PublicOpinionProjectQuery {
            size,
            bookmark,
            title,
            ..
        }: PublicOpinionProjectQuery,
    ) -> Result<Json<PublicOpinionProjectGetResponse>> {
        let _size = size;
        let _bookmark = bookmark;
        let _title = title;
        tracing::debug!("search by");

        Ok(Json(PublicOpinionProjectGetResponse::Query(
            QueryResponse {
                items: vec![],
                total_count: 0,
            },
        )))
    }

    pub async fn create(
        &self,
        params: PublicOpinionProjectCreateRequest,
    ) -> Result<Json<PublicOpinionProject>> {
        tracing::debug!("create opinion: {:?}", params);

        Ok(Json(PublicOpinionProject::default()))
    }

    pub async fn delete(&self, opinion_id: i64) -> Result<Json<PublicOpinionProject>> {
        tracing::debug!("delete opinion: {:?}", opinion_id);

        Ok(Json(PublicOpinionProject::default()))
    }
}
