use crate::api_objects::Level;
use crate::{
    api_objects::{Asks, Bids, Exchange, PairCurrencies},
    exchanges::data_structs::DepthStreamData,
};
use futures_util::StreamExt;

use tokio::sync::mpsc::Sender;
use tokio_tungstenite::{connect_async, tungstenite};
use url::Url;

static BINANCE_WS_API: &str = "wss://stream.binance.com:9443";
pub struct Binance {}

impl Binance {
    pub fn new() -> Self {
        Binance {}
    }

    pub fn decode_data(data: DepthStreamData, exchange: Exchange) -> (Asks, Bids) {
        let bids = data
            .bids
            .into_iter()
            .map(|offer| Level::from_offer_data(&exchange, offer))
            .collect();
        let asks = data
            .asks
            .into_iter()
            .map(|offer| Level::from_offer_data(&exchange, offer))
            .collect();
        (asks, bids)
    }

    pub async fn pull_orders(
        &self,
        pair_currencies: &PairCurrencies,
        tx: Sender<(Vec<Level>, Vec<Level>)>,
    ) -> () {
        let binance_url = format!("{}/ws/ethbtc@depth5@1000ms", BINANCE_WS_API);
        let (socket, response) = connect_async(Url::parse(&binance_url).unwrap())
            .await
            .expect("Can't connect.");
        println!("Connected to binance stream.");
        println!("HTTP status code: {}", response.status());
        println!("Response headers:");
        for (ref header, header_value) in response.headers() {
            println!("- {}: {:?}", header, header_value);
        }
        let (_, mut read_remote) = socket.split();

        tokio::spawn(async move {
            while let Some(message) = read_remote.next().await {
                let msg = match message {
                    Ok(tungstenite::Message::Text(s)) => s,
                    _ => {
                        panic!()
                    }
                };
                let parsed_data: DepthStreamData =
                    serde_json::from_str(&msg).expect("Unable to parse message");
                let data = Self::decode_data(parsed_data, Exchange::BINANCE);
                println!("data: {:?}", data);
                match tx.send(data).await {
                    Ok(_) => println!("data sent successfully"),
                    Err(e) => println!("error {:?}", e),
                };
            }
        });

        // tokio::spawn(async move {
        //     read_remote.for_each(|message| async {
        //         let msg = match message {
        //             Ok(tungstenite::Message::Text(s)) => s,
        //             _ => {
        //                 panic!()
        //             }
        //         };
        //         let parsed_data: DepthStreamData =
        //             serde_json::from_str(&msg).expect("Unable to parse message");
        //         let data = Self::decode_data(parsed_data, Exchange::BINANCE);
        //         println!("data: {:?}", data);

        //     })
        //     .await;
        // });
    }
}

pub struct Bitstamp {}

impl Bitstamp {
    pub fn new() -> Self {
        Bitstamp {}
    }
}
