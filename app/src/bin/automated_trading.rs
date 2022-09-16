enum Trend {
    Up,
    None,
    Down,
}

trait TradingBot {
    // Get the price, calculate the trend and update the state

    fn get_available_funds() -> f64;

    fn get_trends(macd: f64) -> Trend {
        if macd >= 0.0 {
            Trend::Up
        } else {
            Trend::Down
        }
    }

    // Trade

    fn short(pair: &str, amount: f64); // or sell?

    fn long(pair: &str, amount: f64); // or buy?

    // Private functions - utilities
}

fn main() {
    println!("Let's started!");
    let macd = 34;

    loop {
        if macd >= 0 {
            println!("binance_api::buy()")
        } else {
            println!("binance_api::sell()")
        }

        break;
    }
}
