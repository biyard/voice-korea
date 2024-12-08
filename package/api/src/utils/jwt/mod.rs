use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{
    decode, encode,
    errors::{Error as JwtError, ErrorKind},
    Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
    pub id: String, //user id
    pub email: String,
    pub exp: usize,
}

#[allow(dead_code)]
pub fn validate_jwt(token: &str) -> Result<Claims, JwtError> {
    let secret = match option_env!("JWT_SECRET") {
        Some(v) => v,
        _ => {
            return Err(JwtError::from(ErrorKind::InvalidRsaKey(
                "JWT_SECRET required".to_string(),
            )))
        }
    };
    let validation = Validation::new(Algorithm::HS256);
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
    Ok(token_data.claims)
}

pub fn generate_jwt(user_id: &str, email: &str) -> Result<String, JwtError> {
    let secret = match option_env!("JWT_SECRET") {
        Some(v) => v,
        _ => {
            return Err(JwtError::from(ErrorKind::InvalidRsaKey(
                "JWT_SECRET required".to_string(),
            )))
        }
    };

    let expiration = (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 60 * 60) as usize;

    let claims = Claims {
        id: user_id.to_string(),
        email: email.to_string(),
        exp: expiration,
    };

    let key = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;
    Ok(key)
}
