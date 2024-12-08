use std::sync::Arc;

use by_axum::axum::extract::State;
use by_axum::axum::Json;
use easy_dynamodb::Client;
use serde::Deserialize;

use crate::utils::error::ApiError;
use crate::utils::hash::get_hash_string;
use crate::utils::jwt::generate_jwt;

#[derive(Deserialize)]
pub struct LoginParams {
    email: String,
    password: String,
}

pub async fn handler(
    State(db): State<Arc<Client>>,
    Json(body): Json<LoginParams>,
) -> Result<String, ApiError> {
    let email = body.email.clone();
    let result: Result<
        (Option<Vec<models::User>>, Option<String>),
        easy_dynamodb::error::DynamoException,
    > = db
        .find(
            "gsi1-index",
            None,
            Some(1),
            vec![("gsi1", models::User::gsi1(body.email))],
        )
        .await;

    let (docs, _) = match result {
        Ok((Some(docs), Some(_))) => (docs, ()),
        _ => return Err(ApiError::InvalidCredentials(email)),
    };
    let user = match docs.first() {
        Some(user) => user,
        None => return Err(ApiError::InvalidCredentials(email)),
    };
    let hashed_password = get_hash_string(body.password.as_bytes());
    if user.password != hashed_password {
        return Err(ApiError::InvalidCredentials(email));
    }

    generate_jwt(&user.id, &user.email).map_err(|e| ApiError::JWTGenerationFail(e.to_string()))
}
