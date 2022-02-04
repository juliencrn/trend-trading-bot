mod exchanges;
mod models;
mod utils;

#[tokio::main]
async fn main() {
    let client = utils::get_client();
    let result = exchanges::binance::get_klines(client, "1d", "BTCUSDT", 100).await;

    let klines = match result {
        Some(klines) => klines,
        _ => panic!("Something went wrong."),
    };

    println!("first result: {:#?}", klines[0]);
    println!("result count: {:?}", klines.len());
}
