

use std::fmt;
use serde::de;
use serde::de::{Deserialize, Deserializer, Error, SeqAccess, Unexpected, Visitor};
use serde_derive::*;


///////////////////////////////////////////////////////////////////////////
/// Deserializers
///////////////////////////////////////////////////////////////////////////

pub fn deserialize_as_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where D: de::Deserializer<'de>
{
    // define a visitor that deserializes String to f64
    struct F64Visitor;

    impl<'de> de::Visitor<'de> for F64Visitor {
        type Value = f64;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string containing f64 data")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: de::Error {
            // convert to f64
            Ok(serde_json::from_str(v).unwrap())
        }
    }
    // use our visitor to deserialize
    deserializer.deserialize_any(F64Visitor)
}

pub fn deserialize_as_maybe_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
    where D: de::Deserializer<'de>
{
    // define a visitor that deserializes String to f64
    struct F64Visitor;

    impl<'de> de::Visitor<'de> for F64Visitor {
        type Value = Option<f64>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a Option(string) containing f64 data")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: de::Error {
            // convert to f64
            Ok(Some(serde_json::from_str(v).unwrap()))
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E> where E: de::Error {
            Ok(None) // convert to none
        }
    }
    // use our visitor to deserialize
    deserializer.deserialize_any(F64Visitor)
}

pub fn deserialize_as_f32<'de, D>(deserializer: D) -> Result<f32, D::Error>
    where D: de::Deserializer<'de>
{
    // define a visitor that deserializes String to f32
    struct F32Visitor;

    impl<'de> de::Visitor<'de> for F32Visitor {
        type Value = f32;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string containing f32 data")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: de::Error {
            // convert to f32
            Ok(serde_json::from_str(v).unwrap())
        }
    }
    // use our visitor to deserialize
    deserializer.deserialize_any(F32Visitor)
}


pub fn deserialize_as_naive_date_time<'de, D>(deserializer: D) -> Result<chrono::NaiveDateTime, D::Error>
    where D: de::Deserializer<'de>
{
    // define a visitor that deserializes String or i64 to NaiveDateTime
    struct NaiveDateTimeVisitor;

    impl<'de> de::Visitor<'de> for NaiveDateTimeVisitor {
        type Value = chrono::NaiveDateTime;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string, i64, u64, or f64 containing timestamp data.")
        }

        fn visit_i64<E>(self, timestamp: i64) -> Result<Self::Value, E> where E: de::Error {
            // convert to NaiveDateTime
            assert!(timestamp > 1_000_000_000); // Binance timestamps are in milliseconds
            // check that you're not getting timestamps in seconds format.
            // otheriwse you'll overstate milliseconds
            let millisecs = (timestamp % 1000) * 1_000_000;
            // Multiply by 1_000_000 to get Nanoseconds for `from_timestamp(...)`
            // from_timestamp(secs: i64, nsecs: u32) -> NaiveDateTime
            Ok(chrono::NaiveDateTime::from_timestamp(timestamp / 1_000, millisecs as u32))
        }

        fn visit_u64<E>(self, timestamp: u64) -> Result<Self::Value, E> where E: de::Error {
            // convert to NaiveDateTime
            assert!(timestamp > 1_000_000_000); // Binance timestamps are in milliseconds
            let millisecs = (timestamp % 1000) * 1_000_000;
            Ok(chrono::NaiveDateTime::from_timestamp(timestamp as i64 / 1_000, millisecs as u32))
        }

        fn visit_f64<E>(self, timestamp: f64) -> Result<Self::Value, E> where E: de::Error {
            // convert to NaiveDateTime
            assert!(timestamp > 1_000_000_000.0); // Binance timestamps are in milliseconds
            let millisecs = ((timestamp as i64) % 1000) * 1_000_000;
            Ok(chrono::NaiveDateTime::from_timestamp(timestamp as i64 / 1_000, millisecs as u32))
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E> where E: de::Error {
            // convert to NaiveDateTime
            let timestamp: i64 = s.parse::<i64>().unwrap();
            assert!(timestamp > 1_000_000_000); // Binance timestamps are in milliseconds
            let millisecs = (timestamp % 1000) * 1_000_000;
            Ok(chrono::NaiveDateTime::from_timestamp(timestamp as i64 / 1_000, millisecs as u32))
        }
    }
    // use our visitor to deserialize
    deserializer.deserialize_any(NaiveDateTimeVisitor)
}

// let d: i64 = Deserialize::deserialize(input)?;
// let naive = chrono::NaiveDateTime::from_timestamp(d/1000, 0);
// // Create a normal DateTime from the NaiveDateTime
// let datetime = chrono::DateTime::from_utc(naive, chrono::Utc);
