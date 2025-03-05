use std::collections::HashMap;

#[allow(unused)]
use by_axum::axum::{
    extract::{Path, Query, State},
    routing::post,
    Json,
};
use by_axum::{
    auth::{generate_jwt, Authorization},
    axum::{routing::get, Extension},
};
use by_types::{Claims, JsonWithHeaders};
use models::ParticipantUserRepository;
use models::*;
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
            .with_state(ctrl.clone()))
    }

    pub async fn act_user(
        State(ctrl): State<ParticipantUserControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<ParticipantUserAction>,
    ) -> Result<JsonWithHeaders<ParticipantUser>> {
        tracing::debug!("act_user: {:?}", body);
        body.validate()?;

        match body {
            ParticipantUserAction::Signup(req) => ctrl.signup(req).await,
            ParticipantUserAction::Login(req) => ctrl.login(req).await,
        }
    }

    #[instrument]
    pub async fn read_user(
        State(ctrl): State<ParticipantUserControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Query(req): Query<ParticipantUserReadAction>,
    ) -> Result<Json<ParticipantUser>> {
        req.validate()?;

        match req.action {
            Some(ParticipantUserReadActionType::CheckEmail) => ctrl.check_email(req).await,
            Some(ParticipantUserReadActionType::UserInfo) => ctrl.user_info(req).await,
            None => Err(ApiError::BadRequest),
        }
    }
}

impl ParticipantUserControllerV1 {
    pub async fn login(
        &self,
        req: ParticipantUserLoginRequest,
    ) -> Result<JsonWithHeaders<ParticipantUser>> {
        let user = self
            .users
            .find_one(&ParticipantUserReadAction {
                action: Some(ParticipantUserReadActionType::UserInfo),
                email: Some(req.email),
            })
            .await?;

        let jwt = self.generate_token(&user)?;

        Ok(JsonWithHeaders::new(user)
            .with_bearer_token(&jwt)
            .with_cookie(&jwt))
    }

    pub async fn signup(
        &self,
        req: ParticipantUserSignupRequest,
    ) -> Result<JsonWithHeaders<ParticipantUser>> {
        let user = self
            .users
            .insert(req.nickname, req.email, req.profile_url)
            .await?;

        let jwt = self.generate_token(&user)?;

        Ok(JsonWithHeaders::new(user)
            .with_bearer_token(&jwt)
            .with_cookie(&jwt))
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

impl ParticipantUserControllerV1 {
    pub fn generate_token(&self, user: &ParticipantUser) -> Result<String> {
        let mut claims = Claims {
            sub: user.id.to_string(),
            role: by_types::Role::User,
            custom: HashMap::from([("email".to_string(), user.email.clone())]),
            ..Claims::default()
        };

        generate_jwt(&mut claims).map_err(|e| {
            tracing::error!("Failed to generate JWT: {}", e);
            ApiError::JWTGenerationFail(e.to_string())
        })
    }
}
