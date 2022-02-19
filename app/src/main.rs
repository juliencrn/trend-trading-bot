use app::trading_bot::{State, TestMarket, TradingBot, TradingConfig};
use chrono::{DateTime, Local};
use tokio::time::{self, Duration, Instant};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = TradingConfig {
        dip_threshold: 2.0,
        last_operation_price: 5.0,
        next_operation: State::SELL,
        upward_trend_threshold: 8.0,
    };
    let mut bot = TradingBot::new(config, Box::new(TestMarket {}));

    // set the interval for every 20s
    let mut interval = time::interval(Duration::from_secs(5));

    loop {
        // wait ine interval
        interval.tick().await;

        // trading start time
        let start = Instant::now();
        let now: DateTime<Local> = Local::now();

        // trading kick off
        println!("[TRADE] start at {:?} {:?}", now.date(), now.time());
        bot.start().await.unwrap();

        // trading end time
        let duration = start.elapsed();
        println!("[TRADE] end elapsed : {:?}", duration);
        println!("---");
    }
}
