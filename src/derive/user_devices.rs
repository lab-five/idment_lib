use axum::http::Request;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use validator::Validate;
#[derive(Debug, Deserialize, Serialize, FromRow, Validate)]
pub struct UserDevices {
    #[validate(length(min = 3, max = 64))] // 示例验证规则
    pub user_id: String,

    #[validate(length(max = 128))]
    pub device_name: Option<String>,

    pub device_type: Option<String>,

    #[validate(ip)]
    pub ip_address: Option<String>,

    pub is_trusted: bool,
}
/// 获取 IP 地址
pub fn get_ip_from_request<B>(req: &Request<B>) -> Option<String> {
    req.headers()
        .get("x-real-ip")
        .or_else(|| req.headers().get("x-forwarded-for"))
        .or_else(|| req.headers().get("remote-addr"))
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}

/// 自动从请求构建设备信息
pub fn build_user_device(user_id: String) -> UserDevices {
    // let user_agent = req
    //     .headers()
    //     .get("user-agent")
    //     .and_then(|v| v.to_str().ok())
    //     .unwrap_or_default();

    // let parser = UserAgentParser::new().expect("Failed to init user-agent parser");
    // let agent = parser.parse(user_agent);

    let device_name = Some("default device name".to_string()); // e.g. "iPhone", "Windows"
    // let os_family = agent.os.family.to_lowercase();

    let device_type = Some("default device type".to_string());

    let ip_address = Some("127.0.0.1".to_string());

    UserDevices {
        user_id,
        device_name,
        device_type,
        ip_address,
        is_trusted: true,
    }
}
