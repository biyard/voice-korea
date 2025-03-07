use by_axum::{
    auth::Authorization,
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Extension, Json,
    },
};
use by_types::QueryResponse;
use models::*;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DeliberationResponseController {
    repo: DeliberationResponseRepository,
    deliberation: DeliberationRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl DeliberationResponseController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = DeliberationResponse::get_repository(pool.clone());
        let deliberation = Deliberation::get_repository(pool.clone());

        Self {
            repo,
            pool,
            deliberation,
        }
    }

    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let ctrl = Self::new(pool);

        Ok(by_axum::axum::Router::new()
            .route("/:id", get(Self::get_deliberation_response))
            .with_state(ctrl.clone())
            .route(
                "/",
                post(Self::act_deliberation_response).get(Self::list_deliberation_response),
            )
            .with_state(ctrl.clone()))
    }

    pub async fn get_deliberation_response(
        State(_ctrl): State<DeliberationResponseController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path((deliberation_id, id)): Path<(i64, i64)>,
    ) -> Result<Json<DeliberationResponse>> {
        //TODO: implement get_deliberation_response
        tracing::debug!("get_deliberation_response {} {}", deliberation_id, id);
        Ok(Json(DeliberationResponse::default()))
    }

    pub async fn list_deliberation_response(
        State(_ctrl): State<DeliberationResponseController>,
        Path(deliberation_id): Path<i64>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(q): Query<DeliberationResponseParam>,
    ) -> Result<Json<DeliberationResponseGetResponse>> {
        //TODO: implement list_deliberation_response
        tracing::debug!("list_deliberation_response {} {:?}", deliberation_id, q);

        match q {
            DeliberationResponseParam::Query(_q) => Ok(Json(
                DeliberationResponseGetResponse::Query(QueryResponse {
                    total_count: 0,
                    items: vec![],
                }),
            )),
        }
    }

    pub async fn act_deliberation_response(
        State(ctrl): State<DeliberationResponseController>,
        Path(deliberation_id): Path<i64>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<DeliberationResponseAction>,
    ) -> Result<Json<DeliberationResponse>> {
        //TODO: implement act_deliberation_response
        tracing::debug!("act_deliberation_response {} {:?}", deliberation_id, body);

        match body {
            DeliberationResponseAction::RespondAnswer(req) => {
                ctrl.respond_answer(deliberation_id, auth, req).await
            }
        }
    }
}

impl DeliberationResponseController {
    pub async fn respond_answer(
        &self,
        deliberation_id: i64,
        _auth: Option<Authorization>,
        DeliberationResponseRespondAnswerRequest {
            user_id,
            answers,
            deliberation_type,
        }: DeliberationResponseRespondAnswerRequest,
    ) -> Result<Json<DeliberationResponse>> {
        // auth.ok_or(ApiError::Unauthorized)?;
        let _deliberation_id = deliberation_id;
        let _user_id = user_id;
        let _answers = answers;
        let _deliberation_type = deliberation_type;

        Ok(Json(DeliberationResponse::default()))
    }
}
