use crate::api_objects::{Asks, Bids, Exchange, Level, PairCurrencies, Spread, Summary};
use tonic::Status;
use tokio::time::{sleep, Duration};

async fn calculate_spread(highest_bid: &Level, lowest_ask: &Level) -> Spread {
    lowest_ask.price - highest_bid.price
}

async fn pull_orders(pair_currencies: &PairCurrencies, exchange: &Exchange) -> (Asks, Bids) {
    sleep(Duration::from_millis(3000)).await;
    match exchange {
        Exchange::BINANCE => {
            let order1: Level = Level::new(Into::<String>::into(*exchange), 8491.25, 0.008);
            let order3: Level = Level::new(Into::<String>::into(*exchange), 8488.53, 0.002);

            let asks = vec![order1];
            let bids = vec![order3];
            (asks, bids)
        }
        Exchange::BITSTAMP => {
            let order2 = Level::new(Into::<String>::into(*exchange), 8496.37, 0.0303);
            let order4 = Level::new(Into::<String>::into(*exchange), 8484.71, 1.0959);
            let asks = vec![order2];
            let bids = vec![order4];
            (asks, bids)
        }
    }
}

async fn sort_levels<'a>(orders: &'a mut Vec<Level>, ascending: bool) -> &'a Vec<Level> {
    if ascending {
        orders.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
    } else {
        orders.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
    }
    orders
}

async fn merge_orders(orders1: &Vec<Level>, orders2: &Vec<Level>) -> Vec<Level> {
    [orders1.clone(), orders2.clone()].concat()
}

pub async fn process(
    pair_currencies: PairCurrencies,
    exchange1: Exchange,
    exchange2: Exchange,
) -> Result<Summary, Status> {
    let (bid_orders_exch1, ask_orders_exch1) = pull_orders(&pair_currencies, &exchange1).await;
    let (bid_orders_exch2, ask_orders_exch2) = pull_orders(&pair_currencies, &exchange2).await;

    let mut merged_bids: Vec<Level> = merge_orders(&bid_orders_exch1, &bid_orders_exch2).await;
    let mut merged_asks: Vec<Level> = merge_orders(&ask_orders_exch1, &ask_orders_exch2).await;

    let sorted_bids: &Vec<Level> = sort_levels(&mut merged_bids, true).await;
    let sorted_asks: &Vec<Level> = sort_levels(&mut merged_asks, true).await;
    let spread = calculate_spread(sorted_bids.last().unwrap(), sorted_asks.first().unwrap()).await;

    let summary = Summary::new(spread, sorted_bids.clone(), sorted_asks.clone());
    Ok(summary)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api_objects::Level;

    fn round_to(num: f64, num_digits: i32) -> f64 
    {
        let multiplier:f64 = (10 as i64).pow(num_digits.try_into().unwrap()) as f64;
        ((num * multiplier).round()/multiplier) as f64

    }

    #[test]
    fn test_sort_levels() {
        let order1: Level = Level::new("binance".to_string(), 8491.25, 0.008);
        let order2 = Level::new("bitstamp".to_string(), 8496.37, 0.0303);
        let order3: Level = Level::new("binance".to_string(), 8488.53, 0.002);
        let order4 = Level::new("bitstamp".to_string(), 8484.71, 1.0959);

        // ascending order
        let mut asks = vec![order2.clone(), order1.clone()];
        let mut bids = vec![order3.clone(), order4.clone()];

        let sorted_asks = sort_levels(&mut asks, true);
        let sorted_bids = sort_levels(&mut bids, true);

        assert_eq!(*sorted_asks, vec![order1.clone(), order2.clone()]);
        assert_eq!(*sorted_bids, vec![order4.clone(), order3.clone()]);

        // descending order
        let mut asks = vec![order1.clone(), order2.clone()];
        let mut bids = vec![order4.clone(), order3.clone()];

        let sorted_asks = sort_levels(&mut asks, false);
        let sorted_bids = sort_levels(&mut bids, false);

        assert_eq!(*sorted_asks, vec![order2, order1]);
        assert_eq!(*sorted_bids, vec![order3, order4]);
    }

    #[test]
    fn test_spread() {
        let order1: Level = Level::new("binance".to_string(), 8491.25, 0.008);
        let order2 = Level::new("bitstamp".to_string(), 8496.37, 0.0303);
        let order3: Level = Level::new("binance".to_string(), 8488.53, 0.002);
        let order4 = Level::new("bitstamp".to_string(), 8484.71, 1.0959);

        // ascending order
        let mut asks = vec![order1.clone(), order2.clone()];
        let mut bids = vec![order3.clone(), order4.clone()];

        let sorted_asks = sort_levels(&mut asks, true);
        let sorted_bids = sort_levels(&mut bids, true);

        let spread = calculate_spread(sorted_bids.last().unwrap(), sorted_asks.first().unwrap());
        assert_eq!(2.72, round_to(spread,2 as i32))
    }
}
