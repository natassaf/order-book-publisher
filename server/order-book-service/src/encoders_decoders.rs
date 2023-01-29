use tonic::Request;

use crate::{api_objects::PairCurrencies, orderbook};

pub fn decode_pair_currencies(request: Request<orderbook::BookSummaryRequest>) -> PairCurrencies {
    let symbol = request.into_inner().symbol.to_lowercase();
    println!("symbol {:?}", symbol);
    if symbol == "ethbtc" {
        PairCurrencies::ETHBTC
    } else {
        PairCurrencies::UNKNOWN(symbol)
    }
}
