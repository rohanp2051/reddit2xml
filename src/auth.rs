use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use rand::Rng;
use std::io;

const OAUTH_CLIENT_ID: &str = "ohXpoqrZYub1kg";
const TOKEN_URL: &str = "https://www.reddit.com/api/v1/access_token";
pub const USER_AGENT: &str = "Reddit/Version 2024.17.0/Build 1700000/Android 14";

fn generate_device_id() -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    (0..24)
        .map(|_| CHARSET[rng.gen_range(0..CHARSET.len())] as char)
        .collect()
}

pub fn get_access_token() -> Result<String, io::Error> {
    let credentials = format!("{OAUTH_CLIENT_ID}:");
    let encoded = BASE64.encode(credentials.as_bytes());

    let device_id = generate_device_id();
    let body = format!(
        "grant_type={}&device_id={}",
        "https%3A%2F%2Foauth.reddit.com%2Fgrants%2Finstalled_client", device_id
    );

    let resp = ureq::post(TOKEN_URL)
        .set("Authorization", &format!("Basic {encoded}"))
        .set("User-Agent", USER_AGENT)
        .set("Content-Type", "application/x-www-form-urlencoded")
        .send_string(&body)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    let json: serde_json::Value = resp
        .into_json()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

    json["access_token"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("no access_token in response: {json}"),
            )
        })
}
