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
    let close_prices: Vec<f64> = klines.iter().rev().take(100).map(|f| f.close).collect();

    // Calc and print the Simple moving average (SMA)
    let sma = ta::sma(&close_prices, 26).expect("Calculating SMA failed");
    println!("SMA: {:#?}", sma);

    // Calc and print the Exponential moving average (EMA)
    let ema = ta::ema(&close_prices, 26).expect("Calculating EMA failed");
    println!("EMA: {:#?}", ema);

    // Calc and print the Moving Average Convergence Divergence (MACD)
    let macd = ta::macd(&close_prices, 12, 26, 9).expect("Calculating MACD failed");
    println!("MACD: {:#?}", macd);

    // Calc and print the Bollinger Bands
    let typical_prices: Vec<f64> = klines
        .iter()
        .rev()
        .take(100)
        .map(|k| (k.high + k.low + k.close) / 3.0)
        .collect();
    let bolling =
        ta::bolling(&typical_prices, 20, 2.0).expect("Calculating Bollinger Bands failed");
    println!("Bollinger Bands: {:#?}", bolling);

    // Calc and print the Relative Strength Index (RSI)
    let rsi = ta::rsi(&close_prices, 14).expect("Calculating RSI failed");
    println!("RSI: {:#?}", rsi);
}
