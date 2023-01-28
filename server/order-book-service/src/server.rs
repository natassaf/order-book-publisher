use orderbook::orderbook_aggregator_server::{OrderbookAggregator, OrderbookAggregatorServer};
use std::{sync::{Arc}, net::SocketAddr, time::Duration};

use tokio::{sync::{mpsc, Mutex}, time::sleep};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};
use tonic_web::GrpcWebLayer;
use tower_http::cors::{Any, CorsLayer};
use http::Method;

use std::env;
pub mod api_objects;
pub mod utils;
pub mod exchanges;


use api_objects::{Summary, Exchange, PairCurrencies};



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

#[tonic::async_trait]
impl OrderbookAggregator for MyOrderbookAggregator {
    type BookSummaryStream = ReceiverStream<Result<Summary, Status>>;

    async fn book_summary(
        &self,
        request: Request<orderbook::Empty>,
    ) -> Result<tonic::Response<Self::BookSummaryStream>, tonic::Status> {

        println!("Running book_summsry");
        let (tx, mut rx) = mpsc::channel(4);
        let (tx_local, rx_local) = mpsc::channel(4);
        tokio::spawn(async move {
            loop{
                book_summary_endpoint::process(PairCurrencies::ETHBTC, Exchange::BINANCE, Exchange::BITSTAMP, tx.clone()).await;
                let result = rx.recv().await.unwrap();
                // println!("{:?}",i);
                // println!("result: {:?}", result.clone().unwrap());
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

    let addr:SocketAddr = "[::1]:14586".parse()?;
    let my_order_book_aggregator = OrderbookAggregatorServer::new(MyOrderbookAggregator::default());
    println!("OrderbookAggregatorServer running on {:?}", &addr);
    let cors = CorsLayer::new().allow_headers(Any).allow_methods([Method::POST]).allow_origin(Any);


    Server::builder()
        .accept_http1(true)
        .layer(cors)
        .layer(GrpcWebLayer::new())
        .add_service(my_order_book_aggregator)
        .serve(addr)
        .await?;
    Ok(())
}
