use std::fmt;
use chrono::NaiveDateTime;
use serde::de;
use serde::de::{Deserialize, Deserializer};

use crate::currency_pairs::CurrencyPair;
use crate::serde_parsers::{deserialize_as_f32, deserialize_as_naive_date_time};
use crate::schema::mini_tickers;


#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "mini_tickers"]
pub struct MiniTickerDataInsert {
    #[serde(rename = "e")]
    pub event: String,       // Event type
    #[serde(rename = "E")]
    #[serde(deserialize_with = "deserialize_as_naive_date_time")]
    pub event_time: NaiveDateTime,     // Event time
    #[serde(rename = "s")]
    pub symbol: CurrencyPair,  // Symbol
    #[serde(rename = "o")]
    #[serde(deserialize_with = "deserialize_as_f32")]
    pub open: f32,          // Open price
    #[serde(rename = "c")]
    #[serde(deserialize_with = "deserialize_as_f32")]
    pub close: f32,          // Close price
    #[serde(rename = "h")]
    #[serde(deserialize_with = "deserialize_as_f32")]
    pub high: f32,          // High price
    #[serde(rename = "l")]
    #[serde(deserialize_with = "deserialize_as_f32")]
    pub low: f32,          // Low price
    #[serde(rename = "v")]
    #[serde(deserialize_with = "deserialize_as_f32")]
    pub base_asset_vol: f32,  // Total traded base asset volume
    #[serde(rename = "q")]
    #[serde(deserialize_with = "deserialize_as_f32")]
    pub quote_asset_vol: f32  // Total traded quote asset volume
}

impl std::fmt::Display for MiniTickerData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "mini_tickers"]
pub struct MiniTickerData {
    pub event: String,       // Event type
    pub event_time: NaiveDateTime,     // Event time
    pub symbol: CurrencyPair,  // Symbol
    pub open: f32,          // Open price
    pub close: f32,          // Close price
    pub high: f32,          // High price
    pub low: f32,          // Low price
    pub base_asset_vol: f32,  // Total traded base asset volume
    pub quote_asset_vol: f32  // Total traded quote asset volume
}
