use crate::models;
use models::kline::{Kline, PriceTicker};
use reqwest::{Client, StatusCode};

static BINANCE_URL: &str = "https://api.binance.com/api/v3";

pub async fn get_klines(
    client: Client, // from reqwest HTTP lib
    interval: &str, //1m 3m 5m 15m 30m 1h 2h 4h 6h 8h 12h 1d 3d 1w 1M
    symbol: &str,   // eg: BTCUSDT, ETHUSDT, ETHBTC
    limit: u32,     // Number to candles, default: 500, max 1k
) -> Option<Vec<Kline>> {
    let req_url = format!(
        "{}/klines?symbol={}&interval={}&limit={}",
        BINANCE_URL, symbol, interval, limit
    );
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
