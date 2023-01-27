use crate::api_objects::Level;
use crate::{
    api_objects::{Asks, Bids, Exchange, PairCurrencies},
    exchanges::data_structs::DepthStreamData,
};
use futures_util::{
    future, pin_mut,
    stream::{SplitSink, SplitStream},
    Future, StreamExt,
};
use tokio::io::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::{
    net::TcpStream,
    sync::{mpsc, Mutex, MutexGuard},
};
use tokio_tungstenite::{connect_async, tungstenite, MaybeTlsStream, WebSocketStream};
use tonic::async_trait;
use url::Url;

use serde_json::Value;

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
    ) -> Result<tokio::sync::mpsc::Receiver<(Vec<Level>, Vec<Level>)>, Error> {
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
        let (write_remote, read_remote) = socket.split();

        let (tx, mut rx) = mpsc::channel(4);


        tokio::spawn(async move {
            read_remote.for_each(|message| async {
                let msg = match message {
                    Ok(tungstenite::Message::Text(s)) => s,
                    _ => {
                        panic!()
                    }
                };
                // tokio::io::stdout().write_all(&data).await.unwrap();
                let parsed_data: DepthStreamData =
                    serde_json::from_str(&msg).expect("Unable to parse message");
                let data = Self::decode_data(parsed_data, Exchange::BINANCE);
                tx.send(data).await.unwrap();
            })
            .await;
        });
        
        Ok(rx)
    }
}

pub struct Bitstamp {}

impl Bitstamp {
    pub fn new() -> Self {
        Bitstamp {}
    }
}
