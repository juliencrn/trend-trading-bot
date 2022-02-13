use app::exchanges::binance;
use app::utils;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let client = utils::get_client();
    binance::get_exchange_info(client).await?;

    Ok(())
}
