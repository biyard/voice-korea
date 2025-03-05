#[allow(unused)]
use by_axum::axum::{
    extract::{Path, Query, State},
    routing::post,
    Json,
};
use models::{
    v2::{
        Review, ReviewAction, ReviewByIdAction, ReviewCreateRequest, ReviewGetResponse,
        ReviewSummary,
        ReviewParam, ReviewQuery, ReviewQueryActionType, ReviewRepository, ReviewUpdateRequest,
    },
    *,
};

#[derive(Clone, Debug)]
pub struct ReviewControllerV1 {
    review_repo: ReviewRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl ReviewControllerV1 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let review_repo = Review::get_repository(pool.clone());
        let ctrl = ReviewControllerV1 { review_repo, pool };

        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::act_review).get(Self::list_reviews))
            .route("/:id", post(Self::act_by_id).get(Self::get_review))
            .with_state(ctrl.clone()))
    }

    pub async fn get_review(
        State(ctrl): State<ReviewControllerV1>,
        Path(id): Path<i64>,
    ) -> Result<Json<Review>> {
        let _repo = ctrl.review_repo;
        tracing::debug!("get_review: {:?}", id);

        let fetched_review = sqlx::query_as!(
            Review,
            "SELECT * FROM reviews WHERE id = $1",
            id
        )
        .fetch_one(&ctrl.pool)
        .await;
    
        match fetched_review {
            Ok(review) => Ok(Json(review)),
            Err(_) => Err(ApiError::ResourceNotFound),
        }
    }

    pub async fn act_by_id(
        State(ctrl): State<ReviewControllerV1>,
        Path(id): Path<i64>,
        Json(body): Json<ReviewByIdAction>,
    ) -> Result<Json<Review>> {
        let _repo = ctrl.clone().review_repo;
        tracing::debug!("act_by_id: {:?} {:?}", id, body);

        match body {
            ReviewByIdAction::Update(params) => ctrl.update(id, params).await,
        }
    }

    pub async fn list_reviews(
        State(ctrl): State<ReviewControllerV1>,
        Query(params): Query<ReviewParam>,
    ) -> Result<Json<ReviewGetResponse>> {
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
        State(ctrl): State<ReviewControllerV1>,
        Json(body): Json<ReviewAction>,
    ) -> Result<Json<Review>> {
        let _repo = ctrl.clone().review_repo;
        tracing::debug!("act review {:?}", body);

        match body {
            ReviewAction::Delete(params) => ctrl.delete(params.id).await,
            ReviewAction::Create(params) => ctrl.create(params).await,
        }
    }
}

impl ReviewControllerV1 {
    pub async fn update(&self, id: i64, params: ReviewUpdateRequest) -> Result<Json<Review>> {
        tracing::debug!("update review: {:?} {:?}", id, params);

        let fetched_review = sqlx::query_as!(
            Review,
            "UPDATE reviews SET name = $1, image = $2, review = $3 WHERE id = $4 RETURNING *",
            params.name,
            params.image,
            params.review,
            id
        )
        .fetch_one(&self.pool)
        .await;

        match fetched_review {
            Ok(review) => Ok(Json(review)),
            Err(_) => Err(ApiError::ApiCallError("Something went wrong".to_string())),
        }
    }

    pub async fn find(
        &self,
        ReviewQuery { size, bookmark, .. }: ReviewQuery,
    ) -> Result<Json<ReviewGetResponse>> {
        let _size = size as i64;
        let _bookmark = bookmark;

        tracing::debug!("find query");

       let found_reviews = sqlx::query_as!(
            Review,
            "SELECT * FROM reviews ORDER BY created_at DESC LIMIT $1",
            _size
        )
        .fetch_all(&self.pool)
        .await;

        match found_reviews {
            Ok(reviews) => Ok(Json(ReviewGetResponse::Query(QueryResponse {
                total_count: reviews.len() as i64,
                items: reviews.into_iter().map(ReviewSummary::from).collect(),
            }))),
            Err(_) => Err(ApiError::ApiCallError("Something went wrong".to_string())),
        }
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
        let _size = size as i64;
        let _bookmark = bookmark;
        let _name = name;
        tracing::debug!("search by");

         let search_results = sqlx::query_as!(
            Review,
            "SELECT * FROM reviews WHERE name ILIKE $1 ORDER BY created_at DESC LIMIT $2",
            format!("%{}%", _name.unwrap_or_default()),
            _size
        )
        .fetch_all(&self.pool)
        .await;

        match search_results {
            Ok(reviews) => Ok(Json(ReviewGetResponse::Query(QueryResponse {
                total_count: reviews.len() as i64,
                items: reviews.into_iter().map(ReviewSummary::from).collect(),
            }))),
           Err(_) => Err(ApiError::ApiCallError("Something went wrong".to_string())),
        }
    }

    pub async fn create(&self, params: ReviewCreateRequest) -> Result<Json<Review>> {
        tracing::debug!("create review: {:?}", params);
        let new_review = sqlx::query_as!(
            Review,
            "INSERT INTO reviews (name, image, review) VALUES ($1, $2, $3) RETURNING *",
            params.name,
            params.image,
            params.review
        )
        .fetch_one(&self.pool)
        .await;

        match new_review {
            Ok(review) => Ok(Json(review)),
            Err(_) => Err(ApiError::ApiCallError("Something went wrong".to_string())),
        }
    }

    pub async fn delete(&self, review_id: i64) -> Result<Json<Review>> {
        tracing::debug!("delete review: {:?}", review_id);
        let found_review = sqlx::query_as!(
            Review,
            "DELETE FROM reviews WHERE id = $1 RETURNING *",
            review_id
        )
        .fetch_one(&self.pool)
        .await;

        match found_review {
            Ok(review) => Ok(Json(review)),
            Err(_) => Err(ApiError::ApiCallError("Something went wrong".to_string())),
        }
    }
}
