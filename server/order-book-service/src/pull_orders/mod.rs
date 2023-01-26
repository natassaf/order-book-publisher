use crate::api_objects::PairCurrencies;
use tokio::{net::TcpStream, sync::{MutexGuard, Mutex}};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use futures_util::{future, pin_mut, StreamExt, stream::{SplitStream, SplitSink}};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use tonic::async_trait;
use url::Url;

use self::traits::OrdersPuller;

pub mod traits;

static BINANCE_WS_API: &str = "wss://stream.binance.com:9443";
pub struct Binance {}

impl Binance {
    pub fn new() -> Self {
        Binance {}
    }

    pub async fn establish_connection(&self) -> (SplitSink<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>, tokio_tungstenite::tungstenite::Message>, SplitStream<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>) {
        let binance_url = format!("{}/ws/ethbtc@depth5@100ms", BINANCE_WS_API);
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
        (write_remote, read_remote) 
    }
}

// #[async_trait]
// impl OrdersPuller for Binance {
//     async fn pull_orders(&self, pair_currencies: PairCurrencies, socket:Mutex<WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>) {
//         // let socket = socket.lock().await;
//         // let s = MutexGuard::map(socket, |s| s.get_mut().write_message());
//         // // let (mut write, read) = s.split();
//         pull_orde

//     }
// }
