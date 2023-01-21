use crate::orderbook;
pub use orderbook::{Level, Summary};
pub type Spread=f64;
pub type Bids = Vec<Level>;
pub type Asks = Vec<Level>;

impl Summary {
    pub fn new(spread: Spread, bids: Vec<Level>, asks: Vec<Level>) -> Self {
        Summary { spread, bids, asks }
    }
}

impl Level{
    pub fn new(exchange:String, price:f64, amount:f64)->Self{
        Level { exchange, price, amount }
    }
}

pub enum Exchange {
    BINANCE,
    BITSTAMP
}

pub enum PairCurrencies{
    ETHBTC
}