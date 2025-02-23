#[allow(unused)]
use by_axum::axum::{
    extract::{Path, Query, State},
    routing::post,
    Json,
};
use models::{
    v2::{
        Institution, InstitutionAction, InstitutionByIdAction, InstitutionCreateRequest,
        InstitutionGetResponse, InstitutionParam, InstitutionQuery, InstitutionQueryActionType,
        InstitutionRepository, InstitutionUpdateRequest,
    },
    *,
};

#[derive(Clone, Debug)]
pub struct InstitutionControllerM1 {
    institution_repo: InstitutionRepository,
}

impl InstitutionControllerM1 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let institution_repo = Institution::get_repository(pool.clone());
        let ctrl = InstitutionControllerM1 { institution_repo };

        Ok(by_axum::axum::Router::new()
            .route(
                "/",
                post(Self::act_institution).get(Self::list_institutions),
            )
            .route("/:id", post(Self::act_by_id).get(Self::get_institution))
            .with_state(ctrl.clone()))
    }

    pub async fn get_institution(
        State(ctrl): State<InstitutionControllerM1>,
        Path(id): Path<i64>,
    ) -> Result<Json<Institution>> {
        //TODO: implement get institution
        let _repo = ctrl.institution_repo;
        tracing::debug!("get_institution: {:?}", id);

        Ok(Json(Institution::default()))
    }

    pub async fn act_by_id(
        State(ctrl): State<InstitutionControllerM1>,
        Path(id): Path<i64>,
        Json(body): Json<InstitutionByIdAction>,
    ) -> Result<Json<Institution>> {
        //TODO: implement act_by_id
        let _repo = ctrl.clone().institution_repo;
        tracing::debug!("act_by_id: {:?} {:?}", id, body);

        match body {
            InstitutionByIdAction::Update(params) => ctrl.update(id, params).await,
        }
    }

    pub async fn list_institutions(
        State(ctrl): State<InstitutionControllerM1>,
        Query(params): Query<InstitutionParam>,
    ) -> Result<Json<InstitutionGetResponse>> {
        //TODO: implement list_institutions
        let _repo = ctrl.clone().institution_repo;
        tracing::debug!("list_institutions: {:?}", params);

        match params {
            InstitutionParam::Query(params) => match params.action {
                Some(InstitutionQueryActionType::SearchBy) => ctrl.search_by(params).await,
                _ => ctrl.find(params).await,
            },
        }
    }

    pub async fn act_institution(
        State(ctrl): State<InstitutionControllerM1>,
        Json(body): Json<InstitutionAction>,
    ) -> Result<Json<Institution>> {
        //TODO: implement act_institution
        let _repo = ctrl.clone().institution_repo;
        tracing::debug!("act institution {:?}", body);

        match body {
            InstitutionAction::Delete(params) => ctrl.delete(params.id).await,
            InstitutionAction::Create(params) => ctrl.create(params).await,
        }
    }
}

impl InstitutionControllerM1 {
    pub async fn update(
        &self,
        id: i64,
        params: InstitutionUpdateRequest,
    ) -> Result<Json<Institution>> {
        tracing::debug!("update institution: {:?} {:?}", id, params);

        Ok(Json(Institution::default()))
    }

    pub async fn find(
        &self,
        InstitutionQuery { size, bookmark, .. }: InstitutionQuery,
    ) -> Result<Json<InstitutionGetResponse>> {
        let _size = size;
        let _bookmark = bookmark;

        tracing::debug!("find query");

        Ok(Json(InstitutionGetResponse::Query(QueryResponse {
            items: vec![],
            total_count: 0,
        })))
    }

    pub async fn search_by(
        &self,
        InstitutionQuery {
            size,
            bookmark,
            name,
            ..
        }: InstitutionQuery,
    ) -> Result<Json<InstitutionGetResponse>> {
        let _size = size;
        let _bookmark = bookmark;
        let _name = name;
        tracing::debug!("search by");

        Ok(Json(InstitutionGetResponse::Query(QueryResponse {
            items: vec![],
            total_count: 0,
        })))
    }

    pub async fn create(&self, params: InstitutionCreateRequest) -> Result<Json<Institution>> {
        tracing::debug!("create institution: {:?}", params);

        Ok(Json(Institution::default()))
    }

    pub async fn delete(&self, institution_id: i64) -> Result<Json<Institution>> {
        tracing::debug!("delete institution: {:?}", institution_id);

        Ok(Json(Institution::default()))
    }
}
