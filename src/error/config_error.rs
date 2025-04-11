#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    // 其他变体...
    #[error("Postgresql 地址配置错误: {0}")]
    PostgresqlConfig(String),
}
