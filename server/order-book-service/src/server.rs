use orderbook::orderbook_aggregator_server::{OrderbookAggregator, OrderbookAggregatorServer};
use std::net::SocketAddr;
use std::sync::mpsc::{Receiver, Sender};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};
use tonic_web::GrpcWebLayer;
use tower_http::cors::{Any, CorsLayer};
use http::Method;


mod api_objects;
use api_objects::{Summary, Exchange};

use crate::api_objects::PairCurrencies;


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
        let (tx, rx) = mpsc::channel(4);
        
        tokio::spawn(async move {
            for i in 0..10{
                let result:Result<Summary, Status> = book_summary_endpoint::process(PairCurrencies::ETHBTC, Exchange::BINANCE, Exchange::BITSTAMP).await;
                println!("{:?}",i);
                println!("result: {:?}", result.clone().unwrap());
                tx.send(result).await.unwrap();
            }

        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
