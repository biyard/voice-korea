#[allow(unused)]
use by_axum::axum::{
    extract::{Path, Query, State},
    routing::post,
    Json,
};
use models::{
    v2::{
        Review, ReviewAction, ReviewByIdAction, ReviewCreateRequest, ReviewGetResponse,
        ReviewSummary,ReviewRepositoryUpdateRequest,ReviewReadAction,
        ReviewParam, ReviewQuery, ReviewQueryActionType, ReviewRepository, ReviewUpdateRequest,
    },
    *,
};

use sqlx::postgres::PgRow;
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

        let fetched_review = _repo
            .find_one(&ReviewReadAction::new().find_by_id(id))
            .await?;

        Ok(Json(fetched_review))
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
            _ => Err(ApiError::InvalidAction),
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

        let fetched_review = self.review_repo
            .update(
                id,
                ReviewRepositoryUpdateRequest {
                    name: Some(params.name),
                    image: Some(params.image),
                    review: Some(params.review),
                },
            )
            .await?;

        Ok(Json(fetched_review))
    }

    pub async fn find(
        &self,
        ReviewQuery { size, bookmark, .. }: ReviewQuery,
    ) -> Result<Json<ReviewGetResponse>> {
        let _size = size as i64;
        let _bookmark = bookmark;

        let mut total_count: i64 = 0;
        let query = ReviewSummary::base_sql_with("limit $2 offset $3");

        tracing::debug!("find query");

        let items: Vec<ReviewSummary> = sqlx::query(&query)
            .bind(_size as i64)
            .bind(_size as i64 * (_bookmark.unwrap_or("1".to_string()).parse::<i64>().unwrap() - 1))
            .map(|r: PgRow| {
                use sqlx::Row;
                total_count = r.get("total_count");
                r.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(Json(ReviewGetResponse::Query(QueryResponse {
            items,
            total_count,
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
        let _size = size as i64;
        let _bookmark = bookmark;
        let _name = name;
        let mut total_count: i64 = 0;
        let query = ReviewSummary::base_sql_with("name ilike $1 limit $2 offset $3");
        tracing::debug!("search_by query: {}", query);

        let items: Vec<ReviewSummary> = sqlx::query(&query)
            .bind(format!("%{}%", _name.unwrap()))
            .bind(size as i64)
            .bind(size as i64 * (_bookmark.unwrap_or("1".to_string()).parse::<i64>().unwrap() - 1))
            .map(|r: PgRow| {
                use sqlx::Row;

                total_count = r.get("total_count");
                r.into()
            })
            .fetch_all(&self.pool)
            .await?;

         Ok(Json(ReviewGetResponse::Query(QueryResponse {
            items,
            total_count,
        })))
    }

    pub async fn create(&self, params: ReviewCreateRequest) -> Result<Json<Review>> {
        tracing::debug!("create review: {:?}", params);
        let new_review = self
            .review_repo
            .insert(params.name, params.image, params.review)
            .await?;

        Ok(Json(new_review))
    }

    pub async fn delete(&self, review_id: i64) -> Result<Json<Review>> {
        tracing::debug!("delete review: {:?}", review_id);
        let _ = self.review_repo.delete(review_id).await?;
        Ok(Json(Review::default()))
    }
}
