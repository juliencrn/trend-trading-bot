use chrono::NaiveDate;
use reqwest::Client;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

/// Create a `reqwest` Client
pub fn get_client() -> Client {
    let client = Client::builder()
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
