use std::net::Ipv4Addr;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct IpResult {
    ip: String,
}

pub async fn get_ip() -> Option<Ipv4Addr> {
    let ip: IpResult = serde_json::from_str(
        reqwest::get("https://api.ipify.org?format=json")
            .await
            .ok()?
            .text()
            .await
            .ok()?
            .as_str(),
    )
    .ok()?;
    ip.ip.parse().ok()
}
