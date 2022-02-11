use serde::{de, Deserialize, Deserializer, Serialize};

/// Binance Kline; Used to download historical candles.
/// Note: Order matters, we receive an array of unnamed ordered fields.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Kline {
    pub open_time: i64,
    #[serde(deserialize_with = "str_to_float")]
    pub open: f64,
    #[serde(deserialize_with = "str_to_float")]
    pub high: f64,
    #[serde(deserialize_with = "str_to_float")]
    pub low: f64,
    #[serde(deserialize_with = "str_to_float")]
    pub close: f64,
    #[serde(deserialize_with = "str_to_float")]
    pub volume: f64,
    pub close_time: i64,
    #[serde(deserialize_with = "str_to_float")]
    pub quote_asset_volume: f64,
    pub number_of_trades: usize,
    #[serde(deserialize_with = "str_to_float")]
    pub take_buy_base_asset_volume: f64,
    #[serde(deserialize_with = "str_to_float")]
    pub take_buy_quote_asset_volume: f64,
    #[serde(deserialize_with = "str_to_float")]
    pub ignore: f64,
}

/// Price ticker from Binance
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PriceTicker {
    pub symbol: String,
    #[serde(deserialize_with = "str_to_float")]
    pub price: f64,
}

/// Save stream data from binance ws into rust data structures.
/// endpoint: /ws/<symbol>@kline_<interval>
#[derive(Debug, Deserialize)]
pub struct Candlestick {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "k")]
    pub kline: StreamKline,
}

/// Inner part of kline data from binance ws.
/// Note: There is more field returned by the socket not impl. below.
#[derive(Debug, Deserialize)]
pub struct StreamKline {
    // pair info
    #[serde(rename = "t")]
    pub open_time: i64,
    #[serde(rename = "T")]
    pub close_time: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub interval: String,

    // prices
    #[serde(rename = "o", deserialize_with = "str_to_float")]
    pub open: f64,
    #[serde(rename = "c", deserialize_with = "str_to_float")]
    pub close: f64,
    #[serde(rename = "h", deserialize_with = "str_to_float")]
    pub high: f64,
    #[serde(rename = "l", deserialize_with = "str_to_float")]
    pub low: f64,

    // volume
    #[serde(rename = "v", deserialize_with = "str_to_float")]
    pub volume: f64,
    #[serde(rename = "q", deserialize_with = "str_to_float")]
    pub quote_asset_volume: f64,
    #[serde(rename = "n")]
    pub number_of_trades: usize,
    #[serde(rename = "V", deserialize_with = "str_to_float")]
    pub take_buy_base_asset_volume: f64,
    #[serde(rename = "Q", deserialize_with = "str_to_float")]
    pub take_buy_quote_asset_volume: f64,
    #[serde(rename = "B", deserialize_with = "str_to_float")]
    pub ignore: f64,
}

pub fn str_to_float<'a, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'a>,
{
    let str_val = String::deserialize(deserializer)?;
    str_val.parse::<f64>().map_err(de::Error::custom)
}
