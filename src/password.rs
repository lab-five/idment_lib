use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordVerifier, Version, password_hash};

use crate::error::auth_error::AuthError;
use argon2::PasswordHasher;
use password_hash::{SaltString, rand_core::OsRng};

pub async fn argon_config() -> Result<Argon2<'static>, AuthError> {
    let cpus = num_cpus::get() as u32;
    let argon2 = Argon2::new(
        Algorithm::Argon2id, // 使用更安全的变体
        Version::V0x13,      // 使用最新版本
        Params::new(
            30000,    // 内存成本 19MB (单位: KB)
            3,        // 迭代次数 (推荐 2-3)
            cpus,     // 并行度 (推荐与 CPU 核心数一致)
            Some(32), // 哈希输出长度
        )
        .unwrap(),
    );
    Ok(argon2)
}

pub async fn hash_password(password: &str) -> Result<String, AuthError> {
    // 生成随机盐（推荐 16 字节）
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = argon_config().await?;
    // 生成哈希
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AuthError::HashPasswordError(e.to_string()))?;
    Ok(hash.to_string())
}

// 密码验证
pub async fn verify_password(password: &str, hash: &str) -> Result<bool, AuthError> {
    let parsed_hash =
        PasswordHash::new(hash).map_err(|e| AuthError::HashPasswordError(e.to_string()))?;
    let argon2 = argon_config().await?;
    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .map(|_| true)
        .map_err(|e| AuthError::VerifyPasswordError(e.to_string()))?)
}

// let input_password = "ssd";
// let sql_password = "ssd";
// let hash = hash_password(input_password).await.unwrap();

// let a = verify_password(sql_password, &hash).await.unwrap();
// println!("{:?}", a); // true or not
