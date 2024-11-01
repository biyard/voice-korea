#![allow(unused_imports)]
use dioxus::{
    prelude::{
        server_fn::codec::{GetUrl, Json, PostUrl},
        *,
    },
    Ok,
};
use dioxus_logger::tracing;
use serde::{Deserialize, Serialize};

use crate::{api::common::TypeField, models::user::User};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct VerifyEmailRequest {
    pub email: String,
    pub verification_code: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct VerifyEmailResponse {}

#[server(endpoint = "/v1/users/verify", input = Json, output = Json)]
pub async fn verify_email(req: VerifyEmailRequest) -> Result<(), ServerFnError> {
    use crate::api::aws::ses::AuthKeyModel;
    use easy_dynamodb::error::DynamoException;
    use std::result::Result::Ok;

    let log = crate::utils::logger::new_api("POST", &format!("/v1/users/signup"));
    let cli = crate::utils::db::get(&log);

    let user_model: Option<User> = cli.get(req.email.as_str()).await?;

    match user_model {
        Some(_user) => {}
        None => {
            return Err(ServerFnError::ServerError(format!("Email is not exists")));
        }
    };

    let auth_key_model: Result<(Option<Vec<AuthKeyModel>>, Option<String>), DynamoException> = cli
        .find(
            "auth-key-index",
            None,
            Some(1),
            vec![("auth_key", req.verification_code)],
        )
        .await;

    match auth_key_model {
        Ok(v) => {
            let auth_model_vec = &v.0.unwrap();

            if auth_model_vec.len() == 0 {
                return Err(ServerFnError::ServerError(format!(
                    "Auth key is not exists"
                )));
            }

            let auth_model = &auth_model_vec[0];

            let id = auth_model.id.clone();

            match cli
                .update::<TypeField>(id.as_str(), vec![("is_used", TypeField::N(2))])
                .await
            {
                Ok(_) => {
                    return Ok(());
                }
                Err(e) => {
                    return Err(ServerFnError::ServerError(format!("update failed {e}")));
                }
            }
        }
        Err(_e) => {
            return Err(ServerFnError::ServerError(format!(
                "Auth key is not exists"
            )));
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GetVerificationCodeRequest {
    pub email: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GetVerificationCodeResponse {}

#[server(endpoint = "/v1/users/verify", input = GetUrl, output = Json)]
pub async fn get_verification_code(
    req: GetVerificationCodeRequest,
) -> Result<GetVerificationCodeResponse, ServerFnError> {
    tracing::debug!("/v1/users/verify: {:?}", req);

    Err(ServerFnError::ServerError(
        "not implemented yet".to_string(),
    ))
}
