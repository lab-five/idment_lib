use mac_address::get_mac_address;

pub struct MacAddress;

impl MacAddress {
    pub async fn get_mac(&self) -> String {
        match get_mac_address() {
            Ok(Some(mac)) => mac
                .bytes()
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<String>>()
                .join(""),
            Ok(None) => "No MAC address found".to_string(),
            Err(e) => format!("Failed to retrieve MAC address: {}", e),
        }
    }

    pub async fn mac_addr() -> String {
        let formatted_mac = get_mac_address()
            .unwrap()
            .expect("Failed to retrieve MAC address")
            .bytes()
            .iter()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<String>>()
            .join("");
        formatted_mac
    }
}
