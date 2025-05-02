use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::{
    error::jwt_error::JwtError,
    jwt::{claims::TokenClaims, config::JWT_CONFIG},
};

/// 仅解析 claims，不验证过期时间，签名校验可选（默认 false）
/// 用于：获取 user_id、token_type 等场景
pub fn decode_claims_unchecked(
    token: &str,
    token_type: &str,
    validate_signature: bool,
) -> Result<TokenClaims, JwtError> {
    let secret = match token_type {
        "access" => &JWT_CONFIG.access_token_secret,
        "refresh" => &JWT_CONFIG.refresh_token_secret,
        _ => return Err(JwtError::InvalidToken),
    };

    let mut validation = Validation::default();
    validation.validate_exp = false;

    if !validate_signature {
        validation.insecure_disable_signature_validation();
    }

    let data = decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map_err(|_| JwtError::InvalidToken)?;

    Ok(data.claims)
}

/// 校验 token 的有效性（签名、过期、token_type、token_version）
pub fn verify_token_with_version(
    token: &str,
    token_type: &str,
    expected_token_version: i32,
) -> Result<TokenClaims, JwtError> {
    let secret = match token_type {
        "access" => &JWT_CONFIG.access_token_secret,
        "refresh" => &JWT_CONFIG.refresh_token_secret,
        _ => return Err(JwtError::InvalidToken),
    };

    let mut validation = Validation::default();
    validation.validate_exp = true;

    let token_data = decode::<TokenClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
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

    if claims.token_version != expected_token_version {
        return Err(JwtError::InvalidToken);
    }

    Ok(claims)
}

// 一次性生成 access/refresh token 并包装为响应体
// pub fn generate_token_response(
//     user_id: &str,
//     app_name: &str,
//     token_version: i32,
// ) -> Result<TokenResponse, JwtError> {
//     let access_token = generate_access_token(user_id, app_name, token_version)?;
//     let refresh_token = generate_refresh_token(user_id, app_name, token_version)?;
//     let expires_in = (JWT_CONFIG.access_token_exp_minutes * 60) as usize;

//     Ok(TokenResponse {
//         access_token,
//         refresh_token,
//         expires_in,
//     })
// }
