use app::exchanges::binance::get_balance;
use app::utils;
use dotenv::dotenv;
use std::time::SystemTime;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let client = utils::get_client();
    let balance = get_balance(client, Some(SystemTime::now())).await;
    match balance {
        Ok(balance) => {
            println!("Can trade: {}", balance.can_trade);
            println!("Update time: {}", balance.update_time);
            println!("Balances: {:#?}", balance.get_no_empty_assets());
        }
        Err(error) => {
            println!("Something did wrong while fetching user balance: {}", error)
        }
    }

    Ok(())
}
