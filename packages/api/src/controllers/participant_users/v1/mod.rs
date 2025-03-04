#[allow(unused)]
use by_axum::axum::{
    extract::{Path, Query, State},
    routing::post,
    Json,
};
use by_axum::{
    auth::authorization_middleware,
    axum::{middleware, routing::get, Extension},
};
use models::ParticipantUserRepository;
use models::*;
use rest_api::Signature;
use sqlx::{Pool, Postgres};
use tracing::instrument;
use validator::Validate;

#[derive(Clone, Debug)]
pub struct ParticipantUserControllerV1 {
    users: ParticipantUserRepository,
}

impl ParticipantUserControllerV1 {
    pub fn route(pool: Pool<Postgres>) -> Result<by_axum::axum::Router> {
        let users = ParticipantUser::get_repository(pool.clone());

        let ctrl = ParticipantUserControllerV1 { users };

        Ok(by_axum::axum::Router::new()
            .route("/", get(Self::read_user).post(Self::act_user))
            .with_state(ctrl.clone())
            .layer(middleware::from_fn(authorization_middleware)))
    }

    #[instrument]
    pub async fn act_user(
        State(ctrl): State<ParticipantUserControllerV1>,
        Extension(sig): Extension<Option<Signature>>,
        Json(body): Json<ParticipantUserAction>,
    ) -> Result<Json<ParticipantUser>> {
        tracing::debug!("act_user: sig={:?} {:?}", sig, body);
        let sig = sig.ok_or(ApiError::Unauthorized)?;
        body.validate()?;

        match body {
            ParticipantUserAction::Signup(req) => ctrl.signup(req, sig).await,
        }
    }

    #[instrument]
    pub async fn read_user(
        State(ctrl): State<ParticipantUserControllerV1>,
        Extension(sig): Extension<Option<Signature>>,

        Query(mut req): Query<ParticipantUserReadAction>,
    ) -> Result<Json<ParticipantUser>> {
        tracing::debug!("read_user: sig={:?}", sig);
        let principal = sig
            .ok_or(ApiError::Unauthorized)?
            .principal()
            .map_err(|s| {
                tracing::error!("failed to get principal: {:?}", s);
                ApiError::Unknown(s.to_string())
            })?;
        req.validate()?;

        match req.action {
            Some(ParticipantUserReadActionType::CheckEmail) => ctrl.check_email(req).await,
            Some(ParticipantUserReadActionType::UserInfo) => {
                req.principal = Some(principal);
                ctrl.user_info(req).await
            }
            Some(ParticipantUserReadActionType::Login) => {
                req.principal = Some(principal);
                ctrl.login(req).await
            }
            None | Some(ParticipantUserReadActionType::ByPrincipal) => Err(ApiError::BadRequest)?,
        }
    }
}

impl ParticipantUserControllerV1 {
    pub async fn login(&self, req: ParticipantUserReadAction) -> Result<Json<ParticipantUser>> {
        let user = self.users.find_one(&req).await?;

        Ok(Json(user))
    }

    pub async fn signup(
        &self,
        req: ParticipantUserSignupRequest,
        sig: Signature,
    ) -> Result<Json<ParticipantUser>> {
        let principal = sig.principal().map_err(|s| {
            tracing::error!("failed to get principal: {:?}", s);
            ApiError::Unauthorized
        })?;

        let user = self
            .users
            .insert(req.nickname, principal, req.email, req.profile_url)
            .await?;

        Ok(Json(user))
    }

    pub async fn check_email(
        &self,
        req: ParticipantUserReadAction,
    ) -> Result<Json<ParticipantUser>> {
        let user = self
            .users
            .find_one(&req)
            .await
            .map_err(|_| ApiError::NotFound)?;

        Ok(Json(user))
    }

    pub async fn user_info(&self, req: ParticipantUserReadAction) -> Result<Json<ParticipantUser>> {
        let user = self.users.find_one(&req).await?;

        Ok(Json(user))
    }
}
