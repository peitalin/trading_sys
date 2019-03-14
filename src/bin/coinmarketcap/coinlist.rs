
use serde::de;
use std::fmt;


pub fn build_coinlist(coinlist_number: i32) -> Vec<CmcCoinMetadata> {
    //! Builds the list of coinst to scrape for coinmarketcap
    //! Each request only returns 100 entries, so need to build
    //! a list if we want to scrape data for more than 100 coins.
    let (start_slice, limit_slice) = build_starts_and_limits(coinlist_number);
    let mut coinlist: Vec<CmcCoinMetadata> = vec![];

    for zip in start_slice.iter().zip(limit_slice.iter()) {
        let (&start, &limit) = zip;
        coinlist.append(&mut get_coinslist(start, limit ));
    }

    coinlist
}


pub fn build_starts_and_limits(n: i32) -> (Vec<i32>, Vec<i32>) {
    let start_slice = (0..n).step_by(100).collect::<Vec<i32>>();
    let mut limit_slice = start_slice.iter().map(|_| 100).collect::<Vec<i32>>();
    // replace last limit with n % 100 = remainder
    limit_slice.pop();
    limit_slice.push(n % 100);
    (start_slice, limit_slice)
}


pub fn get_coinslist(start: i32, limit: i32) -> Vec<CmcCoinMetadata> {
    let url = format!("https://api.coinmarketcap.com/v1/ticker/?start={}&limit={}", start, limit);
    // Maximum number of entries returned in one request is limit: 100
    println!("Requesting: {:?}", &url);
    let parsed_url = reqwest::Url::parse(&url).expect("Bad url format.");
    let mut response = reqwest::get(parsed_url).expect("Failed to get Url");
    let bodyjson = response.json::<Vec<CmcCoinMetadata>>().unwrap();
    // let body = response.text().unwrap();
    // let bodyjson = serde_json::from_str::<Vec<CoinmarketcapAllcoins>>(&body);
    bodyjson
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CmcCoinMetadata {
    pub id: Option<String>,
    pub name: Option<String>,
    pub symbol: Option<String>,
    #[serde(deserialize_with="deserialize_as_f64")]
    pub rank: Option<f64>,
    #[serde(deserialize_with="deserialize_as_f64")]
    pub price_usd: Option<f64>,
    #[serde(deserialize_with="deserialize_as_f64")]
    pub price_btc: Option<f64>,
    #[serde(rename="24h_volume_usd")]
    #[serde(deserialize_with="deserialize_as_f64")]
    pub volume_usd_24h: Option<f64>,
    #[serde(deserialize_with="deserialize_as_f64")]
    pub market_cap_usd: Option<f64>,
    #[serde(deserialize_with="deserialize_as_f64")]
    pub available_supply: Option<f64>,
    #[serde(deserialize_with="deserialize_as_f64")]
    pub total_supply: Option<f64>,
    #[serde(deserialize_with="deserialize_as_f64")]
    pub max_supply: Option<f64>,
    #[serde(deserialize_with="deserialize_as_f64")]
    pub percent_change_1h: Option<f64>,
    #[serde(deserialize_with="deserialize_as_f64")]
    pub percent_change_24h: Option<f64>,
    #[serde(deserialize_with="deserialize_as_f64")]
    pub percent_change_7d: Option<f64>,
    pub last_updated: Option<String>,
}



fn deserialize_as_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
    where D: de::Deserializer<'de>
{
    // define a visitor that deserializes String to f64
    struct F64Visitor;

    impl<'de> de::Visitor<'de> for F64Visitor {
        type Value = Option<f64>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string containing f64 data")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where E: de::Error
        {
            // convert to f64
            Ok(Some(serde_json::from_str(v).unwrap()))
       }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
            where E: de::Error
        {
            // convert to none
            Ok(None)
        }
    }
    // use our visitor to deserialize
    deserializer.deserialize_any(F64Visitor)
}
