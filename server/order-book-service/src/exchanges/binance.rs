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
use tokio::sync::mpsc::{Receiver, Sender};

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
        tx:Sender<(Vec<Level>, Vec<Level>)>
    ) ->(){
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
        // let (tx, mut rx) = mpsc::channel(4);

        let read = Mutex::new(read_remote);
        tokio::spawn(async move {
            while let Some(message) = read.lock().await.next().await{
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
                match tx.send(data).await{
                    Ok(_)=>println!("data sent successfully"),
                    Err(e)=>println!("error {:?}", e)
                };
            };
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
