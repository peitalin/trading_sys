use std::fmt;
use chrono::NaiveDateTime;
use serde::de;
use serde::de::{Deserialize, Deserializer};

use crate::currency_pairs::CurrencyPair;
use crate::serde_parsers::{deserialize_as_f32, deserialize_as_naive_date_time_ms};
use crate::schema::klines;


#[derive(Debug, Serialize, Deserialize)]
pub struct KlineMetaData {
    #[serde(rename = "e")]
    pub event: String, // Event type
    #[serde(rename = "E")]
    #[serde(deserialize_with = "deserialize_as_naive_date_time_ms")]
    pub event_time: NaiveDateTime, // Event time
    #[serde(rename = "s")]
    pub symbol: CurrencyPair, // Symbol
    #[serde(rename = "k")]
    pub kline_data: NewKlineData, //  without id
}


impl fmt::Display for KlineMetaData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}


#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "klines"]
pub struct NewKlineData {
    #[serde(rename = "t")]
    #[serde(deserialize_with = "deserialize_as_naive_date_time_ms")]
    pub start_time: NaiveDateTime, // Kline start time
    #[serde(rename = "T")]
    #[serde(deserialize_with = "deserialize_as_naive_date_time_ms")]
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

impl fmt::Display for NewKlineData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pretty_json = serde_json::to_string_pretty(&self).unwrap();
        write!(f, "{:#}", pretty_json)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "klines"]
pub struct KlineDataInsert {
    pub event: String,
    pub event_time: NaiveDateTime,
    pub start_time: NaiveDateTime,
    pub close_time: NaiveDateTime,
    pub symbol: CurrencyPair,
    pub interval: String,
    pub first_trade_id: i32,
    pub last_trade_id: i32,
    pub open: f32,
    pub close: f32,
    pub high: f32,
    pub low: f32,
    pub volume: f32,
    pub num_of_trades: i32,
    pub is_kline_closed: bool,
    pub quote_asset_vol: f32,
    pub taker_buy_base_vol: f32,
    pub taker_buy_quote_vol: f32,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "klines"]
pub struct KlineData {
    pub id: i32,
    pub event: String,
    pub event_time: NaiveDateTime,
    pub start_time: NaiveDateTime,
    pub close_time: NaiveDateTime,
    pub symbol: CurrencyPair,
    pub interval: String,
    pub first_trade_id: i32,
    pub last_trade_id: i32,
    pub open: f32,
    pub close: f32,
    pub high: f32,
    pub low: f32,
    pub volume: f32,
    pub num_of_trades: i32,
    pub is_kline_closed: bool,
    pub quote_asset_vol: f32,
    pub taker_buy_base_vol: f32,
    pub taker_buy_quote_vol: f32,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum KlineInterval {
    _1m,
    _3m,
    _5m,
    _15m,
    _30m,
    _1h,
    _2h,
    _4h,
    _6h,
    _8h,
    _12h,
    _1d,
    _3d,
    _1w,
    _1M,
}
impl std::fmt::Display for KlineInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            KlineInterval::_1m => write!(f, "1m"),
            KlineInterval::_3m => write!(f, "3m"),
            KlineInterval::_5m => write!(f, "5m"),
            KlineInterval::_15m => write!(f, "15m"),
            KlineInterval::_30m => write!(f, "30m"),
            KlineInterval::_1h => write!(f, "1h"),
            KlineInterval::_2h => write!(f, "2h"),
            KlineInterval::_4h => write!(f, "4h"),
            KlineInterval::_6h => write!(f, "6h"),
            KlineInterval::_8h => write!(f, "8h"),
            KlineInterval::_12h => write!(f, "12h"),
            KlineInterval::_1d => write!(f, "1d"),
            KlineInterval::_3d => write!(f, "3d"),
            KlineInterval::_1w => write!(f, "1w"),
            KlineInterval::_1M => write!(f, "1M"),
        }
    }
}



pub static TEST_KLINE_DATA: &str = r#"
{
  "e": "kline",
  "E": 1222333444,
  "s": "BNBBTC",
  "k": {
    "t": 1222333444,
    "T": 1222333444,
    "s": "BNBBTC",
    "i": "1m",
    "f": 100,
    "L": 200,
    "o": "0.0010",
    "c": "0.0020",
    "h": "0.0025",
    "l": "0.0015",
    "v": "1000",
    "n": 100,
    "x": false,
    "q": "1.0000",
    "V": "500",
    "Q": "0.500",
    "B": "123456"
  }
}
"#;

#[cfg(test)]
mod tests {
    #[test]
    fn test_deserialization_klines() {
        use crate::models::klines::TEST_KLINE_DATA;
        use crate::models::klines::KlineDataInsert;
        use crate::models::klines::NewKlineData;
        use crate::models::klines::KlineMetaData;
        use crate::serde_parsers::create_timestamp_benchmark;

        let kline_meta_data =
                    serde_json::from_str::<KlineMetaData>(&TEST_KLINE_DATA).unwrap();
        let kd = kline_meta_data.kline_data;
        let kline_data_insert = KlineDataInsert {
            event:          kline_meta_data.event,
            event_time:     kline_meta_data.event_time,
            start_time:     kd.start_time,
            close_time:     kd.close_time,
            symbol:         kd.symbol,
            interval:       kd.interval,
            first_trade_id: kd.first_trade_id,
            last_trade_id:  kd.last_trade_id,
            open:           kd.open,
            close:          kd.close,
            high:           kd.high,
            low:            kd.low,
            volume:         kd.volume,
            num_of_trades:  kd.num_of_trades,
            is_kline_closed:     kd.is_kline_closed,
            quote_asset_vol:     kd.quote_asset_vol,
            taker_buy_base_vol:  kd.taker_buy_base_vol,
            taker_buy_quote_vol: kd.taker_buy_quote_vol,
        };

        let mock_data = KlineDataInsert {
            event: "kline".to_owned(),
            event_time: create_timestamp_benchmark(1_222_333_444),
            start_time: create_timestamp_benchmark(1_222_333_444),
            close_time: create_timestamp_benchmark(1_222_333_444),
            symbol:     crate::currency_pairs::CurrencyPair::BNBBTC,
            interval:   "1m".to_owned(),
            first_trade_id: 100,
            last_trade_id:  200,
            open: 0.0010,
            close: 0.0020,
            high: 0.0025,
            low: 0.0015,
            volume: 1000.0,
            num_of_trades: 100,
            is_kline_closed: false,
            quote_asset_vol: 1.0,
            taker_buy_base_vol: 500.0,
            taker_buy_quote_vol: 0.5,
        };
        assert_eq!(kline_data_insert, mock_data)
    }
}








