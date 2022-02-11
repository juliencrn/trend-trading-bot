use app::exchanges;
use app::models::binance::PriceTicker;
use app::utils;

#[tokio::main]
async fn main() {
    let client = utils::get_client();
    let result = exchanges::binance::get_price(client, "BTCUSDT").await;
    let price_ticker: PriceTicker = match result {
        Some(data) => data,
        _ => panic!("Could not fetch price ticker from Binance"),
    };

    println!("{} price is {}.", price_ticker.symbol, price_ticker.price);
}
