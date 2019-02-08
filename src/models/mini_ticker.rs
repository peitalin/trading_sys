use std::fmt;
use chrono::NaiveDateTime;
use serde::de;
use serde::de::{Deserialize, Deserializer};

use crate::currency_pairs::CurrencyPair;
use crate::serde_parsers::{deserialize_as_f32, deserialize_as_naive_date_time_ms};
use crate::schema::mini_tickers;


#[derive(Debug, Serialize, Deserialize, Insertable, PartialEq)]
#[table_name = "mini_tickers"]
pub struct MiniTickerDataInsert {
    #[serde(rename = "e")]
    pub event: String,       // Event type
    #[serde(rename = "E")]
    #[serde(deserialize_with = "deserialize_as_naive_date_time_ms")]
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

#[derive(Debug, Clone)]
pub enum MiniTickerQueryType {
    AllMarkets,
    SingleMarket,
}

pub static TEST_MINI_TICKER_DATA: &str = r#"
{
    "e": "24hrMiniTicker",
    "E": 1222333444555,
    "s": "BNBBTC",
    "c": "0.0025",
    "o": "0.0010",
    "h": "0.0025",
    "l": "0.0010",
    "v": "10000",
    "q": "18"
}
"#;

#[cfg(test)]
mod tests {
    #[test]
    fn test_deserialization_mini_ticker() {
        use crate::models::mini_ticker::TEST_MINI_TICKER_DATA;
        use crate::models::mini_ticker::MiniTickerDataInsert;
        use crate::serde_parsers::create_timestamp_benchmark;

        let jsond_test = serde_json::from_str::<MiniTickerDataInsert>(TEST_MINI_TICKER_DATA).unwrap();
        let mock_data = MiniTickerDataInsert {
            event: "24hrMiniTicker".to_owned(),
            event_time: create_timestamp_benchmark(1_222_333_444_555),
            symbol: crate::currency_pairs::CurrencyPair::BNBBTC,
            close: 0.0025,
            open: 0.0010,
            high: 0.0025,
            low: 0.0010,
            base_asset_vol: 10000.0,
            quote_asset_vol: 18.0
        };
        assert_eq!(jsond_test, mock_data)
    }
}








