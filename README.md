# Trend trading bot

Experiments repo about (crypto) trend-following trading. I'm not quite sure about the destination of this project, maybe send signals on the Telegram channel, maybe draw charts, maybe automated trading. Let's start playing!

**Disclaimer**:

- I'm new in blockchain
- I'm new in backend
- I'm new in Rust
- I'm new in trading

Don't trust my code here, it's really a work-in-progress sandbox!

## About

This repository is a cargo workspace with some packages:

- `app`: The only one *binary* crate used at "entry point" and providing an access to the programme features (see the `bin` directory):
  
  ```sh
  # Launch the Telegram bot
  cargo run --bin telegram_bot

  # Fetch price on binance and return RSI, MACD...
  cargo run --bin ta_binance

  # test read/write vec<T> in a CSV file
  cargo run --bin csv

  # Script to download historical data into csv files
  # Requires config in the file before
  cargo run --bin get_historical

  # Listen market update by connecting the binance API using web-socket
  cargo run --bin listen_binance_ws

  # Binance API functions
  cargo run --bin get_balance
  cargo run --bin get_btc_price
  cargo run --bin exchange_info
  cargo run --bin order_limit_test
  ```

- `ta`: Technical analysis library crate, calculates indicators like SMA, RSI, MACD...

You want to dive into? Run `cargo doc --open`.

## Telegram

To create a telegram bot, speak with [@BotFather](https://t.me/botfather) on Telegram.

## Environnement variables

Create a `.env` file at the root with:

```sh
TELOXIDE_TOKEN="insertValue" # Ask to @BotFather on Telegram

# https://binance-docs.github.io/apidocs/spot/en/#introduction
BINANCE_API_KEY="insertValue"
BINANCE_SECRET_KEY="insertValue"
```
