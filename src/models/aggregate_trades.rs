use chrono::NaiveDateTime;
use serde::de;
use serde::de::{Deserialize, Deserializer};
use std::fmt;

use crate::currency_pairs::CurrencyPair;
use crate::schema::aggregate_trades;
use crate::serde_parsers::{deserialize_as_f32, deserialize_as_naive_date_time_ms};

///////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "aggregate_trades"]
pub struct AggregateTradeData {
    #[serde(rename = "a")]
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
    pub price: f32, // Bids to be updated
    #[serde(deserialize_with = "deserialize_as_f32")]
    #[serde(rename = "q")]
    pub quantity: f32, // Asks to be updated
    #[serde(rename = "f")]
    pub first_trade_id: i32, // First update ID in event
    #[serde(rename = "l")]
    pub last_trade_id: i32, // Final update ID in event
    #[serde(rename = "T")]
    #[serde(deserialize_with = "deserialize_as_naive_date_time_ms")]
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


pub static TEST_AGGTRADE_DATA: &str = r#"
{
  "e": "aggTrade",
  "E": 1555444333222,
  "s": "BNBBTC",
  "a": 12345,
  "p": "0.001",
  "q": "100",
  "f": 100,
  "l": 105,
  "T": 1666555444333,
  "m": true,
  "M": true
}
"#;

#[cfg(test)]
mod tests {
    #[test]
    fn try_deserialize_aggregate_trades() {
        use crate::models::aggregate_trades::{
            AggregateTradeData,
            TEST_AGGTRADE_DATA,
        };
        use crate::serde_parsers::create_timestamp_benchmark;

        let aggtrade_data = serde_json::from_str::<AggregateTradeData>(&TEST_AGGTRADE_DATA).unwrap();

        let mock_data = AggregateTradeData {
            trade_id: 12345,
            event: "aggTrade".to_owned(),
            event_time: create_timestamp_benchmark(1_555_444_333_222),
            symbol: crate::currency_pairs::CurrencyPair::BNBBTC,
            price: 0.001,
            quantity: 100.0,
            first_trade_id: 100,
            last_trade_id: 105,
            trade_time: create_timestamp_benchmark(1_666_555_444_333),
            buyer_mkt_maker: true,
        };
        assert_eq!(aggtrade_data, mock_data)
    }
}
