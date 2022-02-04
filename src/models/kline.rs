use serde::{de, Deserialize, Deserializer, Serialize};

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

pub fn str_to_float<'a, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'a>,
{
    let str_val = String::deserialize(deserializer)?;
    str_val.parse::<f64>().map_err(de::Error::custom)
}
