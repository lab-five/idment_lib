use chrono::Utc;
use chrono_tz::Asia::Shanghai;

pub struct TimeFormatter {
    // 可以添加其他字段，例如时区等
}

impl TimeFormatter {
    pub async fn get_time(&self) -> String {
        let tz = Shanghai;
        let current_time = chrono::Utc::now().with_timezone(&tz);
        current_time.format("%Y%m%d%H%M%S%6f").to_string()
    }

    pub async fn time() -> String {
        let tz = Shanghai;
        let current_time = Utc::now().with_timezone(&tz);

        let formatted_time = &current_time.format("%Y%m%d%H%M%S%6f");

        formatted_time.to_string()
    }
}
