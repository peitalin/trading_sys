use std::fmt;
use chrono::NaiveDateTime;
use serde::de;
use serde::de::{Deserialize, Deserializer};

use crate::currency_pairs::CurrencyPair;
use crate::serde_parsers::{deserialize_as_f32, deserialize_as_naive_date_time};
use crate::schema::aggregate_trades;


///////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "aggregate_trades"]
pub struct AggregateTradeData {
    #[serde(rename = "a")]
    pub trade_id: i32, // Trade ID
    #[serde(rename = "e")]
    pub event: String, // Event type
    #[serde(rename = "E")]
    #[serde(deserialize_with = "deserialize_as_naive_date_time")]
    pub event_time: NaiveDateTime, // Event time
    #[serde(rename = "s")]
    pub symbol: String, // Symbol
    #[serde(deserialize_with = "deserialize_as_f32")]
    #[serde(rename = "p")]
    pub price: f32, // Bids to be updated
    #[serde(deserialize_with = "deserialize_as_f32")]
    #[serde(rename = "q")]
    pub quantity: f32, // Asks to be updated
    #[serde(rename = "f")]
    pub first_trade_id: i32, // First update ID in event
    #[serde(rename = "l")]
    pub last_trade_id: i32, // Final update ID in event
    #[serde(rename = "T")]
    #[serde(deserialize_with = "deserialize_as_naive_date_time")]
    pub trade_time: NaiveDateTime, // Final update ID in event
    #[serde(rename = "m")]
    pub buyer_mkt_maker: bool, //  is buyer the market maker?
}

impl fmt::Display for AggregateTradeData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pretty_json = serde_json::to_string_pretty(&self).unwrap();
        write!(f, "{}", pretty_json)
    }
}
