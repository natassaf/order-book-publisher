use orderbook::orderbook_aggregator_server::{OrderbookAggregator, OrderbookAggregatorServer};
use orderbook::{Level, Summary};
use std::error::Error;
use std::sync::mpsc::{Receiver, Sender};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

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
    let addr = "[::1]:50051".parse()?;
    let my_order_book_aggregator = MyOrderbookAggregator::default();

    Server::builder()
        .add_service(OrderbookAggregatorServer::new(my_order_book_aggregator))
        .serve(addr)
        .await?;
    Ok(())
}
