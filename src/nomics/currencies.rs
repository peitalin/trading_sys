


use std::fmt;
use serde::de;
use serde::de::{Deserialize, Deserializer, Error, SeqAccess, Unexpected, Visitor};


trait RequestData {
    fn request_data(&self, url: &str) -> reqwest::Response {
        let parsed_url = reqwest::Url::parse(&url).expect("Bad url format.");
        let mut response = reqwest::get(parsed_url).expect("failed to parse url.");
        response
    }
}
impl RequestData for CurrenciesApi {}
impl RequestData for PricesApi {}
impl RequestData for CurrenciesIntervalApi {}
impl RequestData for CurrenciesSparklineApi {}
impl RequestData for SuppliesIntervalApi {}
impl RequestData for AllTimeHighsApi {}


#[derive(Debug)]
pub struct CurrenciesApi {
    pub url: String,
    pub data: Vec<Currencies>,
}
impl CurrenciesApi {
    pub fn new(api_key: String) -> Self {
        CurrenciesApi {
            url: format!("https://api.nomics.com/v1/currencies?key={}", api_key),
            data: vec![],
        }
    }
    pub fn parse_and_set_data(&mut self) {
        let mut response = self.request_data(&self.url);
        let json_data = response.json::<Vec<Currencies>>().expect("Json parsing failed for Currencies");
        self.data = json_data;
    }
}


pub struct PricesApi {
    pub url: String,
    pub data: Vec<Prices>,
}
impl PricesApi {
    pub fn new(api_key: String) -> Self {
        PricesApi {
            url: format!("https://api.nomics.com/v1/prices?key={}", api_key),
            data: Vec::new(),
        }
    }
    pub fn parse_and_set_data(&mut self) {
        let mut response = self.request_data(&self.url);
        let json_data = response.json::<Vec<Prices>>().expect("Json parsing failed for Prices");
        self.data = json_data;
    }
}


pub struct CurrenciesIntervalApi {
    pub url: String,
    pub data: Vec<CurrenciesInterval>,
}
impl CurrenciesIntervalApi {
    pub fn new(api_key: String) -> Self {
        CurrenciesIntervalApi {
            url: format!("https://api.nomics.com/v1/currencies/interval?key={}&start=2018-04-14T00%3A00%3A00Z&end=2018-05-14T00%3A00%3A00Z", api_key),
            data: Vec::new(),
        }
    }
    pub fn parse_and_set_data(&mut self) {
        let mut response = self.request_data(&self.url);
        let json_data = response.json::<Vec<CurrenciesInterval>>().expect("Json parsing failed for Prices");
        self.data = json_data;
    }
}


pub struct CurrenciesSparklineApi {
    pub url: String,
    pub data: Vec<CurrenciesSparkline>,
}
impl CurrenciesSparklineApi {
    pub fn new(api_key: String) -> Self {
        CurrenciesSparklineApi {
            url: format!("https://api.nomics.com/v1/currencies/sparkline?key={}&start=2018-04-14T01%3A01%3A01Z&end=2018-05-14T02%3A02%3A00Z", api_key),
            data: vec![],
        }
    }
    pub fn parse_and_set_data(&mut self) {
        let mut response = self.request_data(&self.url);
        let json_data = response.json::<Vec<CurrenciesSparkline>>().expect("Json parsing failed for CurrenciesSparkline");
        self.data = json_data;
    }
}


pub struct SuppliesIntervalApi {
    pub url: String,
    pub data: Vec<SuppliesInterval>,
}

impl SuppliesIntervalApi {
    pub fn new(api_key: String) -> Self {
        SuppliesIntervalApi {
            url: format!("https://api.nomics.com/v1/supplies/interval?key={}&start=2018-04-14T00%3A00%3A00Z&end=2018-05-14T00%3A00%3A00Z", api_key),
            data: vec![],
        }
    }
    pub fn parse_and_set_data(&mut self) {
        let mut response = self.request_data(&self.url);
        let json_data = response.json::<Vec<SuppliesInterval>>().expect("Json parsing failed for SuppliesInterval");
        self.data = json_data;
    }
}

pub struct AllTimeHighsApi {
    pub url: String,
    pub data: Vec<AllTimeHighs>,
}
impl AllTimeHighsApi {
    pub fn new(api_key: String) -> Self {
        AllTimeHighsApi {
            url: format!("https://api.nomics.com/v1/currencies/highs?key={}", api_key),
            data: vec![],
        }
    }
    pub fn parse_and_set_data(&mut self) {
        let mut response = self.request_data(&self.url);
        let json_data = response.json::<Vec<AllTimeHighs>>().expect("Json parsing failed for AllTimeHighs");
        self.data = json_data;
    }
}





#[derive(Debug, Serialize, Deserialize)]
pub struct Currencies {
    pub id: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Prices {
    pub currency: String,
    #[serde(deserialize_with="deserialize_as_f64")]
    pub price: f64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrenciesInterval {
    pub currency: String,
    #[serde(deserialize_with="deserialize_as_f64")]
    pub volume: f64,
    #[serde(deserialize_with="deserialize_as_f64")]
    pub open: f64,
    pub open_timestamp: String,
    #[serde(deserialize_with="deserialize_as_f64")]
    pub close: f64,
    pub close_timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrenciesSparkline {
    pub currency: String,
    #[serde(deserialize_with="deserialize_as_vec_timestamp")]
    pub timestamps: Vec<chrono::NaiveDateTime>,
    #[serde(deserialize_with="deserialize_as_vec_f64")]
    pub prices: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuppliesInterval {
    pub currency: String,
    #[serde(deserialize_with="deserialize_as_maybe_f64")]
    pub open_available: Option<f64>,
    #[serde(deserialize_with="deserialize_as_maybe_f64")]
    pub open_max: Option<f64>,
    pub open_timestamp: Option<String>,
    #[serde(deserialize_with="deserialize_as_maybe_f64")]
    pub close_available: Option<f64>,
    #[serde(deserialize_with="deserialize_as_maybe_f64")]
    pub close_max: Option<f64>,
    pub close_timestamp: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllTimeHighs {
    pub currency: String,
    #[serde(deserialize_with="deserialize_as_f64")]
    pub price: f64,
    pub timestamp: String,
    pub exchange: String,
    pub quote: String
}





///////////////////////////////////////////////////////////////////////////
/// Deserializers
///////////////////////////////////////////////////////////////////////////

fn deserialize_as_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
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

fn deserialize_as_maybe_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
    where D: de::Deserializer<'de>
{
    // define a visitor that deserializes String to f64
    struct F64Visitor;

    impl<'de> de::Visitor<'de> for F64Visitor {
        type Value = Option<f64>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string containing f64 data")
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

fn deserialize_as_vec_f64<'de, D>(deserializer: D) -> Result<Vec<f64>, D::Error> where D: de::Deserializer<'de>
{
    // define a visitor that deserializes String to f64
    struct VecF64Visitor;

    impl<'de> de::Visitor<'de> for VecF64Visitor {
        type Value = Vec<f64>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string containing Vec<f64> data")
        }

        fn visit_seq<V>(self, mut visitor: V) -> Result<Vec<f64>, V::Error>
            where V: SeqAccess<'de>,
        {
            let mut vec = Vec::new();

            loop {
                match visitor.next_element()? {
                    Some(Value(elem)) => vec.push(elem), // elem is deserialized to f64
                    None => break,
                }
            }

            Ok(vec)
        }

    }
    // use our visitor to deserialize
    deserializer.deserialize_any(VecF64Visitor)
}

fn deserialize_as_vec_timestamp<'de, D>(deserializer: D) -> Result<Vec<chrono::NaiveDateTime>, D::Error>
where D: de::Deserializer<'de>
{
    // define a visitor that deserializes String to f64
    struct VecTimeVisitor;

    impl<'de> de::Visitor<'de> for VecTimeVisitor {
        type Value = Vec<chrono::NaiveDateTime>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string containing Vec<chrono::NaiveDateTime> data")
        }

        fn visit_seq<V>(self, mut visitor: V) -> Result<Vec<chrono::NaiveDateTime>, V::Error>
            where V: SeqAccess<'de>,
        {
            let mut vec = Vec::new();

            loop {
                match visitor.next_element()? {
                    Some(Time(elem)) => {
                        // let dd = chrono::NaiveDateTime::parse_from_str("2019-01-01T00:00:00Z", "%Y-%m-%dT%H:%M:%SZ").unwrap();
                        vec.push(elem); // elem has been deserialized to NaiveDateTime
                    },
                    None => break,
                }
            }

            Ok(vec)
        }

    }
    // use our visitor to deserialize
    deserializer.deserialize_any(VecTimeVisitor)
}







struct Inner(Vec<f64>);

impl<'de> Deserialize<'de> for Inner {
    fn deserialize<D>(deserializer: D) -> Result<Inner, D::Error>
    where D: Deserializer<'de>,
    {
        struct InnerVisitor;

        impl<'de> Visitor<'de> for InnerVisitor {
            type Value = Inner;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a nonempty sequence of numbers")
            }

            #[inline]
            fn visit_seq<V>(self, mut visitor: V) -> Result<Inner, V::Error>
            where V: SeqAccess<'de>,
            {
                let mut vec = Vec::new();

                while let Some(Value(elem)) = visitor.next_element()? {
                    vec.push(elem);
                }

                Ok(Inner(vec))
            }
        }

        deserializer.deserialize_seq(InnerVisitor)
    }
}

struct Value(f64);

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
    where D: Deserializer<'de>
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        s.parse().map(Value).map_err(|_| {
            D::Error::invalid_value(Unexpected::Str(&s), &"a floating point number as a string")
        })
    }
}

struct Time(chrono::NaiveDateTime);

impl<'de> Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Time, D::Error>
    where D: Deserializer<'de>
    {
        let d: String = Deserialize::deserialize(deserializer)?;
        let date = chrono::NaiveDateTime::parse_from_str(&d, "%Y-%m-%dT%H:%M:%SZ").unwrap();
        Ok(Time(date))
    }
}

// #[derive(Debug, Deserialize)]
// struct Payload {
//     #[serde(default, deserialize_with = "from_array_of_arrays_of_strs")]
//     values: Vec<Vec<f64>>,
// }
//
// fn from_array_of_arrays_of_strs<'de, D>(deserializer: D) -> Result<Vec<Vec<f64>>, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     struct OuterVisitor;
//
//     impl<'de> Visitor<'de> for OuterVisitor {
//         type Value = Vec<Vec<f64>>;
//
//         fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//             formatter.write_str("a nonempty sequence of a sequence of numbers")
//         }
//
//         #[inline]
//         fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
//         where
//             V: SeqAccess<'de>,
//         {
//             let mut vec = Vec::new();
//
//             while let Some(Inner(elem)) = visitor.next_element()? {
//                 vec.push(elem);
//             }
//
//             Ok(vec)
//         }
//     }
//
//     deserializer.deserialize_seq(OuterVisitor)
// }


// pub fn main() {
//     let input = r#"
// {
//   "values": [["2", "1.4"], ["8.32", "1.5"]]
// }
// "#;
//
//     let out: Payload = serde_json::from_str(input).unwrap();
//
//     println!("{:?}", out);
// }
