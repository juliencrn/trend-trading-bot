use dotenv::dotenv;
use std::{env, error::Error};

use teloxide::prelude::*;
use teloxide::types::ParseMode::MarkdownV2;
use teloxide::utils::command::BotCommand;
use teloxide::utils::markdown::link;

use app::{exchanges, utils};

// Command examples

// /help
// /register
// /price BTC
// /price BTC USDT
// /price btc usdt
#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,

    #[command(description = "show a cryptocurrency price in USDT by default.")]
    Price(String),
}

/// Handle telegram answer using pattern matching on the Command enum.
async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => {
            log::info!("Ask for help");
            cx.answer(Command::descriptions()).await?
        }
        Command::Price(pair) => {
            log::info!("Ask for price for {}", pair);
            let mut pair_iter = pair.split_whitespace();

            if let Some(symbol_1) = pair_iter.next() {
                // If the 2nd symbol is missing, set "USDT" as default
                let symbol_2 = pair_iter.next().unwrap_or("usdt");
                let target = utils::to_uppercase(&format!("{}{}", &symbol_1, &symbol_2));
                let client = utils::get_client();
                let result = exchanges::binance::get_price(client, &target).await;

                match result {
                    Some(tick) => {
                        let response = format!("The price of {} is {}", target, tick.price);
                        cx.answer(response).await?
                    }
                    None => {
                        let market_link = link(
                            "https://www.binance.com/en/markets/coinInfo",
                            "binance market",
                        );
                        let message = format!(
                            "Could not fetch {} on Binance\\. Look in the {} if you can find it\\.",
                            target, market_link
                        );
                        cx.answer(message).parse_mode(MarkdownV2).send().await?
                    }
                }
            } else {
                let error = format!("Please add a symbol (eg: btc)");
                cx.answer(error).await?
            }
        }
    };

    Ok(())
}

/// Launch a server that's running the telegram bot backend.
async fn run() {
    dotenv().ok();

    teloxide::enable_logging!();
    log::info!("Starting bot...");

    let bot = Bot::from_env().auto_send();
    let bot_name = "Rust trend trading bot".to_string();

    teloxide::commands_repl(bot, bot_name, answer).await;
}

#[tokio::main]
async fn main() {
    run().await;
}
