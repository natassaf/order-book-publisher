
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use futures_util::{future, pin_mut, StreamExt, stream::{SplitStream, SplitSink}};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use url::Url;


static BINANCE_WS_API: &str = "wss://stream.binance.com:9443";

#[tokio::main]
async fn main(){
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
    read_remote.for_each(|message| async {
        let data = message.unwrap().into_data();
        tokio::io::stdout().write_all(&data).await.unwrap();
    }).await;
}
