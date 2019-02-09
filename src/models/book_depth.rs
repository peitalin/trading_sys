use chrono::NaiveDateTime;
use serde::de;
use serde::de::{Deserialize, Deserializer};
use std::fmt;

use crate::currency_pairs::CurrencyPair;
use crate::schema::book_depth;
use crate::serde_parsers::{deserialize_as_f32, deserialize_as_naive_date_time_ms};

#[derive(Queryable)]
pub struct BookDepthData {
    pub id: i32,
    pub event: String,             // Event type
    pub event_time: NaiveDateTime, // Event time
    pub symbol: CurrencyPair,      // Symbol
    pub update_first: i32,         // First update ID in event
    pub update_final: i32,         // Final update ID in event
    pub bids: Vec<Quote>,          // Bids to be updated
    pub asks: Vec<Quote>,          // Asks to be updated
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PartialBookDepthData {
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: i32, // Last update ID
    pub bids: Vec<Quote>, // Bids to be updated
    pub asks: Vec<Quote>, // Asks to be updated
}

impl fmt::Display for PartialBookDepthData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pretty_json = serde_json::to_string_pretty(&self).unwrap();
        write!(f, "{}", pretty_json)
    }
}

///////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "book_depth"]
pub struct BookDepthDataInsert {
    #[serde(rename = "e")]
    pub event: String, // Event type
    #[serde(rename = "E")]
    #[serde(deserialize_with = "deserialize_as_naive_date_time_ms")]
    pub event_time: NaiveDateTime, // Event time
    #[serde(rename = "s")]
    pub symbol: CurrencyPair, // Symbol
    #[serde(rename = "U")]
    pub update_first: i32, // First update ID in event
    #[serde(rename = "u")]
    pub update_final: i32, // Final update ID in event
    #[serde(rename = "b")]
    pub bids: Vec<Quote>, // Bids to be updated
    #[serde(rename = "a")]
    pub asks: Vec<Quote>, // Asks to be updated
}

impl fmt::Display for BookDepthDataInsert {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // let pretty_json = serde_json::to_string_pretty(&self).unwrap();
        write!(f, "{}", &self)
    }
}

#[derive(Debug, PartialEq, Serialize, Clone)]
pub struct Quote {
    pub price: f32,
    pub quantity: f32,
}
/// This trait deserializes nested vector/strings: ["0.00123", "150.4", []]
/// and turns it into Quote { price: 0.00123, quantity: 150.4 }
impl<'de> Deserialize<'de> for Quote {
    fn deserialize<D>(deserializer: D) -> Result<Quote, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct QuoteVisitor;

        impl<'de> de::Visitor<'de> for QuoteVisitor {
            type Value = Quote;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("string or a list of strings")
            }

            fn visit_map<M>(self, mut dict: M) -> Result<Self::Value, M::Error>
            where
                M: de::MapAccess<'de>,
            {
                let mut h: std::collections::HashMap<String, f32> =
                    std::collections::HashMap::new();
                loop {
                    match dict.next_entry()? {
                        Some((key, val)) => {
                            h.insert(key, val);
                        }
                        None => break,
                    }
                }
                Ok(Quote {
                    price: *h.get("price").unwrap(),
                    quantity: *h.get("quantity").unwrap(),
                })
            }

            fn visit_seq<S>(self, mut visitor: S) -> Result<Self::Value, S::Error>
            where
                S: de::SeqAccess<'de>,
            {
                let mut vec = Vec::new();
                // visitor iterators nested vector/string: ["0.00123", "150.4", []]
                loop {
                    match visitor.next_element()? {
                        Some(StringOrVec::Price(elem)) => vec.push(elem), // convert strings to f64
                        Some(StringOrVec::Vec(elem)) => continue,         // skip vectors []
                        None => break, // break, when next_element() is empty
                    }
                }
                Ok(Quote {
                    price: vec[0],
                    quantity: vec[1],
                })
            }
        }
        deserializer.deserialize_any(QuoteVisitor)
    }
}

/// These functions deserializes:
/// StringOrVec::Price(elem) and StringOrVec::Vec(elem) accordingly
#[derive(Debug, Serialize, Clone)]
pub enum StringOrVec {
    Price(f32),
    Vec(Option<Vec<f32>>),
}
impl<'de> Deserialize<'de> for StringOrVec {
    fn deserialize<D>(deserializer: D) -> Result<StringOrVec, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StringOrVecVisitor;

        impl<'de> de::Visitor<'de> for StringOrVecVisitor {
            type Value = StringOrVec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("string or list of strings")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(StringOrVec::Price(value.to_owned().parse::<f32>().unwrap()))
            }

            fn visit_seq<S>(self, visitor: S) -> Result<Self::Value, S::Error>
            where
                S: de::SeqAccess<'de>,
            {
                // Ignore empty lists from Binance
                Ok(StringOrVec::Vec(None))
            }
        }
        deserializer.deserialize_any(StringOrVecVisitor)
    }
}

use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::serialize::{Output, ToSql};
use diesel::sql_types::Jsonb;

impl ToSql<Jsonb, Pg> for Quote {
    fn to_sql<W: std::io::Write>(&self, out: &mut Output<W, Pg>) -> diesel::serialize::Result {
        let quote: serde_json::Value = json!({ "price": self.price, "quantity": self.quantity });
        ToSql::<Jsonb, Pg>::to_sql(&quote, out)
    }
}

impl FromSql<Jsonb, Pg> for Quote {
    fn from_sql(maybe_bytes: Option<&[u8]>) -> diesel::deserialize::Result<Self> {
        let value = <serde_json::Value as FromSql<Jsonb, Pg>>::from_sql(maybe_bytes)?;
        let jsond: Quote = serde_json::from_value(value)?;
        Ok(jsond)
    }
}

#[derive(Debug, Clone)]
pub enum DepthLevels {
    _5,
    _10,
    _20,
}
impl fmt::Display for DepthLevels {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DepthLevels::_5 => write!(f, "5"),
            DepthLevels::_10 => write!(f, "10"),
            DepthLevels::_20 => write!(f, "20"),
        }
    }
}



pub static TEST_BOOKDEPTH_DATA: &str = r#"
{
  "e": "depthUpdate",
  "E": 1555444333222,
  "s": "BNBBTC",
  "U": 157,
  "u": 160,
  "b": [
    [
      "0.0024",
      "10",
      []
    ]
  ],
  "a": [
    [
      "0.0026",
      "100",
      []
    ]
  ]
}
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_deserialize_book_depth() {
        use crate::models::book_depth::{
            BookDepthDataInsert,
            TEST_BOOKDEPTH_DATA,
        };
        use crate::serde_parsers::create_timestamp_benchmark;

        let test_book_depth_data = serde_json::from_str::<BookDepthDataInsert>(&TEST_BOOKDEPTH_DATA).unwrap();

        let mock_data = BookDepthDataInsert {
            event: "depthUpdate".to_owned(),
            event_time: create_timestamp_benchmark(1_555_444_333_222),
            symbol: crate::currency_pairs::CurrencyPair::BNBBTC,
            update_first: 157,
            update_final: 160,
            bids: vec![Quote { price: 0.0024, quantity: 10.0 }],
            asks: vec![Quote { price: 0.0026, quantity: 100.0 }],
        };
        assert_eq!(test_book_depth_data, mock_data)
    }
}
