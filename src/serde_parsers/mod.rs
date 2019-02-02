

use std::fmt;
use serde::de;
use serde::de::{Deserialize, Deserializer, Error, SeqAccess, Unexpected, Visitor};

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


pub struct Time(pub chrono::NaiveDateTime);

impl<'de> Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Time, D::Error>
    where D: Deserializer<'de>
    {
        let d: String = Deserialize::deserialize(deserializer)?;
        let date = chrono::NaiveDateTime::parse_from_str(&d, "%Y-%m-%dT%H:%M:%SZ").unwrap();
        Ok(Time(date))
    }
}

/// Implement traits fors UtcTime data structure
/// Used for serde_json to deserialize Unix timestamps (int)
/// into chrono::DateTime types
#[derive(Debug, Serialize, Clone)]
pub struct UtcTime(chrono::DateTime<chrono::Utc>);

impl<'de> Deserialize<'de> for UtcTime {
    fn deserialize<D>(deserializer: D) -> Result<UtcTime, D::Error>
    where D: Deserializer<'de>
    {
        let d: i64 = Deserialize::deserialize(deserializer)?;
        let naive = chrono::NaiveDateTime::from_timestamp(d/1000, 0);
        // Create a normal DateTime from the NaiveDateTime
        let datetime = chrono::DateTime::from_utc(naive, chrono::Utc);
        // // Format the datetime how you want
        // let newdate = datetime.format("%Y-%m-%d %H:%M:%S");
        Ok(UtcTime(datetime))
    }
}

impl fmt::Display for UtcTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let datetime = self.0;
        let newdate = datetime.format("%Y-%m-%d %H:%M:%S");
        write!(f, "{}", newdate)
    }
}
