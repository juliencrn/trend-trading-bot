use async_trait::async_trait;
use rand::Rng;

/// Struct for a individual bot which can be instantiated by the main.rs
/// with different market/trading strategy use cases
pub struct TradingBot {
    pub config: TradingConfig,
    pub market: Box<dyn Market>,
}

pub enum State {
    BUY,
    SELL,
}

pub struct TradingConfig {
    pub last_operation_price: f64,
    pub next_operation: State,
    pub upward_trend_threshold: f64,
    pub dip_threshold: f64,
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[async_trait]
pub trait Market {
    async fn get_balance(&self) -> Result<f64>;
    async fn get_market_price(&self) -> Result<f64>;
    async fn place_sell_order(&self, amount: f64) -> Result<f64>;
    async fn place_buy_order(&self, amount: f64) -> Result<f64>;
}

impl TradingBot {
    pub fn new(config: TradingConfig, market: Box<dyn Market>) -> Self {
        TradingBot { config, market }
    }

    // strategy: high sell, low buy
    pub async fn start(&mut self) -> Result<()> {
        let current_price = self.market.get_market_price().await?;

        println!("[PRICE] current market price: {:?} $", current_price);

        let percentage_diff = (current_price - self.config.last_operation_price)
            / self.config.last_operation_price
            * 100 as f64;

        println!("[PRICE] percentage_diff: {:?} $", percentage_diff);

        // based on operation state for the buy and sell action
        match self.config.next_operation {
            State::BUY => {
                self.config.last_operation_price = self.try_to_buy(percentage_diff).await?;
            }
            State::SELL => {
                self.config.last_operation_price = self.try_to_sell(percentage_diff).await?;
            }
        }

        Ok(())
    }

    async fn try_to_buy(&mut self, diff: f64) -> Result<f64> {
        let is_up = diff >= self.config.upward_trend_threshold;
        let is_dip = diff <= self.config.dip_threshold;

        if is_up || is_dip {
            let balance = self.market.get_balance().await?;

            println!("[BALANCE] current account balance {:?} $", balance);

            self.config.last_operation_price = self.market.place_buy_order(balance).await?;
            self.config.next_operation = State::SELL;

            println!(
                "[BUY] Bought x BTC for {:?} $",
                self.config.last_operation_price
            );
        }

        Ok(self.config.last_operation_price)
    }

    async fn try_to_sell(&mut self, diff: f64) -> Result<f64> {
        let is_up = diff <= self.config.upward_trend_threshold;
        let is_dip = diff >= self.config.dip_threshold;

        if is_up || is_dip {
            let balance = self.market.get_balance().await?;

            println!("[BALANCE] current account balance {:?} $", balance);

            self.config.last_operation_price = self.market.place_buy_order(balance).await?;
            self.config.next_operation = State::BUY;

            println!(
                "[SELL] Sold x BTC for {:?} $",
                self.config.last_operation_price
            );
        }

        Ok(self.config.last_operation_price)
    }
}

pub struct TestMarket {}

#[async_trait]
impl Market for TestMarket {
    async fn get_balance(&self) -> Result<f64> {
        // API call
        Ok(10.0)
    }

    async fn get_market_price(&self) -> Result<f64> {
        let mut rng = rand::thread_rng();
        Ok(rng.gen_range(0.0..10.0))
    }

    async fn place_sell_order(&self, amount: f64) -> Result<f64> {
        // API call
        Ok(amount)
    }

    async fn place_buy_order(&self, amount: f64) -> Result<f64> {
        // API call
        Ok(amount)
    }
}
