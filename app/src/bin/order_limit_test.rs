use app::exchanges::binance;
use app::utils;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let client = utils::get_client();
    binance::order_limit_test(client, "BTCUSDT", 0.001, 40_000.0).await?;

    Ok(())
}
