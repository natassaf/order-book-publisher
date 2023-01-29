use std::sync::Arc;

use futures_util::{
    stream::{SplitStream},
    SinkExt, StreamExt,
};
use serde_json::json;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

use tokio::sync::mpsc::Sender;

use crate::{
    api_objects::{Asks, Bids, Exchange, Level, PairCurrencies},
    exchanges::BitstampResponseData,
};

#[derive(Clone)]
pub struct BitstampConnection {
    read_from_socket: Arc<Mutex<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>>,
}

impl BitstampConnection {
    pub async fn new(data_channel: String) -> Self {
        let exchange_url = "wss://ws.bitstamp.net";
        let (ws_stream, _) = connect_async(exchange_url).await.unwrap();
        let (mut write_to_socket, mut read_from_socket) = ws_stream.split();

        write_to_socket
            .send(
                Message::Text(
                    json!({
                        "event": "bts:subscribe",
                        "data": {
                            "channel": data_channel
                        }
                    })
                    .to_string(),
                )
                .into(),
            )
            .await
            .unwrap();
        println!(
            "establiched connection {}",
            read_from_socket.next().await.unwrap().unwrap()
        );

        BitstampConnection {
            read_from_socket: Arc::new(Mutex::new(read_from_socket)),
        }
    }

    pub fn get_channel_name(pair_currencies: &PairCurrencies) -> String {
        match pair_currencies {
            PairCurrencies::ETHBTC => "order_book_ethbtc".to_string(),
            PairCurrencies::UNKNOWN(input_text) => {
                println!("This symbol is not explicitly handled for bitstamp");
                format!("order_book_{}", input_text)
            }
        }
    }

    pub fn decode_data(data: BitstampResponseData, exchange: Exchange) -> (Asks, Bids) {
        let bids: Vec<Level> = data
            .data
            .bids
            .into_iter()
            .map(|offer| Level::from_offer_data(&exchange, offer))
            .collect();
        let asks: Vec<Level> = data
            .data
            .asks
            .into_iter()
            .map(|offer| Level::from_offer_data(&exchange, offer))
            .collect();
        (asks[0..10].into(), bids[0..10].into())
    }

    pub async fn pull_orders(self, tx: Sender<(Vec<Level>, Vec<Level>)>) {
        tokio::spawn(async move {
            while let Some(message) = self.read_from_socket.lock().await.next().await {
                let msg = match message {
                    Ok(Message::Text(s)) => s,
                    _ => {
                        panic!()
                    }
                };
                let parsed_data: BitstampResponseData =
                    serde_json::from_str(&msg).expect("Unable to parse message");
                let data = Self::decode_data(parsed_data, Exchange::BITSTAMP);

                match tx.send(data).await {
                    Ok(_) => println!("bitstamp data sent successfully"),
                    Err(e) => println!("error {:?}", e),
                };
            }
        });
    }
}
