use orderbook::orderbook_aggregator_server::{OrderbookAggregator, OrderbookAggregatorServer};
use orderbook::{Level, Summary};
use std::net::SocketAddr;
use std::sync::mpsc::{Receiver, Sender};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};
use tonic_web::GrpcWebLayer;
use tower_http::cors::{Any, CorsLayer};
use http::Method;



pub mod orderbook {
    tonic::include_proto!("orderbook");
}

impl Summary {
    pub fn new(spread: f64, bids: Vec<Level>, asks: Vec<Level>) -> Self {
        Summary { spread, bids, asks }
    }
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
        let (tx, rx) = mpsc::channel(4);
        println!("Running book_summsry");
        // let features = .clone();
        let summary = Summary::new(0.0, vec![], vec![]);
        
        tokio::spawn(async move {
            tx.send(Ok(summary.clone())).await.unwrap();
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
