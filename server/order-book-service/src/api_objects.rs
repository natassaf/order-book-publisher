use std::{error::Error, io::{self, ErrorKind}};

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

#[derive(Copy, Clone)]
pub enum Exchange {
    BINANCE,
    BITSTAMP,
}

impl From<String> for Exchange{
    fn from(item:String)->Exchange{
        let item = item.to_lowercase();
        if item=="binance"{
            Exchange::BINANCE
        }
        else if item == "bitstamp"{
            Exchange::BITSTAMP
        }
        else{
            panic!("Cannot find exchange")
        }
    }
}

impl Into<String> for Exchange{

    fn into(self)->String{
        match self{
            Exchange::BINANCE=>"binance".to_string(),
            Exchange::BITSTAMP=>"bitstamp".to_string()
        }
    }

}

pub enum PairCurrencies{
    ETHBTC
}