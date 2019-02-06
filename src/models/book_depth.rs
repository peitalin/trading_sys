use std::fmt;
use chrono::NaiveDateTime;
use serde::de;
use serde::de::{Deserialize, Deserializer};

use crate::currency_pairs::CurrencyPair;
use crate::serde_parsers::{deserialize_as_f32, deserialize_as_naive_date_time};
use crate::schema::book_depth;


#[derive(Queryable)]
pub struct BookDepthDataQuery {
    pub id: i32,
    pub event: String,             // Event type
    pub event_time: NaiveDateTime, // Event time
    pub symbol: CurrencyPair,      // Symbol
    pub update_first: i32,         // First update ID in event
    pub update_final: i32,         // Final update ID in event
    pub bids: Vec<Quote>,          // Bids to be updated
    pub asks: Vec<Quote>,          // Asks to be updated
}

///////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "book_depth"]
pub struct BookDepthData {
    #[serde(rename = "e")]
    pub event: String, // Event type
    #[serde(rename = "E")]
    #[serde(deserialize_with = "deserialize_as_naive_date_time")]
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

impl fmt::Display for BookDepthData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pretty_json = serde_json::to_string_pretty(&self).unwrap();
        write!(f, "{}", pretty_json)
    }
}

#[derive(Debug, Serialize, Clone)]
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
        println!("---> {:?}", value);
        let jsond: Quote = serde_json::from_value(value)?;
        println!("---> {:?}", jsond);
        Ok(jsond)
    }
}