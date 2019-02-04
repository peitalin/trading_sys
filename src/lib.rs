#![allow(unused_variables)]
extern crate chrono;
extern crate clap;
extern crate regex;
extern crate scraper;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate strum_macros;
extern crate strum;

#[macro_use]
extern crate env_logger;
extern crate reqwest;

extern crate data_encoding;
extern crate ring;

extern crate actix;
extern crate actix_web;
extern crate futures;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate redis;

// pub mod coinmarketcap;
pub mod currency_pairs;
pub mod models;
pub mod schema;
pub mod serde_parsers;

use crate::models::TradeDataInsert;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn establish_connection_pg() -> PgConnection {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_trade<'a>(conn: &PgConnection, trade_data: &TradeDataInsert) {
    use crate::schema::trades;
    use diesel::prelude::*; // .get_result trait

    let res = diesel::insert_into(trades::table)
        .values(trade_data)
        .execute(conn);

    // let res: TradeData = diesel::insert_into(trade::table)
    //     .values(trade_data)
    //     .get_result(conn)
    //     .expect("Error saving new trade");

    println!("Database write result: {:?}\n", res);
}
