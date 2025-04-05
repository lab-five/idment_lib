use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Token验证失败: {0}")]
    ValidationError(String),

    #[error("Token生成失败: {0}")]
    GenerationError(String),

    #[error("配置加载失败: {0}")]
    ConfigError(String),

    #[error("密钥错误: {0}")]
    KeyError(String),

    #[error("无效的签名算法")]
    InvalidAlgorithm,
}
