use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use crate::api_objects::PairCurrencies;

#[tonic::async_trait]
pub trait OrdersPuller {
    async fn pull_orders(
        &self,
        pair_currencies: PairCurrencies,
        socket: Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ) {
    }
}
