use crate::{
    error::jwt_error::JwtError,
    jwt::{claims::TokenClaims, config::JWT_CONFIG},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};

pub fn generate_access_token(
    user_id: &str,
    app_name: &str,
    device_id: i32,
    token_version: i32,
) -> Result<String, JwtError> {
    generate_token(user_id, app_name, device_id, "access", token_version)
}

pub fn generate_refresh_token(
    user_id: &str,
    app_name: &str,
    device_id: i32,
    token_version: i32,
) -> Result<String, JwtError> {
    generate_token(user_id, app_name, device_id, "access", token_version)
}

pub fn generate_token(
    user_id: &str,
    app_name: &str,
    device_id: i32,
    token_type: &str,
    token_version: i32,
) -> Result<String, JwtError> {
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
        aud: app_name.to_string(),
        device_id,
        token_type: token_type.into(),
        token_version,
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
