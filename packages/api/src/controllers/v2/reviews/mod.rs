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
use sqlx::postgres::PgRow;

#[derive(Clone, Debug)]
pub struct ReviewController {
    review_repo: ReviewRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl ReviewController {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let review_repo = Review::get_repository(pool.clone());
        let ctrl = ReviewController { review_repo, pool };

        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::act_review).get(Self::list_reviews))
            .route("/:id", post(Self::act_by_id).get(Self::get_review))
            .with_state(ctrl.clone()))
    }

    pub async fn get_review(
        State(ctrl): State<ReviewController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(id): Path<i64>,
    ) -> Result<Json<Review>> {
        let review = Review::query_builder()
            .id_equals(id)
            .query()
            .map(Review::from)
            .fetch_one(&ctrl.pool)
            .await?;

        Ok(Json(review))
    }

    pub async fn act_by_id(
        State(ctrl): State<ReviewController>,
        Path(id): Path<i64>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<ReviewByIdAction>,
    ) -> Result<Json<Review>> {
        tracing::debug!("act_by_id: {:?} {:?}", id, body);
        auth.ok_or(ApiError::Unauthorized)?;

        match body {
            ReviewByIdAction::Update(params) => ctrl.update(id, params).await,
            ReviewByIdAction::Delete(_) => ctrl.delete(id).await,
        }
    }

    pub async fn list_reviews(
        State(ctrl): State<ReviewController>,
        Query(params): Query<ReviewParam>,
        Extension(_auth): Extension<Option<Authorization>>,
    ) -> Result<Json<ReviewGetResponse>> {
        tracing::debug!("list_reviews: {:?}", params);

        match params {
            ReviewParam::Query(params) => match params.action {
                Some(ReviewQueryActionType::SearchBy) => ctrl.search_by(params).await,
                _ => ctrl.find(params).await,
            },
        }
    }

    pub async fn act_review(
        State(ctrl): State<ReviewController>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<ReviewAction>,
    ) -> Result<Json<Review>> {
        tracing::debug!("act review {:?}", body);
        auth.ok_or(ApiError::Unauthorized)?;

        match body {
            ReviewAction::Create(params) => ctrl.create(params).await,
        }
    }
}

impl ReviewController {
    pub async fn update(&self, id: i64, params: ReviewUpdateRequest) -> Result<Json<Review>> {
        tracing::debug!("update review: {:?} {:?}", id, params);

        let review = self.review_repo.update(id, params.into()).await?;

        Ok(Json(review))
    }

    pub async fn find(&self, q: ReviewQuery) -> Result<Json<ReviewGetResponse>> {
        let mut total_count: i64 = 0;
        let items: Vec<ReviewSummary> = Review::query_builder()
            .limit(q.size())
            .page(q.page())
            .with_count()
            .query()
            .map(|r: sqlx::postgres::PgRow| {
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

    pub async fn search_by(&self, q: ReviewQuery) -> Result<Json<ReviewGetResponse>> {
        let mut total_count: i64 = 0;

        let items = ReviewSummary::query_builder()
            .name_contains(q.clone().name.unwrap_or_default())
            .limit(q.size())
            .page(q.page())
            .query()
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

        let review = self
            .review_repo
            .insert(
                params.deliberation_id,
                params.user_id,
                params.name,
                params.image,
                params.review,
            )
            .await?;

        Ok(Json(review))
    }

    pub async fn delete(&self, review_id: i64) -> Result<Json<Review>> {
        tracing::debug!("delete review: {:?}", review_id);
        let review = self.review_repo.delete(review_id).await?;

        Ok(Json(review))
    }
}
