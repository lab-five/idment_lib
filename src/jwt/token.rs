use crate::jwt::{claims::TokenClaims, config::JWT_CONFIG};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum JwtError {
    #[error("Invalid token")]
    InvalidToken,
    #[error("Expired token")]
    ExpiredToken,
    #[error("Token creation failed")]
    TokenCreationFailed,
    #[error("Token version mismatch")]
    TokenVersionMismatch,
    #[error("Other error: {0}")]
    Other(String),
}

pub fn generate_access_token(user_id: &str, token_version: i32) -> Result<String, JwtError> {
    generate_token(user_id, "access", token_version)
}

pub fn generate_refresh_token(user_id: &str, token_version: i32) -> Result<String, JwtError> {
    generate_token(user_id, "refresh", token_version)
}

fn generate_token(user_id: &str, token_type: &str, token_version: i32) -> Result<String, JwtError> {
    let now = Utc::now();
    let exp = match token_type {
        "access" => now + Duration::minutes(JWT_CONFIG.access_token_exp_minutes),
        "refresh" => now + Duration::days(JWT_CONFIG.refresh_token_exp_days),
        _ => return Err(JwtError::Other("Unknown token type".into())),
    };

    let claims = TokenClaims {
        sub: user_id.to_string(),
        exp: exp.timestamp() as usize,
        iat: now.timestamp() as usize,
        iss: JWT_CONFIG.issuer.clone(),
        aud: "idment_system".into(),
        token_type: token_type.into(),
        token_version, // 👈 新增
    };

    let secret = match token_type {
        "access" => &JWT_CONFIG.access_token_secret,
        "refresh" => &JWT_CONFIG.refresh_token_secret,
        _ => return Err(JwtError::InvalidToken),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| JwtError::TokenCreationFailed)
}

pub fn verify_token(
    token: &str,
    token_type: &str,
    expected_version: i32,
) -> Result<TokenClaims, JwtError> {
    let secret = match token_type {
        "access" => &JWT_CONFIG.access_token_secret,
        "refresh" => &JWT_CONFIG.refresh_token_secret,
        _ => return Err(JwtError::InvalidToken),
    };

    let token_data: TokenData<TokenClaims> = decode(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| {
        if e.to_string().contains("ExpiredSignature") {
            JwtError::ExpiredToken
        } else {
            JwtError::InvalidToken
        }
    })?;

    let claims = token_data.claims;

    if claims.token_type != token_type {
        return Err(JwtError::InvalidToken);
    }

    if claims.token_version != expected_version {
        return Err(JwtError::InvalidToken); // 可以自定义 JwtError::TokenVersionMismatch
    }

    Ok(claims)
}
