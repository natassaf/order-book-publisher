use crate::api_objects::{Asks, Bids, Exchange, Level, PairCurrencies, Spread, Summary};
use tokio::time::{sleep, Duration};
use tonic::Status;
use crate::utils::round_to;


async fn calculate_spread(highest_bid: &Level, lowest_ask: &Level) -> Spread {
    lowest_ask.price - highest_bid.price
}

async fn pull_orders(pair_currencies: &PairCurrencies, exchange: &Exchange, num:usize) -> (Asks, Bids) {
    sleep(Duration::from_millis(500)).await;
    match exchange {
        Exchange::BINANCE => {
            let order1: Level = Level::new(Into::<String>::into(*exchange), 8491.25+ num as f64, 0.008 + num as f64);
            let order3: Level = Level::new(Into::<String>::into(*exchange), 8488.53+ num as f64, 0.002+ num as f64);

            let asks = vec![order1];
            let bids = vec![order3];
            (asks, bids)
        }
        Exchange::BITSTAMP => {
            let order2 = Level::new(Into::<String>::into(*exchange), 8496.37+ num as f64, 0.0303+ num as f64);
            let order4 = Level::new(Into::<String>::into(*exchange), 8484.71+ num as f64, 1.0959+ num as f64);
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
    num: usize,
) -> Result<Summary, Status> {
    let (ask_orders_exch1, bid_orders_exch1) = pull_orders(&pair_currencies, &exchange1, num).await;
    let (ask_orders_exch2, bid_orders_exch2) = pull_orders(&pair_currencies, &exchange2, num).await;

    let mut merged_bids: Vec<Level> = merge_orders(&bid_orders_exch1, &bid_orders_exch2).await;
    let mut merged_asks: Vec<Level> = merge_orders(&ask_orders_exch1, &ask_orders_exch2).await;

    let sorted_bids: &Vec<Level> = sort_levels(&mut merged_bids, false).await;
    let sorted_asks: &Vec<Level> = sort_levels(&mut merged_asks, false).await;
    let spread = calculate_spread(sorted_bids.first().unwrap(), sorted_asks.last().unwrap()).await;
    let spread = round_to(spread, 4);
    let summary = Summary::new(spread, sorted_bids.clone(), sorted_asks.clone());
    Ok(summary)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api_objects::Level;
    use crate::utils::round_to;

    #[tokio::test]
    async fn test_sort_levels() {
        let order1: Level = Level::new("binance".to_string(), 8491.25, 0.008);
        let order2 = Level::new("bitstamp".to_string(), 8496.37, 0.0303);
        let order3: Level = Level::new("binance".to_string(), 8488.53, 0.002);
        let order4 = Level::new("bitstamp".to_string(), 8484.71, 1.0959);

        // ascending order
        let mut asks = vec![order2.clone(), order1.clone()];
        let mut bids = vec![order3.clone(), order4.clone()];

        let sorted_asks = sort_levels(&mut asks, true).await;
        let sorted_bids = sort_levels(&mut bids, true).await;

        assert_eq!(*sorted_asks, vec![order1.clone(), order2.clone()]);
        assert_eq!(*sorted_bids, vec![order4.clone(), order3.clone()]);

        // descending order
        let mut asks = vec![order1.clone(), order2.clone()];
        let mut bids = vec![order4.clone(), order3.clone()];

        let sorted_asks = sort_levels(&mut asks, false).await;
        let sorted_bids = sort_levels(&mut bids, false).await;

        assert_eq!(*sorted_asks, vec![order2, order1]);
        assert_eq!(*sorted_bids, vec![order3, order4]);
    }

    #[tokio::test]
    async fn test_spread() {
        let order1: Level = Level::new("binance".to_string(), 8491.25, 0.008);
        let order2 = Level::new("bitstamp".to_string(), 8496.37, 0.0303);
        let order3: Level = Level::new("binance".to_string(), 8488.53, 0.002);
        let order4 = Level::new("bitstamp".to_string(), 8484.71, 1.0959);

        // ascending order
        let mut asks = vec![order1.clone(), order2.clone()];
        let mut bids = vec![order3.clone(), order4.clone()];

        let sorted_asks = sort_levels(&mut asks, true).await;
        let sorted_bids = sort_levels(&mut bids, true).await;

        let spread = calculate_spread(sorted_bids.last().unwrap(), sorted_asks.first().unwrap()).await;
        assert_eq!(2.72, round_to(spread, 2 as i32))
    }
}
