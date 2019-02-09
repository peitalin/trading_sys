use chrono::NaiveDateTime;
use serde::de;
use serde::de::{Deserialize, Deserializer};
use std::fmt;

use crate::currency_pairs::CurrencyPair;
use crate::schema::tickers;
use crate::serde_parsers::{deserialize_as_f32, deserialize_as_naive_date_time_ms};


#[derive(Debug, Serialize, Deserialize, Insertable, PartialEq)]
#[table_name = "tickers"]
pub struct TickerDataInsert {
    #[serde(rename = "e")]
    pub event: String, // Event type
    #[serde(rename = "E")]
    #[serde(deserialize_with = "deserialize_as_naive_date_time_ms")]
    pub event_time: NaiveDateTime, // Event time
    #[serde(rename = "s")]
    pub symbol: CurrencyPair, // Symbol
    #[serde(rename = "p")]
    #[serde(deserialize_with = "deserialize_as_f32")]
    pub price_change: f32, // Price change
    #[serde(rename = "P")]
    #[serde(deserialize_with = "deserialize_as_f32")]
    pub price_change_pct: f32, // Price change percent
    #[serde(rename = "w")]
    #[serde(deserialize_with = "deserialize_as_f32")]
    pub weight_avg_price: f32, // weighted average price
    #[serde(rename = "x")]
    #[serde(deserialize_with = "deserialize_as_f32")]
    pub first_trade_before_24hr_window: f32, // First trade(F)-1 price (first trade before the 24hr rolling window)
    #[serde(rename = "c")]
    #[serde(deserialize_with = "deserialize_as_f32")]
    pub last_price: f32, // Last Price
    #[serde(rename = "Q")]
    #[serde(deserialize_with = "deserialize_as_f32")]
    pub last_quantity: f32, // Last Quantity
    #[serde(rename = "b")]
    #[serde(deserialize_with = "deserialize_as_f32")]
    pub best_bid_price: f32, // Best bid price
    #[serde(rename = "B")]
    #[serde(deserialize_with = "deserialize_as_f32")]
    pub best_bid_quantity: f32, // Best bid quantity
    #[serde(rename = "a")]
    #[serde(deserialize_with = "deserialize_as_f32")]
    pub best_ask_price: f32, // Best ask price
    #[serde(rename = "A")]
    #[serde(deserialize_with = "deserialize_as_f32")]
    pub best_ask_quantity: f32, // Best ask quantity
    #[serde(rename = "o")]
    #[serde(deserialize_with = "deserialize_as_f32")]
    pub open_price: f32, // Open Price
    #[serde(rename = "h")]
    #[serde(deserialize_with = "deserialize_as_f32")]
    pub high_price: f32, // High Price
    #[serde(rename = "l")]
    #[serde(deserialize_with = "deserialize_as_f32")]
    pub low_price: f32, // Low price
    #[serde(rename = "v")]
    #[serde(deserialize_with = "deserialize_as_f32")]
    pub base_asset_vol: f32, // Total traded base asset volume
    #[serde(rename = "q")]
    #[serde(deserialize_with = "deserialize_as_f32")]
    pub quote_asset_vol: f32, // Total traded quote asset volume
    #[serde(rename = "O")]
    #[serde(deserialize_with = "deserialize_as_naive_date_time_ms")]
    pub open_time: NaiveDateTime, // Statistics Open Time
    #[serde(rename = "C")]
    #[serde(deserialize_with = "deserialize_as_naive_date_time_ms")]
    pub close_time: NaiveDateTime, // Statistics close time
    #[serde(rename = "F")]
    pub first_trade_id: i32, // First trade ID
    #[serde(rename = "L")]
    pub last_trade_id: i32, // Last trade ID
    #[serde(rename = "n")]
    pub total_num_trades: i32, // Total number of trades
}

impl std::fmt::Display for TickerData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct TickerData {
    pub id: i32,                    // PostgreSQL id
    pub event: String,              // Event type
    pub event_time: NaiveDateTime,  // Event time
    pub symbol: CurrencyPair,       // Symbol
    pub price_change: f32,          // Price change
    pub price_change_pct: f32,      // Price change percent
    pub weight_avg_price: f32,      // weighted average price
    pub first_trade_before_24hr_window: f32, // First trade(F)-1 price (first trade before the 24hr rolling window)
    pub last_price: f32,            // Last Price
    pub last_quantity: f32,         // Last Quantity
    pub best_bid_price: f32,        // Best bid price
    pub best_bid_quantity: f32,     // Best bid quantity
    pub best_ask_price: f32,        // Best ask price
    pub best_ask_quantity: f32,     // Best ask quantity
    pub open_price: f32,            // Open Price
    pub high_price: f32,            // High Price
    pub low_price: f32,             // Low price
    pub base_asset_vol: f32,        // Total traded base asset volume
    pub quote_asset_vol: f32,       // Total traded quote asset volume
    pub open_time: NaiveDateTime,   // Statistics Open Time
    pub close_time: NaiveDateTime,  // Statistics close time
    pub first_trade_id: i32,        // First trade ID
    pub last_trade_id: i32,         // Last trade ID
    pub total_num_trades: i32,      // Total number of trades
}


pub static TEST_TICKER_DATA: &str = r#"
{
    "e": "24hrTicker",
    "E": 1222333444555,
    "s": "BNBBTC",
    "p": "0.0015",
    "P": "250.00",
    "w": "0.0018",
    "x": "0.0009",
    "c": "0.0025",
    "Q": "10",
    "b": "0.0024",
    "B": "10",
    "a": "0.0026",
    "A": "100",
    "o": "0.0010",
    "h": "0.0025",
    "l": "0.0010",
    "v": "10000",
    "q": "18",
    "O": 1555444333222,
    "C": 1666555444333,
    "F": 0,
    "L": 18150,
    "n": 18151
}
"#;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_deserialize_ticker() {
        use crate::models::tickers::TickerDataInsert;
        use crate::models::tickers::TEST_TICKER_DATA;
        use crate::serde_parsers::create_timestamp_benchmark;

        let jsond_test =
            serde_json::from_str::<TickerDataInsert>(TEST_TICKER_DATA).unwrap();
        let mock_data = TickerDataInsert {
            event: "24hrTicker".to_owned(),
            event_time: create_timestamp_benchmark(1_222_333_444_555),
            symbol: crate::currency_pairs::CurrencyPair::BNBBTC,
            price_change: 0.0015,
            price_change_pct: 250.00,
            weight_avg_price: 0.0018,
            first_trade_before_24hr_window: 0.0009,
            last_price: 0.0025,
            last_quantity: 10.,
            best_bid_price: 0.0024,
            best_bid_quantity: 10.0,
            best_ask_price: 0.0026,
            best_ask_quantity: 100.,
            open_price: 0.0010,
            high_price: 0.0025,
            low_price: 0.0010,
            base_asset_vol: 10000.0,
            quote_asset_vol: 18.0,
            open_time: create_timestamp_benchmark(1_555_444_333_222),
            close_time: create_timestamp_benchmark(1_666_555_444_333),
            first_trade_id: 0,
            last_trade_id: 18150,
            total_num_trades: 18151
        };
        assert_eq!(jsond_test, mock_data)
    }
}



