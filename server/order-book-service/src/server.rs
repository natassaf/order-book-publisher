use std::error::Error;

use orderbook::orderbook_aggregator_server::{OrderbookAggregator, OrderbookAggregatorServer};
use orderbook::{Level, Summary};
use tonic::{transport::Server, Request, Response, Status};

pub mod orderbook {
    tonic::include_proto!("orderbook");
}

impl Summary {
    pub fn new(spread: f64, bids: Vec<Level>, asks: Vec<Level>) -> Self {
        Summary { spread, bids, asks }
    }
}

#[derive(Clone, Debug)]
pub struct MyOrderbookAggregator {}

impl MyOrderbookAggregator {
    pub fn new() -> Self {
        MyOrderbookAggregator {}
    }
}

#[tonic::async_trait]
impl OrderbookAggregator for MyOrderbookAggregator {
    async fn book_summary(
        &self,
        request: Request<()>,
    ) -> Result<tonic::Response<Self::BookSummaryStream>, tonic::Status> {
        Ok(Response::new(Summary::new(0.0, vec![], vec![])))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyOrderbookAggregator::default();

    Server::builder()
        .add_service(MyOrderbookAggregator::new())
        .serve(addr)
        .await?;

    Ok(())
}
