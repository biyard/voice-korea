#[allow(unused)]
use by_axum::axum::{
    extract::{Path, Query, State},
    routing::post,
    Json,
};
use by_axum::{auth::Authorization, axum::Extension};
use models::{
    review::{
        Review, ReviewAction, ReviewByIdAction, ReviewCreateRequest, ReviewGetResponse,
        ReviewParam, ReviewQuery, ReviewQueryActionType, ReviewRepository, ReviewSummary,
        ReviewUpdateRequest,
    },
    *,
};

#[derive(Clone, Debug)]
pub struct ReviewControllerV2 {
    review_repo: ReviewRepository,
}

impl ReviewControllerV2 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let review_repo = Review::get_repository(pool.clone());
        let ctrl = ReviewControllerV2 { review_repo };

        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::act_review).get(Self::list_reviews))
            .route("/:id", post(Self::act_by_id).get(Self::get_review))
            .with_state(ctrl.clone()))
    }

    pub async fn get_review(
        State(ctrl): State<ReviewControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(id): Path<i64>,
    ) -> Result<Json<Review>> {
        //TODO: implement get review
        let _repo = ctrl.review_repo;
        tracing::debug!("get_review: {:?}", id);

        Ok(Json(Review {
            id,
            created_at: 1741585389,
            updated_at: 1741585389,
            deliberation_id: 1,
            user_id: 1,
            name: "profile name".to_string(),
            image: "profile image".to_string(),
            review: "test review".to_string(),
        }))
    }

    pub async fn act_by_id(
        State(ctrl): State<ReviewControllerV2>,
        Path(id): Path<i64>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<ReviewByIdAction>,
    ) -> Result<Json<Review>> {
        //TODO: implement act_by_id
        //TODO: add authorization by auth params
        let _repo = ctrl.clone().review_repo;
        tracing::debug!("act_by_id: {:?} {:?}", id, body);

        match body {
            ReviewByIdAction::Update(params) => ctrl.update(id, params).await,
        }
    }

    pub async fn list_reviews(
        State(ctrl): State<ReviewControllerV2>,
        Query(params): Query<ReviewParam>,
        Extension(_auth): Extension<Option<Authorization>>,
    ) -> Result<Json<ReviewGetResponse>> {
        //TODO: implement list_reviews
        let _repo = ctrl.clone().review_repo;
        tracing::debug!("list_reviews: {:?}", params);

        match params {
            ReviewParam::Query(params) => match params.action {
                Some(ReviewQueryActionType::SearchBy) => ctrl.search_by(params).await,
                _ => ctrl.find(params).await,
            },
        }
    }

    pub async fn act_review(
        State(ctrl): State<ReviewControllerV2>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<ReviewAction>,
    ) -> Result<Json<Review>> {
        //TODO: implement act_review
        //TODO: add authorization by auth params
        let _repo = ctrl.clone().review_repo;
        tracing::debug!("act review {:?}", body);

        match body {
            ReviewAction::Delete(params) => ctrl.delete(params.id).await,
            ReviewAction::Create(params) => ctrl.create(params).await,
        }
    }
}

impl ReviewControllerV2 {
    pub async fn update(&self, id: i64, params: ReviewUpdateRequest) -> Result<Json<Review>> {
        tracing::debug!("update review: {:?} {:?}", id, params);

        Ok(Json(Review {
            id,
            created_at: 1741585389,
            updated_at: 1741585389,
            deliberation_id: 1,
            user_id: 1,
            name: "profile name".to_string(),
            image: "profile image".to_string(),
            review: "test review".to_string(),
        }))
    }

    pub async fn find(
        &self,
        ReviewQuery { size, bookmark, .. }: ReviewQuery,
    ) -> Result<Json<ReviewGetResponse>> {
        let _size = size;
        let _bookmark = bookmark;

        tracing::debug!("find query");

        Ok(Json(ReviewGetResponse::Query(QueryResponse {
            items: vec![
                ReviewSummary {
                    id: 1,
                    created_at: 1741585389,
                    updated_at: 1741585389,
                    deliberation_id: 1,
                    user_id: 1,
                    name: "profile name".to_string(),
                    image: "profile image".to_string(),
                    review: "test review".to_string(),
                };
                3
            ],
            total_count: 3,
        })))
    }

    pub async fn search_by(
        &self,
        ReviewQuery {
            size,
            bookmark,
            name,
            ..
        }: ReviewQuery,
    ) -> Result<Json<ReviewGetResponse>> {
        let _size = size;
        let _bookmark = bookmark;
        let _name = name;
        tracing::debug!("search by");

        Ok(Json(ReviewGetResponse::Query(QueryResponse {
            items: vec![
                ReviewSummary {
                    id: 1,
                    created_at: 1741585389,
                    updated_at: 1741585389,
                    deliberation_id: 1,
                    user_id: 1,
                    name: "profile name".to_string(),
                    image: "profile image".to_string(),
                    review: "test review".to_string(),
                };
                3
            ],
            total_count: 3,
        })))
    }

    pub async fn create(&self, params: ReviewCreateRequest) -> Result<Json<Review>> {
        tracing::debug!("create review: {:?}", params);

        Ok(Json(Review {
            id: 1,
            created_at: 1741585389,
            updated_at: 1741585389,
            deliberation_id: 1,
            user_id: 1,
            name: "profile name".to_string(),
            image: "profile image".to_string(),
            review: "test review".to_string(),
        }))
    }

    pub async fn delete(&self, review_id: i64) -> Result<Json<Review>> {
        tracing::debug!("delete review: {:?}", review_id);

        Ok(Json(Review {
            id: 1,
            created_at: 1741585389,
            updated_at: 1741585389,
            deliberation_id: 1,
            user_id: 1,
            name: "profile name".to_string(),
            image: "profile image".to_string(),
            review: "test review".to_string(),
        }))
    }
}
