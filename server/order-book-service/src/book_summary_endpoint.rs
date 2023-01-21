use crate::api_objects::{Summary, Exchange, PairCurrencies, Level, Spread, Bids, Asks};
use tonic::{Status};

fn calculate_spread(highest_bid:&Level, lowest_ask:&Level)->Spread{
    lowest_ask.price - highest_bid.price
}

fn pull_orders(pair_currencies:&PairCurrencies, exchange1:&Exchange)-> (Asks, Bids){
    let asks= (0..10).map(|i| Level::new("Binance".to_string(), i as f64, i as f64)).collect();
    let bids = (0..10).map(|i| Level::new("Binance".to_string(), i as f64, i as f64)).collect();
    (asks, bids)
}

fn sort_levels(orders:Vec<Level>, ascending:bool)->Vec<Level>{
    unimplemented!()
}

fn merge_orders(orders1:Vec<Level>, orders2:Vec<Level>)->Vec<Level>{
    unimplemented!()
}

pub fn process(pair_currencies:PairCurrencies, exchange1:Exchange, exchange2:Exchange) -> Result<Summary, Status> {
    let summary = Summary::new(0.0, vec![], vec![]);

    let (bid_orders_exch1, ask_orders_exch1) = pull_orders(&pair_currencies, &exchange1);
    let (bid_orders_exch2, ask_orders_exch2) = pull_orders(&pair_currencies, &exchange2);

    let merged_bids:Vec<Level> = merge_orders(bid_orders_exch1, bid_orders_exch2);
    let merged_asks:Vec<Level> = merge_orders(ask_orders_exch1, ask_orders_exch2);
    let sorted_bids:Vec<Level> = sort_levels(merged_bids, true);
    let sorted_asks:Vec<Level> = sort_levels(merged_asks, true);

    let spread = calculate_spread(sorted_bids.last().unwrap(), sorted_asks.first().unwrap());
   

    let summary = Summary::new(spread, sorted_bids, sorted_asks);
    Ok(summary)
}
