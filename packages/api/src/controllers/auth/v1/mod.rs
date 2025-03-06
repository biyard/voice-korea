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
    pool: sqlx::Pool<sqlx::Postgres>,
    repo: UserRepository,
    org: OrganizationRepository,
    org_mem: OrganizationMemberRepository,
    group_mem: GroupMemberV2Repository,
    invite: InvitationRepository,
}

impl UserControllerV1 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = User::get_repository(pool.clone());
        let org = Organization::get_repository(pool.clone());
        let org_mem = OrganizationMember::get_repository(pool.clone());
        let group_mem: GroupMemberV2Repository = GroupMemberV2::get_repository(pool.clone());
        let invite = Invitation::get_repository(pool.clone());
        let ctrl = UserControllerV1 {
            pool: pool.clone(),
            repo,
            org,
            org_mem,
            group_mem,
            invite,
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
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<UserAction>,
    ) -> Result<JsonWithHeaders<User>> {
        tracing::debug!("act_user {:?}", body);
        // Ok(Json(User::default()))
        body.validate()?;

        match body {
            UserAction::Signup(params) => ctrl.signup(params).await,
            UserAction::Login(params) => ctrl.login(params).await,
            UserAction::Reset(params) => ctrl.reset(params).await,
            UserAction::UserSignup(params) => ctrl.user_signup(auth, params).await,
            UserAction::UserLogin(params) => ctrl.user_login(auth, params).await,
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

    pub async fn signup(
        &self,
        UserSignupRequest {
            email,
            password,
            code,
        }: UserSignupRequest,
    ) -> Result<JsonWithHeaders<User>> {
        let mut tx = self.pool.begin().await?;

        Verification::query_builder()
            .email_equals(email.clone())
            .value_equals(code.clone())
            .expired_at_greater_than_equals(chrono::Utc::now().timestamp())
            .query()
            .map(Verification::from)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| {
                tracing::error!("Verification Error: {:?}", e);
                ApiError::InvalidVerificationCode
            })?
            .ok_or(ApiError::InvalidVerificationCode)?;

        let pw = get_hash_string(password.as_bytes());

        let user = self
            .repo
            .insert_with_tx(&mut *tx, email.clone(), pw.clone())
            .await?
            .ok_or(ApiError::DuplicateUser)?;

        let org = self
            .org
            .insert_with_tx(&mut *tx, user.email.clone())
            .await?
            .ok_or(ApiError::DuplicateUser)?;

        self.org_mem
            .insert_with_tx(
                &mut *tx,
                user.id,
                org.id,
                user.email.clone(),
                Some(Role::Admin),
                None,
            )
            .await?
            .ok_or(ApiError::DuplicateUser)?;

        let user = User::query_builder()
            .id_equals(user.id)
            .query()
            .map(User::from)
            .fetch_optional(&mut *tx)
            .await?
            .ok_or(ApiError::DuplicateUser)?;

        tx.commit().await?;

        let jwt = self.generate_token(&user)?;

        self.invite_user(user.clone()).await?;

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

    pub async fn reset(
        &self,
        UserResetRequest {
            email,
            password,
            code,
        }: UserResetRequest,
    ) -> Result<JsonWithHeaders<User>> {
        let mut tx = self.pool.begin().await?;

        Verification::query_builder()
            .email_equals(email.clone())
            .value_equals(code.clone())
            .expired_at_greater_than_equals(chrono::Utc::now().timestamp())
            .query()
            .map(Verification::from)
            .fetch_optional(&mut *tx)
            .await?
            .ok_or(ApiError::InvalidVerificationCode)?;

        let pw = get_hash_string(password.as_bytes());

        sqlx::query("update users set password = $1 where email = $2")
            .bind(pw)
            .bind(&email)
            .execute(&mut *tx)
            .await?;

        let user = User::query_builder()
            .email_equals(email)
            .query()
            .map(User::from)
            .fetch_optional(&mut *tx)
            .await?
            .ok_or(ApiError::NotFound)?;

        tx.commit().await?;

        let jwt = self.generate_token(&user)?;

        Ok(JsonWithHeaders::new(user)
            .with_bearer_token(&jwt)
            .with_cookie(&jwt))
    }

    pub async fn user_login(
        &self,
        _auth: Option<Authorization>,
        body: UserUserLoginRequest,
    ) -> Result<JsonWithHeaders<User>> {
        // TODO: authorize token

        let user = self
            .repo
            .find_one(&UserReadAction::new().find_by_email(body.email))
            .await
            .map_err(|e| {
                tracing::error!("Failed to find user: {}", e);
                ApiError::AuthKeyNotMatch("check your email".to_string())
            })?;

        let jwt = self.generate_token(&user)?;

        Ok(JsonWithHeaders::new(user)
            .with_bearer_token(&jwt)
            .with_cookie(&jwt))
    }

    pub async fn user_signup(
        &self,
        _auth: Option<Authorization>,
        body: UserUserSignupRequest,
    ) -> Result<JsonWithHeaders<User>> {
        // TODO: authorize token

        let user = match self
            .repo
            .find_one(&UserReadAction::new().find_by_email(body.email.clone()))
            .await
        {
            //already exists => update
            Ok(v) => {
                let user = self
                    .repo
                    .update(
                        v.id,
                        UserRepositoryUpdateRequest {
                            email: Some(body.email.clone()),
                            password: Some(v.password),
                            nickname: body.nickname,
                        },
                    )
                    .await
                    .map_err(|e| {
                        tracing::error!("Failed to update user: {}", e);
                        ApiError::PutObjectFailed
                    })?;

                user
            }
            //not exists => insert
            Err(_) => {
                let user = self
                    .repo
                    .insert(body.email.clone(), "".to_string(), body.nickname)
                    .await
                    .map_err(|e| {
                        tracing::error!("Failed to insert user: {}", e);
                        ApiError::DuplicateUser
                    })?;

                let org = self.org.insert(user.email.clone()).await?;

                self.org_mem
                    .insert(user.id, org.id, user.email.clone(), Some(Role::Admin), None)
                    .await?;

                self.invite_user(user.clone()).await?;

                user
            }
        };

        let jwt = self.generate_token(&user)?;

        Ok(JsonWithHeaders::new(user)
            .with_bearer_token(&jwt)
            .with_cookie(&jwt))
    }

    async fn invite_user(&self, user: User) -> Result<()> {
        let query = InvitationSummary::base_sql_with("where email = $1");
        tracing::debug!("invite_user query: {}", query);

        match sqlx::query(&query)
            .bind(user.email.clone())
            .map(|r: sqlx::postgres::PgRow| r.into())
            .fetch_all(&self.pool)
            .await
        {
            Ok(invites) => {
                let invites: Vec<InvitationSummary> = invites;
                for i in invites.iter() {
                    let mut tx = self.pool.begin().await?;
                    match self
                        .org_mem
                        .insert_with_tx(
                            &mut *tx,
                            user.id,
                            i.org_id,
                            i.name.clone().unwrap_or(i.email.clone()),
                            i.role.clone(),
                            None,
                        )
                        .await
                    {
                        Ok(_) => {
                            if let Some(group_id) = i.group_id {
                                match self
                                    .group_mem
                                    .insert_with_tx(&mut *tx, group_id, user.id)
                                    .await
                                {
                                    Ok(_) => {
                                        self.invite.delete(i.id).await?;
                                        tx.commit().await?;
                                    }
                                    Err(e) => {
                                        tracing::error!("Failed to insert group member: {}", e);
                                        tx.rollback().await?;
                                    }
                                }
                            } else {
                                self.invite.delete(i.id).await?;
                                tx.commit().await?;
                            }
                        }
                        Err(e) => {
                            tracing::error!("Failed to insert org member: {}", e);
                            tx.rollback().await?;
                        }
                    }
                }
            }
            Err(_) => {}
        };
        Ok(())
    }
}
