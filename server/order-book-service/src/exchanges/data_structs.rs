// use serde_derive::Deserialize;

// #[derive(Deserialize, Debug)]
// pub struct BinanceResponse {
//     asks: Vec<Entries>,
//     bids:Vec<Entries>
// }

// #[derive(Deserialize, Debug)]
// pub struct Entries {
//     entries: Vec<Order>,
// }

// #[derive(Deserialize, Debug)]
// pub struct Order{
//     price:String,
//     amount:String
// }

use serde::de;
use serde::{Deserialize, Deserializer};
use serde_derive::Deserialize;


#[derive(Debug, Deserialize)]
pub struct OfferData {
    #[serde(deserialize_with = "de_float_from_str")]
    pub price: f32,
    #[serde(deserialize_with = "de_float_from_str")]
    pub size: f32,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepthStreamData {
    pub last_update_id: usize,
    pub bids: Vec<OfferData>,
    pub asks: Vec<OfferData>,
}
pub fn de_float_from_str<'a, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'a>,
{
    let str_val = String::deserialize(deserializer)?;
    str_val.parse::<f32>().map_err(de::Error::custom)
}