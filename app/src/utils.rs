use chrono::NaiveDate;
use dotenv::dotenv;
use hex::encode as hex_encode;
use hmac::{Hmac, Mac, NewMac};
use reqwest::{header, Client};
use sha2::Sha256;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

/// Create a `reqwest` Client with default settings
pub fn get_client() -> Client {
    dotenv().ok();

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::HeaderName::from_static("x-mbx-apikey"),
        header::HeaderValue::from_str(&env::var("BINANCE_API_KEY").unwrap()).unwrap(),
    );

    println!("Request Headers: {:#?}", &headers);

    let client = Client::builder()
        .default_headers(headers)
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();

    client
}

/// Uppercase a string, useful for the symbols
pub fn to_uppercase(string: &str) -> String {
    string.chars().map(|c| c.to_ascii_uppercase()).collect()
}

/// Create a date from human entry and return its timestamp
pub fn to_timestamp(y: i32, m: u32, d: u32, h: u32, min: u32, s: u32) -> i64 {
    NaiveDate::from_ymd(y, m, d)
        .and_hms(h, min, s)
        .timestamp_millis()
}

/// Returns the timestamp for a given SystemTime
pub fn get_timestamp(time: SystemTime) -> u128 {
    let since_epoch = time.duration_since(UNIX_EPOCH).unwrap();
    since_epoch.as_millis()
}

/// Create signature from Hmac sha256 to auth on binance
pub fn get_signature(request: &str) -> String {
    type HmacSha256 = Hmac<Sha256>;
    let secret_key = env::var("BINANCE_SECRET_KEY").unwrap();
    let mut signed_key: HmacSha256 = NewMac::new_from_slice(secret_key.as_bytes()).unwrap();
    signed_key.update(request.as_bytes());
    let signature = hex_encode(signed_key.finalize().into_bytes());
    format!("{}", signature)
}
