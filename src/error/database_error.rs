#[derive(thiserror::Error, Debug)]
pub enum DatabaseError {
    #[error("数据库连接错误: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("数据库迁移错误: {0}")]
    MigrationError(#[from] sqlx::migrate::MigrateError),

    #[error("Application Not Found")]
    ApplicationNotFound,
}
