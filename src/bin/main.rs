
#![allow(unused_variables)]
extern crate chrono;
extern crate clap;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate env_logger;
extern crate reqwest;

extern crate ring;
extern crate data_encoding;

extern crate redis;

extern crate trading_sys;

use std::fmt;
use ring::{ digest, hmac };



fn main() -> std::io::Result<()> {

    // Setup logging
    env_logger::init();

    // // The AP endpoint. The address where the datafeed can be accessed via the web
    // let url = "https://api.binance.com/wapi/v3/depositHistory.html";
    // // The query/message we send to the API endpoint to tell them what data we want.
    // let query = format!("timestamp={}", chrono::Utc::now().timestamp_millis());
    // // We sign the query with my BINANCE_SECRET_KEY for security
    // let url_full = sign_query(url, &query);
    // // Another BINANCE_API_KEY which matches BINANCE_SECRET_KEY, so they know the request is really
    // // coming from me, not some one else pretending to be me.
    // let api_key = std::env::var("BINANCE_API_KEY").expect("No <BINANCE_API_KEY> environment variable set.");
    //
    // // Send a http request to the API with the query + signature
    // let mut response = reqwest::Client::new()
    //     .get(&url_full)
    //     .header("X-MBX-APIKEY", api_key)
    //     .send().unwrap();
    //
    // let resp_json = &response.json::<DepositHistoryResponse>().unwrap();
    //
    // println!("Response:\n{}", resp_json);
    // // println!("response: {:?}", &response.text().unwrap());
    // Ok(())

    // std::process::Command::new("redis-server")
    //     .output()
    //     .expect("redis-server not installed properly");

    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let conn = client.get_connection().unwrap();
    redis_get(&conn);

    Ok(())

}




fn redis_get(conn: &redis::Connection) -> redis::RedisResult<()> {
    let _ : () = redis::cmd("SET").arg("my_key").arg(42).query(conn).unwrap();
    let user2: String = redis::cmd("GET").arg("user2").query(conn).unwrap();
    let user3: String = redis::cmd("GET").arg("user3").query(conn).unwrap();
    println!("Redis: user2: {}", user2);
    println!("Redis: user3: {}", user3);
    Ok(())
}


pub fn sign_query(url: &str, query_string: &str) -> String {

    let secret_key = std::env::var("BINANCE_SECRET_KEY").expect("No <BINANCE_SECRET_KEY> environment variable set.");
    let signing_key = hmac::SigningKey::new(&digest::SHA256, secret_key.as_bytes());
    let signature = hmac::sign(&signing_key, query_string.as_bytes());
    println!("signature:  {:?}", signature);
    // println!("Sha256hash: Signature(SHA256:{})", HexDigest(signature));
    let url_full = format!("{}?{}&signature={}", url, query_string, HexDigest(signature));
    url_full
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

impl fmt::Display for DepositHistoryResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pretty_json = serde_json::to_string_pretty(&self).unwrap();
        write!(f, "{}", pretty_json)
    }
}


#[derive(Debug, Deserialize, Serialize)]
struct DepositAddressResponse {
    address: String,
    success: bool,
    address_tag: Option<String>,
    asset: String,
}


#[derive(Debug, Deserialize, Serialize)]
struct DepositHistoryResponse {
    success: bool,
    deposit_list: Vec<DepositHistoryItem>,
}

#[derive(Debug, Deserialize, Serialize)]
struct DepositHistoryItem {
    insert_time: u64,
    amount: f64,
    address: String,
    address_tag: Option<String>,
    tx_id: String,
    asset: String,
    status: u64,
}





