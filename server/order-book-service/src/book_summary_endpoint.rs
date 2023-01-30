use std::sync::Arc;

use crate::exchanges::BitstampConnection;
use crate::utils::round_to;
use crate::{
    api_objects::{Asks, Bids, Exchange, Level, Spread, Summary},
    exchanges::BinanceConnection,
};
use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, Mutex};
use tokio::time::{sleep, Duration};
use tonic::Status;

#[allow(dead_code)]
async fn get_lowest_asks_highest_bids_per_exchange_10(
    ask_orders_exch1: Asks,
    bid_orders_exch1: Bids,
    ask_orders_exch2: Asks,
    bid_orders_exch2: Bids,
) -> (Asks, Bids) {
    let num_asks1 = ask_orders_exch1.len();
    let top_10_ask_orders_exch1 =
        sort_levels(ask_orders_exch1, false).await[num_asks1 - 10..num_asks1].to_vec();

    let num_asks2 = ask_orders_exch2.len();
    let top_10_ask_orders_exch2 =
        sort_levels(ask_orders_exch2, false).await[num_asks2 - 10..num_asks2].to_vec();

    let top_10_bids_orders_exch1 = sort_levels(bid_orders_exch1, false).await[0..10].to_vec();

    let top_10_bids_orders_exch2 = sort_levels(bid_orders_exch2, false).await[0..10].to_vec();

    let merged_sorted_bids = {
        let merged_bids: Vec<Level> =
            merge_orders(&top_10_bids_orders_exch1, &top_10_bids_orders_exch2).await;
        let sorted_bids: Vec<Level> = sort_levels(merged_bids, false).await;
        sorted_bids
    };
    let merged_sorted_asks = {
        let merged_asks: Vec<Level> =
            merge_orders(&top_10_ask_orders_exch1, &top_10_ask_orders_exch2).await;
        let sorted_asks: Vec<Level> = sort_levels(merged_asks, false).await;
        sorted_asks
    };
    (merged_sorted_asks, merged_sorted_bids)
}

#[allow(dead_code)]
async fn pull_orders(
    exchange: &Exchange,
    num: usize,
) -> (Asks, Bids) {
    sleep(Duration::from_millis(500)).await;
    match exchange {
        Exchange::BINANCE => {
            let order1: Level = Level::new(
                Into::<String>::into(*exchange),
                8491.25 + num as f64,
                0.008 + num as f64,
            );
            let order3: Level = Level::new(
                Into::<String>::into(*exchange),
                8488.53 + num as f64,
                0.002 + num as f64,
            );

            let asks = vec![order1];
            let bids = vec![order3];
            (asks, bids)
        }
        Exchange::BITSTAMP => {
            let order2 = Level::new(
                Into::<String>::into(*exchange),
                8496.37 + num as f64,
                0.0303 + num as f64,
            );
            let order4 = Level::new(
                Into::<String>::into(*exchange),
                8484.71 + num as f64,
                1.0959 + num as f64,
            );
            let asks = vec![order2];
            let bids = vec![order4];
            (asks, bids)
        }
    }
}

async fn calculate_spread(highest_bid: &Level, lowest_ask: &Level) -> Spread {
    lowest_ask.price - highest_bid.price
}

async fn sort_levels(mut orders: Vec<Level>, ascending: bool) -> Vec<Level> {
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

async fn get_merged_sorted_orders(
    ask_orders_exch1: Asks,
    bid_orders_exch1: Bids,
    ask_orders_exch2: Asks,
    bid_orders_exch2: Bids,
    num_bids_to_return:usize,
    num_asks_to_return:usize
) -> (Asks, Bids) {
    let merged_sorted_bids = {
        let merged_bids: Vec<Level> = merge_orders(&bid_orders_exch1, &bid_orders_exch2).await;
        let sorted_bids: Vec<Level> = sort_levels(merged_bids, false).await;
        let num_bids = sorted_bids.len();

        let lowest = if num_bids_to_return < num_bids{
            sorted_bids[(num_bids-num_bids_to_return)..num_bids].to_vec()
        }
        else{
            sorted_bids
        };
        lowest
    };
    let merged_sorted_asks = {
        let merged_asks: Vec<Level> = merge_orders(&ask_orders_exch1, &ask_orders_exch2).await;
        let sorted_asks: Vec<Level> = sort_levels(merged_asks, false).await;
        let hightest = sorted_asks[0..num_asks_to_return].to_vec();
        hightest
    };

    (merged_sorted_asks, merged_sorted_bids)
}

pub async fn process<'a>(
    exchange1_connection: Arc<Mutex<BinanceConnection>>,
    exchange2_connection: Arc<Mutex<BitstampConnection>>,
    tx: Sender<Result<Summary, Status>>,
) {
    let (tx1, mut rx1) = mpsc::channel(4);
    let (tx2, mut rx2) = mpsc::channel(4);
    tokio::spawn(async move {
        loop {
            exchange1_connection
                .lock()
                .await
                .clone()
                .pull_orders(tx1.clone())
                .await;
            exchange2_connection
                .lock()
                .await
                .clone()
                .pull_orders(tx2.clone())
                .await;

            let (ask_orders_exch1, bid_orders_exch1): (Asks, Bids) = rx1.recv().await.unwrap();
            let (ask_orders_exch2, bid_orders_exch2): (Asks, Bids) = rx2.recv().await.unwrap();

            // get the 10 highest bids and 10 lowest asks in descending order
            let num_bids_to_return = 10;
            let num_asks_to_return = 10;
            let (merged_sorted_asks, merged_sorted_bids) = get_merged_sorted_orders(
                ask_orders_exch1,
                bid_orders_exch1,
                ask_orders_exch2,
                bid_orders_exch2,
                num_bids_to_return,
                num_asks_to_return
            ).await;
            let highest_bid = merged_sorted_bids.first().unwrap();
            let lowest_ask = merged_sorted_asks.last().unwrap();

            let spread = calculate_spread(highest_bid, lowest_ask).await;
            let spread = round_to(spread, 8);
            // println!("spread {:?}, {:?}, {:?}", spread, lowest_ask, highest_bid);
            let summary = Summary::new(
                spread,
                merged_sorted_bids,
                merged_sorted_asks,
            );
            let _res = tx.send(Ok(summary)).await;
        }
    });
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
        let asks = vec![order2.clone(), order1.clone()];
        let bids = vec![order3.clone(), order4.clone()];

        let sorted_asks = sort_levels(asks, true).await;
        let sorted_bids = sort_levels(bids, true).await;

        assert_eq!(*sorted_asks, vec![order1.clone(), order2.clone()]);
        assert_eq!(*sorted_bids, vec![order4.clone(), order3.clone()]);

        // descending order
        let asks = vec![order1.clone(), order2.clone()];
        let bids = vec![order4.clone(), order3.clone()];

        let sorted_asks = sort_levels(asks, false).await;
        let sorted_bids = sort_levels(bids, false).await;

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
        let asks = vec![order1.clone(), order2.clone()];
        let bids = vec![order3.clone(), order4.clone()];

        let sorted_asks = sort_levels(asks, true).await;
        let sorted_bids = sort_levels(bids, true).await;

        let spread =
            calculate_spread(sorted_bids.last().unwrap(), sorted_asks.first().unwrap()).await;
        assert_eq!(2.72, round_to(spread, 2 as i32))
    }

    #[tokio::test]
    async fn test_get_merged_sorted_orders() {
        let order1: Level = Level::new("binance".to_string(), 8491.25, 0.008);
        let order2 = Level::new("bitstamp".to_string(), 8496.37, 0.0303);
        let order3: Level = Level::new("binance".to_string(), 8488.53, 0.002);
        let order4 = Level::new("bitstamp".to_string(), 8484.71, 1.0959);

        // ascending order
        let _asks = vec![order1.clone(), order2.clone()];
        let _bids = vec![order3.clone(), order4.clone()];

        let (merged_sorted_asks, merged_sorted_bids) = get_merged_sorted_orders(vec![order1], vec![order3], vec![order2], vec![order4], 2, 2).await;
        let highest_bid = merged_sorted_bids.first().unwrap();
        let lowest_ask = merged_sorted_asks.last().unwrap();

        let spread = calculate_spread(highest_bid, lowest_ask).await;
        assert_eq!(2.72, round_to(spread, 2 as i32))
    }
}
