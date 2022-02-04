use ta;

mod exchanges;
mod models;
mod utils;

#[tokio::main]
async fn main() {
    // Get klines from binance
    let client = utils::get_client();
    let kline_result = exchanges::binance::get_klines(client, "1d", "BTCUSDT", 100).await;
    let klines = match kline_result {
        Some(klines) => klines,
        _ => panic!("Something went wrong."),
    };

    println!("first result: {:#?}", klines[0]);
    println!("result count: {:?}", klines.len());

    // Calc and print the Simple moving average (SMA)
    let price_data: Vec<f64> = klines.iter().rev().take(100).map(|f| f.close).collect();
    let result = ta::sma(&price_data, 26);
    let sma_data = match result {
        Some(data) => data,
        _ => panic!("Calculating SMA failed"),
    };
    println!("SMA: {:?}", sma_data);
}
