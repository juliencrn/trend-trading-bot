use crate::models;
use models::kline::{Kline, PriceTicker};
use reqwest::{Client, StatusCode};

static BINANCE_URL: &str = "https://api.binance.com/api/v3";

pub async fn get_klines(
    client: Client,          // from reqwest HTTP lib
    interval: &str,          //1m 3m 5m 15m 30m 1h 2h 4h 6h 8h 12h 1d 3d 1w 1M
    symbol: &str,            // eg: BTCUSDT, ETHUSDT, ETHBTC
    limit: Option<u32>,      // Number to candles, default: 500, max 1k
    start_time: Option<i64>, // Timestamp
    end_time: Option<i64>,   // Timestamp
) -> Option<Vec<Kline>> {
    let limit = if limit.is_some() { limit } else { Some(1000) }.unwrap();

    let req_url = if start_time.is_some() & end_time.is_some() {
        format!(
            "{}/klines?symbol={}&interval={}&startTime={}&endTime={}&limit={}",
            BINANCE_URL,
            symbol,
            interval,
            start_time.unwrap(),
            end_time.unwrap(),
            limit
        )
    } else {
        format!(
            "{}/klines?symbol={}&interval={}&limit={}",
            BINANCE_URL, symbol, interval, limit
        )
    };

    println!("request url: {}", &req_url);

    let result = client.get(&req_url).send().await.unwrap();
    let data: Vec<Kline> = match result.status() {
        StatusCode::OK => {
            serde_json::from_value::<Vec<Kline>>(result.json().await.unwrap()).unwrap()
        }
        _ => {
            println!("StatusCode: {}", result.status());
            println!("Message: {:?}", result.text().await);
            return None;
        }
    };

    Some(data)
}

pub async fn get_price(client: Client, symbol: &str) -> Option<PriceTicker> {
    let req_url = format!("{}/ticker/price?symbol={}", BINANCE_URL, symbol);
    println!("request url: {}", &req_url);

    let result = client.get(&req_url).send().await.unwrap();
    let data: PriceTicker = match result.status() {
        StatusCode::OK => {
            serde_json::from_value::<PriceTicker>(result.json().await.unwrap()).unwrap()
        }
        _ => {
            println!("StatusCode: {}", result.status());
            println!("Message: {:?}", result.text().await);
            return None;
        }
    };

    Some(data)
}
