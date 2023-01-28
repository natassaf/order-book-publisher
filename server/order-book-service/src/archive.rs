


// use futures_util::{StreamExt, SinkExt};
// use serde_json::{json, Value};
// use tokio_tungstenite::{
//    connect_async, tungstenite::Message,
// };
// use url::Url;
// mod exchanges;

// use exchanges::BitstampResponse;

// #[tokio::main]
// async fn main() {
//     let (socket, _response) = connect_async(Url::parse("wss://ws.bitstamp.net").unwrap())
//         .await
//         .expect("Can't connect");

//     let url = "wss://ws.bitstamp.net";
//     let (ws_stream, _) = connect_async(url).await.unwrap();
//     let (mut write, mut read) = ws_stream.split();


//     write
//         .send(
//             Message::Text(
//                 json!({
//                     "event": "bts:subscribe",
//                     "data": {
//                         "channel": "order_book_ethbtc"
//                     }
//                 })
//                 .to_string(),
//             )
//             .into(),
//         )
//         .await
//         .unwrap();
//     read.next().await;
//     loop{
//         while let Some(message) = read.next().await {
//             let msg = match message {
//                 Ok(Message::Text(s)) => s,
//                 _ => {
//                     panic!()
//                 }
//             };
//             let parsed_data: BitstampResponse =
//                 serde_json::from_str(&msg).expect("Unable to parse message");
//             // let data = Self::decode_data(parsed_data, Exchange::BINANCE);
//             println!("data: {:?}", parsed_data);

//             };
//     }

// }
