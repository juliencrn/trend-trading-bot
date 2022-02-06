# Trend trading bot

Experiments repo about (crypto) trend trading. By "trend" I mean trading following the trend using technical indicators (vs other kinds of trading bots and strategies). I'm not quite sure about the destination of this project, maybe send signals on the Telegram channel, maybe draw charts, maybe trade. Let's start playing!

## About

This repository is a cargo workspace with some packages:

- `app`: The only one *binary* crate used at "entry point" and providing an access to the programme features (see the `bin` directory):
  
  ```sh
  # Launch the Telegram bot
  cargo run --bin telegram_bot

  # Fetch price on binance and return RSI, MACD...
  cargo run --bin ta_binance
  ```

- `ta`: Technical analysis library crate, calculates indicators like SMA, RSI, MACD...

You want to dive into? Run `cargo doc --open`.

## Telegram

To create a telegram bot, speak with [@BotFather](https://t.me/botfather) on Telegram.

## Environnement variables

Create a `.env` file at the root with:

```sh
TELOXIDE_TOKEN="insertValue" # Ask to @BotFather on Telegram
```
