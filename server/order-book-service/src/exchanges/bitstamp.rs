pub struct Bitstamp {}

impl Bitstamp {
    pub fn new() -> Self {
        Bitstamp {}
    }

    pub fn decode_data(data: DepthStreamData, exchange: Exchange) -> (Asks, Bids){
        unimplemented();
    }

    
    pub async fn pull_orders(){

    }

}