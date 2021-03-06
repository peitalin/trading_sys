use serde::de;
use serde::de::{Deserialize, Deserializer, Error, SeqAccess, Unexpected, Visitor};
use serde_derive::*;
use std::fmt;

///////////////////////////////////////////////////////////////////////////
/// Deserializers
///////////////////////////////////////////////////////////////////////////

pub fn deserialize_as_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: de::Deserializer<'de>,
{
    // define a visitor that deserializes String to f64
    struct F64Visitor;

    impl<'de> de::Visitor<'de> for F64Visitor {
        type Value = f64;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string containing f64 data")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            // convert to f64
            Ok(serde_json::from_str(v).unwrap())
        }
    }
    // use our visitor to deserialize
    deserializer.deserialize_any(F64Visitor)
}

pub fn deserialize_as_maybe_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: de::Deserializer<'de>,
{
    // define a visitor that deserializes String to f64
    struct F64Visitor;

    impl<'de> de::Visitor<'de> for F64Visitor {
        type Value = Option<f64>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a Option(string) containing f64 data")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            // convert to f64
            Ok(Some(serde_json::from_str(v).unwrap()))
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None) // convert to none
        }
    }
    // use our visitor to deserialize
    deserializer.deserialize_any(F64Visitor)
}

pub fn deserialize_as_f32<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: de::Deserializer<'de>,
{
    // define a visitor that deserializes String to f32
    struct F32Visitor;

    impl<'de> de::Visitor<'de> for F32Visitor {
        type Value = f32;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string containing f32 data")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            // convert to f32
            Ok(serde_json::from_str(v).unwrap())
        }
    }
    // use our visitor to deserialize
    deserializer.deserialize_any(F32Visitor)
}

pub fn deserialize_as_naive_date_time_ms<'de, D>(
    deserializer: D,
) -> Result<chrono::NaiveDateTime, D::Error>
where
    D: de::Deserializer<'de>,
{
    // define a visitor that deserializes String or i64 to NaiveDateTime
    struct NaiveDateTimeVisitor;

    impl<'de> de::Visitor<'de> for NaiveDateTimeVisitor {
        type Value = chrono::NaiveDateTime;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string, i64, u64, or f64 containing timestamp data.")
        }

        fn visit_i64<E>(self, timestamp: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(create_timestamp_ms(timestamp))
        }

        fn visit_u64<E>(self, timestamp: u64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(create_timestamp_ms(timestamp))
        }

        fn visit_f64<E>(self, timestamp: f64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(create_timestamp_ms(timestamp))
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            let timestamp: i64 = s.parse::<i64>().unwrap();
            Ok(create_timestamp_ms(timestamp))
        }
    }
    // use our visitor to deserialize
    deserializer.deserialize_any(NaiveDateTimeVisitor)
}

use num::cast::{FromPrimitive, ToPrimitive};
use std::ops::{AddAssign, Div};

fn create_timestamp_ms<T>(_timestamp: T) -> chrono::NaiveDateTime
where
    T: Div + ToPrimitive + AddAssign + Default,
{
    // convert to NaiveDateTime
    let timestamp: i64 = _timestamp.to_i64().unwrap();
    // Timestamps should much much larger than 10 digits
    match timestamp {
        0...1_000_000_000_000 => {
            // Timestamp of 1_000_000_000 is 14th July 2017 2:40:00 AM
            // Binance Launched on 14th July 2017.
            panic!("Timestamp too small: {}.\n Timestamp format may be in seconds instead of milliseconds", timestamp)
        }
        1_000_000_000_000...1_500_000_000_000 => {
            // Timestamp of 1_500_000_000 is 14th July 2017 2:40:00 AM
            // Binance Launched on 14th July 2017.
            warn!("Timestamp {} appears before Binance launched on the 14-Jul-2017.", timestamp);
            let ms = (timestamp % 1000) * 1_000_000;
            chrono::NaiveDateTime::from_timestamp(timestamp / 1_000, ms as u32)
        }
        _ => {
            // Timestamp are in milliseconds
            let ms = (timestamp % 1000) * 1_000_000;
            // get remainder in milliseconds, convert to nanoseconds
            // as from_timestamp takes nanoseconds in the 2nd argument
            chrono::NaiveDateTime::from_timestamp(timestamp / 1_000, ms as u32)
            // first argument is seconds, second argument is in nanoseconds
        }
    }
}

pub fn create_timestamp_benchmark(sec: i64) -> chrono::NaiveDateTime {
    let ms = (sec % 1000) * 1_000_000;
    let t_benchmark = chrono::NaiveDateTime::from_timestamp(sec as i64 / 1_000 as i64, ms as u32);
    t_benchmark
}



///////////////// Tests ///////////////
use proptest::prelude::*;

#[cfg(test)]
mod tests {

    use crate::serde_parsers::create_timestamp_benchmark;
    use crate::serde_parsers::deserialize_as_naive_date_time_ms;

    #[derive(Deserialize)]
    struct Mock_Json_Timestamp {
        #[serde(deserialize_with = "deserialize_as_naive_date_time_ms")]
        json_time: chrono::NaiveDateTime,
    }

    #[test]
    fn try_naive_date_time_str() {
        let _t1 = r#"{ "json_time": "1222333444555" }"#;
        let t1: Mock_Json_Timestamp = serde_json::from_str::<Mock_Json_Timestamp>(&_t1).unwrap();
        let t_benchmark = create_timestamp_benchmark(1_222_333_444_555);
        assert_eq!(t1.json_time, t_benchmark)
    }

    #[test]
    fn try_naive_date_time_int() {
        let _t1 = r#"{ "json_time": 1222333444555 }"#;
        let t1: Mock_Json_Timestamp = serde_json::from_str::<Mock_Json_Timestamp>(&_t1).unwrap();
        let t_benchmark = create_timestamp_benchmark(1_222_333_444_555);
        assert_eq!(t1.json_time, t_benchmark)
    }

    #[test]
    fn try_naive_date_time_float() {
        let _t1 = r#"{ "json_time": 1222333444555.0 }"#;
        let t1: Mock_Json_Timestamp = serde_json::from_str::<Mock_Json_Timestamp>(&_t1).unwrap();
        let t_benchmark = create_timestamp_benchmark(1_222_333_444_555);
        assert_eq!(t1.json_time, t_benchmark)
    }

    proptest! {
        #[test]
        fn try_random_timestamps(n in 1_000_000_000_000..2_000_000_000_000 as i64) {
            // 1_000_000_000 is 2001-09-09
            let _t1 = format!(r#"{{ "json_time": {} }}"#, n);
            let t1: Mock_Json_Timestamp = serde_json::from_str::<Mock_Json_Timestamp>(&_t1).unwrap();
            let t_benchmark = create_timestamp_benchmark(n);
            assert_eq!(t1.json_time, t_benchmark)
        }

        #[test]
        #[should_panic]
        fn should_fail_on_small_timestamps(n in 0..1_000_000_000 as i64) {
            let _t1 = format!(r#"{{ "json_time": {} }}"#, n);
            let t1: Mock_Json_Timestamp = serde_json::from_str::<Mock_Json_Timestamp>(&_t1).unwrap();
            let t_benchmark = create_timestamp_benchmark(n);
            assert_eq!(t1.json_time, t_benchmark)
        }
    }

}
