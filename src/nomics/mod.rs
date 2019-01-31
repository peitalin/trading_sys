

// NOMICS_API_KEY

use std::error::Error;
use std::fmt;

mod currencies;
use currencies::CurrenciesSparkline;



pub fn nomics_main() {
    let api_key = get_api_key().unwrap();


    let mut nomics = currencies::CurrenciesSparklineApi::new(api_key);
    nomics.parse_and_set_data();
    let data: Vec<CurrenciesSparkline> = nomics.data;

    for d in &data {
        for zip in d.timestamps.iter().zip(d.prices.iter()) {
            let (&timestamp, &price) = zip;
            println!("{}\t{:?}\t${:?}", &d.currency, timestamp.date(), price);
        }
    }



}



fn get_api_key() -> Result<String, NomicsError> {
    let key = "NOMICS_API_KEY";
    match std::env::var(key) {
        Ok(val) => return Ok(val),
        Err(e) => return Err(NomicsError::new("Error getting NOMICS_API_KEY, please set environment variable")),
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct NomicsMarkets {
    exchange: String,
    market: String,
    base: String,
    quote: String,
}



#[derive(Debug)]
pub struct NomicsError {
    details: String
}

impl NomicsError {
    pub fn new(msg: &str) -> Self {
        NomicsError { details: msg.to_string() }
    }
}

impl fmt::Display for NomicsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for NomicsError {
    fn description(&self) -> &str {
        &self.details
    }
}

