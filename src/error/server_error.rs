#[derive(thiserror::Error, Debug)]
pub enum ServereError {
    #[error("TCP Server Error")]
    TcpServerError(String),
}

// #[error("服务启动错误: {0}")]
// ServerStart(#[from] axum::Error),
