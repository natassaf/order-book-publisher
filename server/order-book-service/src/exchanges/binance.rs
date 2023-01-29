use std::sync::Arc;

use futures_util::{stream::SplitStream, StreamExt};

use tokio::{
    net::TcpStream,
    sync::{mpsc::Sender, Mutex},
};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{self},
    MaybeTlsStream, WebSocketStream,
};
use url::Url;

use crate::api_objects::{Asks, Bids, Exchange, Level, PairCurrencies};

use super::{BinanceResponseData, BinanceSpeeds};

static BINANCE_WS_API: &str = "wss://stream.binance.com:9443";

#[derive(Clone)]
pub struct BinanceConnection {
    read_from_socket: Arc<Mutex<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
}

impl BinanceConnection {
    pub async fn new(endpoind_url: String) -> Self {
        let (socket, response) = connect_async(Url::parse(&endpoind_url).unwrap())
            .await
            .expect("Can't connect.");
        println!("Connected to binance stream.");
        println!("HTTP status code: {}", response.status());

        let (_write_to_socket, read_from_socket) = socket.split();
        BinanceConnection {
            read_from_socket: Arc::new(Mutex::new(read_from_socket)),
        }
    }

    pub fn compose_binance_depth_url(
        symbol: &PairCurrencies,
        levels: usize,
        speed: BinanceSpeeds,
    ) -> String {
        let symbol = match symbol {
            PairCurrencies::ETHBTC => "ethbtc",
            PairCurrencies::UNKNOWN(input_text) => {
                println!("This symbol is not explicitly handled for binance");
                &input_text
            }
        };

        let speed = match speed {
            BinanceSpeeds::HundredMill => 100,
            BinanceSpeeds::ThousandMill => 1000,
        };
        format!(
            "{}/ws/{}@depth{}@{}ms",
            BINANCE_WS_API, symbol, levels, speed
        )
    }

    pub fn decode_data(data: BinanceResponseData, exchange: Exchange) -> (Asks, Bids) {
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

    pub async fn pull_orders<'a>(self, tx: Sender<(Vec<Level>, Vec<Level>)>) -> () {
        tokio::spawn(async move {
            while let Some(message) = self.read_from_socket.lock().await.next().await {
                let msg = match message {
                    Ok(tungstenite::Message::Text(s)) => s,
                    _ => {
                        panic!("Unable to read data from binance")
                    }
                };
                let parsed_data: BinanceResponseData =
                    serde_json::from_str(&msg).expect("Unable to parse message from binance ");
                let data = Self::decode_data(parsed_data, Exchange::BINANCE);

                match tx.send(data).await {
                    Ok(_) => println!("data sent successfully"),
                    Err(e) => println!("error {:?}", e),
                };
            }
        });
    }
}

pub struct Bitstamp {}

impl Bitstamp {
    pub fn new() -> Self {
        Bitstamp {}
    }
}
