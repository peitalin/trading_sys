use std::fmt;
use chrono::NaiveDateTime;
use serde::de;
use serde::de::{Deserialize, Deserializer};

use crate::currency_pairs::CurrencyPair;
use crate::serde_parsers::{deserialize_as_f32, deserialize_as_naive_date_time_ms};
use crate::schema::trades;


#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "trades"]
pub struct TradeData {
    #[serde(rename = "t")]
    pub trade_id: i32, // Trade ID
    #[serde(rename = "e")]
    pub event: String, // Event type
    #[serde(rename = "E")]
    #[serde(deserialize_with = "deserialize_as_naive_date_time_ms")]
    pub event_time: NaiveDateTime, // Event time
    #[serde(rename = "s")]
    pub symbol: CurrencyPair, // Symbol
    #[serde(deserialize_with = "deserialize_as_f32")]
    #[serde(rename = "p")]
    pub price: f32, // Price
    #[serde(deserialize_with = "deserialize_as_f32")]
    #[serde(rename = "q")]
    pub quantity: f32, // Quantity
    #[serde(rename = "b")]
    pub buyer_order_id: i32, // Buyer order ID
    #[serde(rename = "a")]
    pub seller_order_id: i32, // Seller order ID
    #[serde(rename = "m")]
    pub buyer_mkt_maker: bool, //  is buyer the market maker?
}

impl std::fmt::Display for TradeData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
