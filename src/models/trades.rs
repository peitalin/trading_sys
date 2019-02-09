use chrono::NaiveDateTime;
use serde::de;
use serde::de::{Deserialize, Deserializer};
use std::fmt;

use crate::currency_pairs::CurrencyPair;
use crate::schema::trades;
use crate::serde_parsers::{deserialize_as_f32, deserialize_as_naive_date_time_ms};

#[derive(Debug, PartialEq, Serialize, Deserialize, Queryable, Insertable)]
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
    #[serde(rename = "T")]
    #[serde(deserialize_with = "deserialize_as_naive_date_time_ms")]
    pub trade_time: NaiveDateTime, // trade time
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


pub static TEST_TRADE_DATA: &str = r#"
{
  "e": "trade",
  "E": 1555444333222,
  "s": "BNBBTC",
  "t": 12345,
  "p": "0.001",
  "q": "100",
  "b": 88,
  "a": 50,
  "T": 1666555444333,
  "m": true,
  "M": true
}
"#;

#[cfg(test)]
mod tests {
    #[test]
    fn try_deserialize_trades() {
        use crate::models::trades::{
            TradeData,
            TEST_TRADE_DATA,
        };
        use crate::serde_parsers::create_timestamp_benchmark;

        let trade_data = serde_json::from_str::<TradeData>(&TEST_TRADE_DATA).unwrap();

        let mock_data = TradeData {
            trade_id: 12345,
            event: "trade".to_owned(),
            event_time: create_timestamp_benchmark(1_555_444_333_222),
            symbol: crate::currency_pairs::CurrencyPair::BNBBTC,
            price: 0.001,
            quantity: 100.0,
            buyer_order_id: 88,
            seller_order_id: 50,
            trade_time: create_timestamp_benchmark(1_666_555_444_333),
            buyer_mkt_maker: true,
        };
        assert_eq!(trade_data, mock_data)
    }
}
