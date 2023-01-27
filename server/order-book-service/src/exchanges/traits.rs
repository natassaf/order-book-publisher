use tokio::io::Error;
use futures_util::Future;

use crate::api_objects::PairCurrencies;

// #[tonic::async_trait]
// pub trait OrdersPuller {
//     async fn pull_orders(
//         &self,
//         pair_currencies: PairCurrencies,
//     ) ->dyn Future<Output=Result<Option<Vec<u8>>, Error>>{}
