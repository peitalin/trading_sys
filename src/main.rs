
#![allow(unused_variables)]
extern crate chrono;
extern crate regex;
extern crate clap;
extern crate scraper;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate strum_macros;
extern crate strum;

#[macro_use]
extern crate env_logger;

extern crate reqwest;

// mod coinmarketcap;
// mod nomics;
mod binance;
mod serde_parsers;



extern crate ring;
extern crate data_encoding;

use std::fmt;
use ring::{ digest, hmac };




fn main() -> std::io::Result<()> {

    // Setup logging
    env_logger::init();
    let connection = "wss://stream.binance.com:9443/ws/ethbtc@depth";

    // binance::main(connection);

    //
    // let url = "https://api.binance.com/wapi/v3/depositHistory.html";
    //
    // let SECRET_KEY = std::env::var("BINANCE_SECRET_KEY").expect("No env BINANCE_SECRET_KEY set");
    // let API_KEY = std::env::var("BINANCE_API_KEY").expect("No env BINANCE_API_KEY set");
    //
    // let now = chrono::Utc::now().timestamp_millis();
    // let query = format!("timestamp={}", now);
    //
    //
    // let signed_key = hmac::SigningKey::new(&digest::SHA256, SECRET_KEY.as_bytes());
    // let signature = hmac::sign(&signed_key, query.as_bytes());
    //
    //
    // // println!("signature:  {:?}", signature);
    // // println!("Sha256hash: Signature(SHA256:{})", HexDigest(signature));
    // let url_full = format!("{}?{}&signature={}", url, query, HexDigest(signature));
    // println!("url_full: {:?}\n", url_full);
    //
    //
    // let mut response = reqwest::Client::new()
    //     .get(&url_full)
    //     .header("X-MBX-APIKEY", API_KEY)
    //     .send().unwrap();
    //
    //
    // let mut resp_json = &response.json::<depositHistoryResponse>().unwrap();
    //
    // println!("Response:\n{}", resp_json);
    // // println!("response: {:?}", &response.text().unwrap());


    Ok(())
}


#[derive(Debug)]
pub struct HexDigest(ring::hmac::Signature);

impl fmt::Display for HexDigest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sig_as_ref: &[u8] = self.0.as_ref();
        let hex_digest = data_encoding::HEXLOWER.encode(sig_as_ref);
        write!(f, "{}", hex_digest)
    }
}

impl fmt::Display for depositHistoryResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pretty_json = serde_json::to_string_pretty(&self).unwrap();
        write!(f, "{}", pretty_json)
    }
}


#[derive(Debug, Deserialize, Serialize)]
struct depositAddressResponse {
    address: String,
    success: bool,
    addressTag: Option<String>,
    asset: String,
}


#[derive(Debug, Deserialize, Serialize)]
struct depositHistoryResponse {
    success: bool,
    depositList: Vec<depositHistoryItem>,
}

#[derive(Debug, Deserialize, Serialize)]
struct depositHistoryItem {
    insertTime: u64,
    amount: f64,
    address: String,
    addressTag: Option<String>,
    txId: String,
    asset: String,
    status: u64,
}





