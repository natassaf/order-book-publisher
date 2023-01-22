use crate::api_objects::{Asks, Bids, Exchange, Level, PairCurrencies, Spread, Summary};
use tonic::Status;

fn calculate_spread(highest_bid: &Level, lowest_ask: &Level) -> Spread {
    lowest_ask.price - highest_bid.price
}

fn pull_orders(pair_currencies: &PairCurrencies, exchange: &Exchange) -> (Asks, Bids) {
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

fn sort_levels<'a>(orders: &'a mut Vec<Level>, ascending: bool) -> &'a Vec<Level> {
    orders.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap());
    orders
}

fn merge_orders(orders1: &Vec<Level>, orders2: &Vec<Level>) -> Vec<Level> {
    [orders1.clone(), orders2.clone()].concat()
}

pub fn process(
    pair_currencies: PairCurrencies,
    exchange1: Exchange,
    exchange2: Exchange,
) -> Result<Summary, Status> {
    let (bid_orders_exch1, ask_orders_exch1) = pull_orders(&pair_currencies, &exchange1);
    let (bid_orders_exch2, ask_orders_exch2) = pull_orders(&pair_currencies, &exchange2);

    let mut merged_bids: Vec<Level> = merge_orders(&bid_orders_exch1, &bid_orders_exch2);
    let mut merged_asks: Vec<Level> = merge_orders(&ask_orders_exch1, &ask_orders_exch2);

    let sorted_bids: &Vec<Level> = sort_levels(&mut merged_bids, true);
    let sorted_asks: &Vec<Level> = sort_levels(&mut merged_asks, true);

    let spread = calculate_spread(sorted_bids.last().unwrap(), sorted_asks.first().unwrap());

    let summary = Summary::new(spread, sorted_bids.clone(), sorted_asks.clone());
    Ok(summary)
}
