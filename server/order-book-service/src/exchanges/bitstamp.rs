use futures_channel::mpsc::UnboundedReceiver;
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio::sync::{futures, mpsc};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{self, Message},
};
use url::Url;

use tokio::sync::mpsc::Sender;

use crate::{
    api_objects::{Exchange, Level, PairCurrencies, Bids, Asks},
    exchanges::BitstampResponse,
};

pub struct Bitstamp {}

impl Bitstamp {
    pub fn new() -> Self {
        Bitstamp {}
    }

    pub fn decode_data(data: BitstampResponse, exchange: Exchange) -> (Asks, Bids) {
        let bids:Vec<Level> = data.data.bids.into_iter().map(|offer| Level::from_offer_data(&exchange, offer)).collect();
        let asks:Vec<Level> = data.data.asks.into_iter().map(|offer| Level::from_offer_data(&exchange, offer)).collect();
        (asks[0..10].into(), bids[0..10].into())
    }

    pub async fn pull_orders(
        &self,
        pair_currencies: &PairCurrencies,
        tx: Sender<(Vec<Level>, Vec<Level>)>,
    ) {
        let (socket, _response) = connect_async(Url::parse("wss://ws.bitstamp.net").unwrap())
            .await
            .expect("Can't connect");

        let url = "wss://ws.bitstamp.net";
        let (ws_stream, _) = connect_async(url).await.unwrap();
        let (mut write, mut read) = ws_stream.split();
        println!("pulling orders from bitstamp");
        write
            .send(
                Message::Text(
                    json!({
                        "event": "bts:subscribe",
                        "data": {
                            "channel": "order_book_ethbtc"
                        }
                    })
                    .to_string(),
                )
                .into(),
            )
            .await
            .unwrap();
        read.next().await;
        tokio::spawn(async move {
            while let Some(message) = read.next().await {
                let msg = match message {
                    Ok(Message::Text(s)) => s,
                    _ => {
                        panic!()
                    }
                };
                let parsed_data: BitstampResponse =
                    serde_json::from_str(&msg).expect("Unable to parse message");
                let data = Self::decode_data(parsed_data, Exchange::BITSTAMP);
                match tx.send(data).await {
                    Ok(_) => println!("data sent successfully bitstamp"),
                    Err(e) => println!("error {:?}", e),
                };
            }
        });
    }
}



