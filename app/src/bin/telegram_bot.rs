use dotenv::dotenv;
use std::{env, error::Error};

use teloxide::prelude::*;
use teloxide::utils::command::BotCommand;

use app::utils::to_uppercase;

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
        Command::Help => cx.answer(Command::descriptions()).await?,
        Command::Price(pair) => {
            let mut pair_iter = pair.split_whitespace();

            if let Some(symbol_1) = pair_iter.next() {
                // If the 2nd symbol is missing, set "USDT" as default
                let symbol_2 = pair_iter.next().unwrap_or("usdt");
                let target = to_uppercase(&format!("{}{}", &symbol_1, &symbol_2));
                let response = format!("You ask price for pair: {}.", target);
                cx.answer(response).await?
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
