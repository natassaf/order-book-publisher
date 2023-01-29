use orderbook::orderbook_aggregator_server::{OrderbookAggregator, OrderbookAggregatorServer};
use std::{net::SocketAddr, sync::Arc};

use http::Method;
use tokio::{
    sync::{mpsc, Mutex},
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};
use tonic_web::GrpcWebLayer;
use tower_http::cors::{Any, CorsLayer};
use encoders_decoders::decode_pair_currencies;

use std::env;
pub mod api_objects;
mod encoders_decoders;
pub mod exchanges;
pub mod utils;

use api_objects::{PairCurrencies, Summary};

use crate::exchanges::{BinanceConnection, BitstampConnection};

mod book_summary_endpoint;

pub mod orderbook {
    tonic::include_proto!("orderbook");
}

#[derive(Clone, Debug, Default)]
pub struct MyOrderbookAggregator {}

impl MyOrderbookAggregator {
    pub fn new() -> Self {
        MyOrderbookAggregator {}
    }
}

async fn connect_to_bitstamp(pair_currencies: &PairCurrencies) -> Arc<Mutex<BitstampConnection>> {
    let data_channel = BitstampConnection::get_channel_name(&pair_currencies);
    Arc::new(Mutex::new(BitstampConnection::new(data_channel).await))
}

async fn connect_to_binance(pair_currencies: &PairCurrencies) -> Arc<Mutex<BinanceConnection>> {
    let url = BinanceConnection::compose_binance_depth_url(
        pair_currencies,
        20,
        crate::exchanges::BinanceSpeeds::HundredMill,
    );
    Arc::new(Mutex::new(BinanceConnection::new(url).await))
}

#[tonic::async_trait]
impl OrderbookAggregator for MyOrderbookAggregator {
    type BookSummaryStream = ReceiverStream<Result<Summary, Status>>;

    async fn book_summary(
        &self,
        request: Request<orderbook::BookSummaryRequest>,
    ) -> Result<tonic::Response<Self::BookSummaryStream>, tonic::Status> {
        let (tx, mut rx) = mpsc::channel(4); // exchange messages with fn process 
        let (tx_local, rx_local) = mpsc::channel(4); // prevents memory overflow due to sender sending more data than the receiver can take

        let pair_currencies = decode_pair_currencies(request);

        let exchange1_connection = connect_to_binance(&pair_currencies).await;
        let exchange2_connection = connect_to_bitstamp(&pair_currencies).await;

        tokio::spawn(async move {
            loop {
                book_summary_endpoint::process(
                    exchange1_connection.clone(),
                    exchange2_connection.clone(),
                    tx.clone(),
                )
                .await;
                let result = rx.recv().await.unwrap();
                tx_local.send(result).await.unwrap(); 
            }
        });

        Ok(Response::new(ReceiverStream::new(rx_local)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key = "TOKIO_WORKER_THREADS";
    env::set_var(key, "1");

    let addr: SocketAddr = "[::1]:14586".parse()?;
    let my_order_book_aggregator = OrderbookAggregatorServer::new(MyOrderbookAggregator::default());
    println!("OrderbookAggregatorServer running on {:?}", &addr);
    let cors = CorsLayer::new()
        .allow_headers(Any)
        .allow_methods([Method::POST])
        .allow_origin(Any); // gives permision to any IP to call this service

    Server::builder()
        .accept_http1(true)
        .layer(cors)
        .layer(GrpcWebLayer::new())
        .add_service(my_order_book_aggregator)
        .serve(addr)
        .await?;
    Ok(())
}
