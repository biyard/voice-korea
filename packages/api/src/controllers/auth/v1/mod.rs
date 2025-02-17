pub mod verification;

use std::collections::HashMap;

use by_axum::{
    auth::{generate_jwt, Authorization},
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Extension, Json,
    },
};
use by_types::{Claims, JsonWithHeaders};
use models::*;
use validator::Validate;
use verification::VerificationControllerV1;

use crate::utils::hash::get_hash_string;

#[derive(Clone, Debug)]
pub struct UserControllerV1 {
    repo: UserRepository,
    verification: VerificationRepository,
    org: OrganizationRepository,
}

impl UserControllerV1 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = User::get_repository(pool.clone());
        let verification = Verification::get_repository(pool.clone());
        let org = Organization::get_repository(pool.clone());

        let ctrl = UserControllerV1 {
            repo,
            verification,
            org,
        };

        Ok(by_axum::axum::Router::new()
            .route("/:id", get(Self::get_user))
            .with_state(ctrl.clone())
            .route("/", post(Self::act_user).get(Self::list_user))
            .with_state(ctrl.clone())
            .nest(
                "/verification",
                VerificationControllerV1::route(pool.clone())?,
            ))
    }

    pub async fn act_user(
        State(ctrl): State<UserControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<UserAction>,
    ) -> Result<JsonWithHeaders<User>> {
        tracing::debug!("act_user {:?}", body);
        // Ok(Json(User::default()))
        body.validate()?;

        match body {
            UserAction::Signup(params) => ctrl.signup(params).await,
            UserAction::Login(params) => ctrl.login(params).await,
            UserAction::Reset(params) => ctrl.reset(params).await,
        }
    }

    pub async fn get_user(
        State(_ctrl): State<UserControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(id): Path<String>,
    ) -> Result<Json<User>> {
        tracing::debug!("get_user {:?}", id);
        Ok(Json(User::default()))
    }

    pub async fn list_user(
        State(ctrl): State<UserControllerV1>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<UserParam>,
    ) -> Result<Json<UserGetResponse>> {
        tracing::debug!("list_user {:?}", q);

        match q {
            UserParam::Query(_params) => Ok(Json(UserGetResponse::Query(QueryResponse::default()))),
            UserParam::Read(action) => match action.action.unwrap() {
                UserReadActionType::Refresh => {
                    if auth.is_none() {
                        return Err(ApiError::Unauthorized);
                    }
                    ctrl.refresh_user(auth.unwrap()).await
                }
                _ => Err(ApiError::InvalidAction),
            },
        }
    }
}

impl UserControllerV1 {
    pub fn generate_token(&self, user: &User) -> Result<String> {
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

    pub async fn refresh_user(&self, auth: Authorization) -> Result<Json<UserGetResponse>> {
        match auth {
            Authorization::Bearer { claims } => {
                let user = self
                    .repo
                    .find_one(&UserReadAction::new().find_by_email(claims.custom["email"].clone()))
                    .await?;

                Ok(Json(UserGetResponse::Read(user)))
            }
            _ => Err(ApiError::Unauthorized),
        }
    }

    pub async fn verify_code(&self, email: String, code: String) -> Result<()> {
        let req = VerificationReadAction::new().get_verification_code(email.clone(), code.clone());
        let res = self.verification.find_one(&req).await?;

        if res.value != code
            || res.email != email
            || res.expired_at < chrono::Utc::now().timestamp()
        {
            tracing::error!(
                "Invalid verification code: {:?} at {}",
                res,
                chrono::Utc::now().timestamp()
            );
            return Err(ApiError::InvalidVerificationCode);
        } else {
            Ok(())
        }
    }

    pub async fn signup(&self, body: UserSignupRequest) -> Result<JsonWithHeaders<User>> {
        self.verify_code(body.email.clone(), body.code.clone())
            .await?;

        let pw = get_hash_string(body.password.as_bytes());

        let user = self
            .repo
            .insert(body.email.clone(), pw.clone())
            .await
            .map_err(|e| {
                tracing::error!("Failed to insert user: {}", e);
                ApiError::DuplicateUser
            })?;
        self.org
            .insert_with_dependency(user.id, user.clone().email)
            .await?;

        let user = self
            .repo
            .find_one(&UserReadAction::new().get_user(body.email, pw))
            .await?;

        let jwt = self.generate_token(&user)?;

        // TODO: check invitation table and add user to organization (groups)

        Ok(JsonWithHeaders::new(user)
            .with_bearer_token(&jwt)
            .with_cookie(&jwt))
    }

    pub async fn login(&self, body: UserLoginRequest) -> Result<JsonWithHeaders<User>> {
        let user = self
            .repo
            .find_one(&UserReadAction::new().get_user(
                body.email.clone(),
                get_hash_string(body.password.as_bytes()),
            ))
            .await
            .map_err(|e| {
                tracing::error!("Failed to find user: {}", e);
                ApiError::AuthKeyNotMatch("check your password".to_string())
            })?;

        let jwt = self.generate_token(&user)?;

        Ok(JsonWithHeaders::new(user)
            .with_bearer_token(&jwt)
            .with_cookie(&jwt))
    }

    pub async fn reset(&self, body: UserResetRequest) -> Result<JsonWithHeaders<User>> {
        self.verify_code(body.email.clone(), body.code.clone())
            .await?;

        // TODO: update password

        todo!()
    }
}
