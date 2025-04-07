use std::{error::Error, num::ParseIntError, time::Instant};

use chrono::Utc;
use mac_address::get_mac_address;

// 十六进制字符串转换为十进制无符号 64 位整数
pub async fn hex_to_decimal(hex_str: &str) -> Result<u64, ParseIntError> {
    let s = hex_str.trim_start_matches("0x").trim_start_matches("0X");
    u64::from_str_radix(s, 16)
}

// 获取物理地址
pub async fn mac_addr() -> Result<String, Box<dyn Error>> {
    let mac_address = get_mac_address().map_err(|e| format!("{}", e));

    let formatted_mac = &mac_address
        .expect("Failed to retrieve MAC address")
        .expect("MAC address is not available on this device")
        .bytes()
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<String>>()
        .join("");

    let decimal_mac = hex_to_decimal(&formatted_mac).await?;

    Ok(decimal_mac.to_string())
}

// 当前时间
pub async fn time() -> Result<String, Box<dyn Error>> {
    let current_time = Utc::now();

    let formatted_time = &current_time.format("%Y%m%d%H%M%S%6f");

    Ok(formatted_time.to_string())
}

// 测量 time  mac_addr 异步函数的顺序执行所需总时间，返回 纳秒 字符串
pub async fn elapsed() -> Result<String, Box<dyn Error>> {
    let start_time = Instant::now();
    time().await?;
    mac_addr().await?;
    let elapsed_time = Instant::now() - start_time;
    let elapsed_ns = &elapsed_time.as_nanos();
    Ok(elapsed_ns.to_string())
}

// 雪花朋克算法 20位 当下时间 + 15位 机器码 + 5位 函数处理完所需时间
pub async fn snowpunk() -> Result<u128, Box<dyn Error>> {
    let time_result = time().await?;
    let mac_addr_result = mac_addr().await?.replace([':', '-'], "");
    let elapsed_result = elapsed().await?;
    let combined_number = format!("{}{}{}", &time_result, &mac_addr_result, &elapsed_result);
    let filtered_number: String = combined_number
        .chars()
        .filter(|c| c.is_ascii_digit())
        .take(39)
        .collect();
    let parsed_number = filtered_number.parse::<u128>().unwrap_or(0);
    Ok(parsed_number)
}
