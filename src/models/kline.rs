use std::fmt;
use chrono::NaiveDateTime;
use serde::de;
use serde::de::{Deserialize, Deserializer};

use crate::currency_pairs::CurrencyPair;
use crate::serde_parsers::{deserialize_as_f32, deserialize_as_naive_date_time};
use crate::schema::klines;


#[derive(Debug, Serialize, Deserialize)]
pub struct KlineMetaData {
    #[serde(rename = "e")]
    pub event: String, // Event type
    #[serde(rename = "E")]
    #[serde(deserialize_with = "deserialize_as_naive_date_time")]
    pub event_time: NaiveDateTime, // Event time
    #[serde(rename = "s")]
    pub symbol: CurrencyPair, // Symbol
    #[serde(rename = "k")]
    pub kline_data: KlineData, //  KlineData
}

impl fmt::Display for KlineMetaData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}




#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "klines"]
pub struct KlineData {
    #[serde(rename = "t")]
    #[serde(deserialize_with = "deserialize_as_naive_date_time")]
    pub start_time: NaiveDateTime, // Kline start time
    #[serde(rename = "T")]
    #[serde(deserialize_with = "deserialize_as_naive_date_time")]
    pub close_time: NaiveDateTime, // Kline close time
    #[serde(rename = "s")]
    pub symbol: CurrencyPair, // Symbol
    #[serde(rename = "i")]
    pub interval: String, // Kline Intervel
    #[serde(rename = "f")]
    pub first_trade_id: i32, // First trade ID
    #[serde(rename = "L")]
    pub last_trade_id: i32, // Last trade ID
    #[serde(rename = "o")]
    #[serde(deserialize_with = "deserialize_as_f32")]
    pub open: f32, // Open price
    #[serde(rename = "c")]
    #[serde(deserialize_with = "deserialize_as_f32")]
    pub close: f32, // Close price
    #[serde(rename = "h")]
    #[serde(deserialize_with = "deserialize_as_f32")]
    pub high: f32, // High price
    #[serde(rename = "l")]
    #[serde(deserialize_with = "deserialize_as_f32")]
    pub low: f32, // Low price
    #[serde(rename = "v")]
    #[serde(deserialize_with = "deserialize_as_f32")]
    pub volume: f32, // Volume
    #[serde(rename = "n")]
    pub num_of_trades: i32, // Number of trades
    #[serde(rename = "x")]
    pub is_kline_closed: bool, // Is this Kline closed?
    #[serde(rename = "q")]
    #[serde(deserialize_with = "deserialize_as_f32")]
    pub quote_asset_vol: f32, // Quote asset volume
    #[serde(rename = "V")]
    #[serde(deserialize_with = "deserialize_as_f32")]
    pub taker_buy_base_vol: f32, // Taker buy base asset volume
    #[serde(rename = "Q")]
    #[serde(deserialize_with = "deserialize_as_f32")]
    pub taker_buy_quote_vol: f32, // Taker buy quote asset volume
}

impl fmt::Display for KlineData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pretty_json = serde_json::to_string_pretty(&self).unwrap();
        write!(f, "{:#}", pretty_json)
    }
}


#[derive(Queryable)]
pub struct KlineDataQuery {
    id: i32,
    start_time: NaiveDateTime,
    close_time: NaiveDateTime,
    symbol: CurrencyPair,
    interval: String,
    first_trade_id: i32,
    last_trade_id: i32,
    open: f32,
    close: f32,
    high: f32,
    low: f32,
    volume: f32,
    num_of_trades: i32,
    is_kline_closed: bool,
    quote_asset_vol: f32,
    taker_buy_base_vol: f32,
    taker_buy_quote_vol: f32,
}
