use by_axum::{
    axum::{
        body::Body,
        extract::Request,
        http::{header::AUTHORIZATION, Response},
        middleware::Next,
    },
    log::root,
};

use crate::utils::{error::ApiError, jwt::validate_jwt};

pub async fn authorization_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response<Body>, ApiError> {
    let auth_header = req.headers_mut().get(AUTHORIZATION);
    let auth_header = match auth_header {
        Some(header) => header.to_str().map_err(|_| {
            ApiError::InvalidCredentials("Please add the JWT token to the header".to_string())
        })?,
        None => {
            return Err(ApiError::InvalidCredentials(
                "Please add the JWT token to the header".to_string(),
            ));
        }
    };
    let mut header = auth_header.split_whitespace();
    let (_bearer, token) = (header.next(), header.next());
    let token_data = match validate_jwt(token.unwrap()) {
        Ok(data) => data,
        Err(_) => {
            return Err(ApiError::InvalidCredentials(
                "Unable to decode token".to_string(),
            ));
        }
    };
    slog::debug!(root(), "EXP: {}", token_data.exp);

    req.extensions_mut().insert(token_data);
    Ok(next.run(req).await)
}
