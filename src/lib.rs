#![allow(unused_imports)]
extern crate chrono;
extern crate clap;
extern crate regex;
extern crate scraper;

#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
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

extern crate uuid;

// pub mod coinmarketcap;
pub mod currency_pairs;
pub mod models;
pub mod schema;
pub mod serde_parsers;

use crate::models::aggregate_trades::AggregateTradeData;
use crate::models::book_depth::{BookDepthDataInsert, BookDepthData};
use crate::models::trades::TradeData;
use crate::models::klines::KlineDataInsert;
use crate::models::mini_ticker::MiniTickerDataInsert;

use diesel::pg::PgConnection;
use diesel::prelude::*;


pub fn establish_connection_pg() -> PgConnection {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_trade<'a>(conn: &PgConnection, trade_data: &TradeData) {
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

pub fn create_aggregate_trade<'a>(conn: &PgConnection, aggregate_trade_data: &AggregateTradeData) {
    use crate::schema::aggregate_trades; // DB table name
    use diesel::prelude::*; // .get_result trait

    let res = diesel::insert_into(aggregate_trades::table)
        .values(aggregate_trade_data)
        .execute(conn);

    println!("Database write result: {:?}\n", res);
}

pub fn create_book_depth<'a>(conn: &PgConnection, book_depth_data: BookDepthDataInsert) {
    use crate::schema::book_depth; // DB table name
    use diesel::prelude::*; // .get_result trait

    let res = diesel::insert_into(book_depth::table)
        .values(book_depth_data)
        .execute(conn);

    println!("Database write result: {:?}\n", res);
}

pub fn create_kline<'a>(conn: &PgConnection, kline_data: KlineDataInsert) {
    use crate::schema::klines; // DB table name
    use diesel::prelude::*;

    let res = diesel::insert_into(klines::table)
        .values(kline_data)
        .execute(conn);

    println!("Database write result: {:?}\n", res);
}

pub fn create_mini_tickers<'a>(conn: &PgConnection, mini_ticker_data: MiniTickerDataInsert) {
    use crate::schema::mini_tickers; // DB table name
    use diesel::prelude::*;

    let res = diesel::insert_into(mini_tickers::table)
        .values(mini_ticker_data)
        .execute(conn);

    println!("Database write result: {:?}\n", res);
}




#[cfg(test)]
mod tests {

    #[test]
    fn test_deserialization_mini_ticker() {
        use crate::models::mini_ticker::TEST_MINI_TICKER_DATA;
        use crate::models::mini_ticker::MiniTickerDataInsert;

        let jsond_test = serde_json::from_str::<MiniTickerDataInsert>(TEST_MINI_TICKER_DATA).unwrap();
        println!("test deserialization <mini_ticker>:\n{:?}", jsond_test);
        let mock_data = MiniTickerDataInsert {
            event: "24hrMiniTicker".to_owned(),
            event_time: chrono::NaiveDateTime::from_timestamp(1_222_333_444_555, 0),
            symbol: crate::currency_pairs::CurrencyPair::BNBBTC,
            close: 0.0025,
            open: 0.0010,
            high: 0.0025,
            low: 0.0010,
            base_asset_vol: 10000.0,
            quote_asset_vol: 18.0
        };
        println!("test deserialization <mini_ticker>:\n{:?}", jsond_test);
        assert_eq!(1,1)
    }

    // #[test]
    // fn test_actor() {
    //     use diesel::result::Error;
    //
    //     use crate::schema::mini_tickers; // DB table name
    //
    //     let conn = establish_connection_pg();
    //     conn.test_transaction::<_, Error, _>(|| {
    //         diesel::insert_into(mini_ticker_data)
    //             .values()
    //             .execute(&conn)?;
    //     })
    // }
}








