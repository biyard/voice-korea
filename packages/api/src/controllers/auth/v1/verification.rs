use std::time::SystemTime;

use aws_sdk_sesv2::types::Content;
use by_axum::{
    auth::Authorization,
    axum::{extract::State, routing::post, Extension, Json},
};
use models::*;
use tracing::instrument;

use crate::utils::email::send_email;

#[derive(Clone, Debug)]
pub struct VerificationControllerV1 {
    repo: VerificationRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
    verification_expiration: i64,
}

impl VerificationControllerV1 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = Verification::get_repository(pool.clone());

        let ctrl = VerificationControllerV1 {
            pool,
            repo,
            verification_expiration: crate::config::get().verification_expiration,
        };

        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::act_verification))
            .with_state(ctrl.clone()))
    }

    pub async fn act_verification(
        State(ctrl): State<VerificationControllerV1>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<VerificationAction>,
    ) -> Result<Json<Verification>> {
        tracing::debug!("act_verification {:?}", body);

        match body {
            VerificationAction::SendVerificationCode(params) => {
                ctrl.send_verification_email(params).await
            }
            VerificationAction::Verify(params) => ctrl.verify(params).await,
        }
    }
}

impl VerificationControllerV1 {
    pub async fn verify(
        &self,
        VerificationVerifyRequest { email, value }: VerificationVerifyRequest,
    ) -> Result<Json<Verification>> {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let code = Verification::query_builder()
            .email_equals(email)
            .value_equals(value)
            .expired_at_greater_than(now)
            .query()
            .map(Verification::from)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("Verification Error: {:?}", e);
                ApiError::InvalidVerificationCode
            })?;

        Ok(Json(Verification {
            id: code.id,
            expired_at: code.expired_at,
            ..Verification::default()
        }))
    }

    #[instrument]
    pub async fn send_verification_email(
        &self,
        body: VerificationSendVerificationCodeRequest,
    ) -> Result<Json<Verification>> {
        use rand::distributions::Alphanumeric;
        use rand::{thread_rng, Rng};

        tracing::debug!("body {:?}", body);

        let code: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect();

        send_email(
            body.email.clone(),
            Content::builder()
                .data("인증번호 6자리입니다. 확인 후 3분 이내에 입력해주세요.")
                .build()
                .unwrap(),
            Content::builder()
                .data(format!("인증번호: {:?}", code))
                .build()
                .unwrap(),
        )
        .await
        .map_err(|e| {
            tracing::error!("Email Send Error: {:?}", e);
            ApiError::SESServiceError(e.to_string())
        })?;

        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let result = self
            .repo
            .insert(body.email, code, now + self.verification_expiration, 0)
            .await?;

        Ok(Json(Verification {
            id: result.id,
            expired_at: result.expired_at,
            ..Verification::default()
        }))
    }
}
