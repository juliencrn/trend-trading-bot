//! This script downloads 1 year of data in csv file.
//! This data will be used for back-test trading strategies.

use app::exchanges::binance;
use app::models::binance::Kline;
use app::utils;
use serde::Serialize;
use std::error::Error;

// Be careful
// - That operation will override the given file
// - The binance get kline call is limited at 1000 items

// represents start/end date-time as timestamps
struct TimeRange(i64, i64);

struct Config<'a> {
    interval: &'a str,
    symbol: &'a str,
    filename: &'a str,
    time_ranges: Vec<TimeRange>,
}

#[tokio::main]
async fn main() {
    // This example represents 1 years in 4h interval,
    // splitted in 4 * 3 months ranges because of the 1000 limit by api call.
    let config = Config {
        interval: "4h",
        symbol: "BTCUSDT",
        filename: "BTCUSDT-4h-2021.csv",
        time_ranges: vec![
            TimeRange(
                utils::to_timestamp(2021, 1, 1, 0, 0, 0),
                utils::to_timestamp(2021, 3, 31, 23, 59, 59),
            ),
            TimeRange(
                utils::to_timestamp(2021, 4, 1, 0, 0, 0),
                utils::to_timestamp(2021, 6, 30, 23, 59, 59),
            ),
            TimeRange(
                utils::to_timestamp(2021, 7, 1, 0, 0, 0),
                utils::to_timestamp(2021, 9, 30, 23, 59, 59),
            ),
            TimeRange(
                utils::to_timestamp(2021, 10, 1, 0, 0, 0),
                utils::to_timestamp(2021, 12, 31, 23, 59, 59),
            ),
        ],
    };

    match fetch_and_write_file::<Kline>(config).await {
        Ok(_) => println!("Success"),
        Err(e) => println!("There is an error: {}", e),
    };
}

// TODO: Refactor, it was Copied/pasted/updated from the csv mod
async fn fetch_and_write_file<'a, T>(config: Config<'a>) -> Result<(), Box<dyn Error>>
where
    T: Serialize,
{
    let path = format!("data/historical/{}", config.filename);
    let mut writer = csv::Writer::from_path(path).unwrap();

    for TimeRange(start_time, end_time) in config.time_ranges {
        let client = utils::get_client();
        let result: Vec<Kline> = binance::get_klines(
            client,
            config.interval,
            config.symbol,
            None,
            Some(start_time),
            Some(end_time),
        )
        .await
        .unwrap();

        println!("{:?} lines found", result.len());

        for line in result {
            writer.serialize(line)?;
        }
    }

    Ok(())
}
