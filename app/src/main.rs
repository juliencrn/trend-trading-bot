use ta;

mod exchanges;
mod models;
mod utils;

#[tokio::main]
async fn main() {
    // Get klines from binance
    let client = utils::get_client();
    let result = exchanges::binance::get_klines(client, "1d", "BTCUSDT", 100).await;
    let klines = match result {
        Some(data) => data,
        _ => panic!("Could fetch klines from Binance"),
    };
    println!("first result: {:#?}", klines[0]);
    println!("result count: {:?}", klines.len());

    // Extract close prices
    let price_data: Vec<f64> = klines.iter().rev().take(100).map(|f| f.close).collect();

    // Calc and print the Simple moving average (SMA)
    let sma = ta::sma(&price_data, 26).expect("Calculating SMA failed");
    println!("SMA: {:#?}", sma);

    // Calc and print the Exponential moving average (EMA)
    let ema = ta::ema(&price_data, 26).expect("Calculating EMA failed");
    println!("EMA: {:#?}", ema);

    // Calc and print the Moving Average Convergence Divergence (MACD)
    let macd = ta::macd(&price_data, 12, 26, 9).expect("Calculating MACD failed");
    println!("MACD: {:#?}", macd);
}
