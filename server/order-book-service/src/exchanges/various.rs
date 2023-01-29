use serde::{de, Deserialize, Deserializer};
use serde_derive::Deserialize;

use tokio::{ sync::mpsc::Sender, io};

use crate::api_objects::Level;

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

#[derive(Debug, Deserialize)]
pub struct Item {
    pub microtimestamp: String,
    pub timestamp: String,
    pub bids: Vec<OfferData>,
    pub asks: Vec<OfferData>,
}

#[derive(Debug, Deserialize)]
pub struct BitstampResponse {
    pub channel: String,
    pub data: Item,
    pub event: String,
}

#[derive(Copy, Clone)]
pub enum BinanceSpeeds {
    HundredMill = 100,
    ThousandMill = 1000,
}


