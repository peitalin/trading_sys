
#![allow(unused_imports)]

#[macro_use] extern crate log;
#[macro_use] extern crate env_logger;

extern crate chrono;
extern crate clap;
extern crate regex;
extern crate scraper;

extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

#[macro_use] extern crate strum_macros;
extern crate strum;

#[macro_use] extern crate proptest;

extern crate data_encoding;
extern crate ring;

extern crate reqwest;
extern crate actix;
extern crate actix_web;
extern crate futures;

#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate redis;
extern crate uuid;

// pub mod coinmarketcap;
pub mod currency_pairs;
pub mod models;
pub mod schema;
pub mod serde_parsers;

use crate::models::aggregate_trades::AggregateTradeData;
use crate::models::book_depth::{BookDepthData, BookDepthDataInsert};
use crate::models::klines::KlineDataInsert;
use crate::models::trades::TradeData;
use crate::models::mini_ticker::MiniTickerDataInsert;
use crate::models::tickers::TickerDataInsert;

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

pub fn create_tickers<'a>(conn: &PgConnection, ticker_data: TickerDataInsert) {
    use crate::schema::tickers; // DB table name
    use diesel::prelude::*;

    let res = diesel::insert_into(tickers::table)
        .values(ticker_data)
        .execute(conn);

    println!("Database write result: {:?}\n", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    use diesel::connection::Connection;
    use diesel::result::Error;

    #[test]
    fn db_mini_ticker_postgres_write() {
        use crate::models::mini_ticker::MiniTickerDataInsert;
        use crate::models::mini_ticker::TEST_MINI_TICKER_DATA;
        use crate::schema::mini_tickers; // DB table name

        let test_data = serde_json::from_str::<MiniTickerDataInsert>(TEST_MINI_TICKER_DATA).unwrap();

        let conn: PgConnection = establish_connection_pg();
        conn.test_transaction::<_, Error, _>(|| {
            diesel::insert_into(mini_tickers::table)
                .values(test_data)
                .execute(&conn)
        });
    }

    #[test]
    fn db_tickers_postgres_write() {
        use crate::models::tickers::TickerDataInsert;
        use crate::models::tickers::TEST_TICKER_DATA;
        use crate::schema::tickers; // DB table name

        let test_data = serde_json::from_str::<TickerDataInsert>(TEST_TICKER_DATA).unwrap();

        let conn: PgConnection = establish_connection_pg();
        conn.test_transaction::<_, Error, _>(|| {
            diesel::insert_into(tickers::table)
                .values(test_data)
                .execute(&conn)
        });
    }

    #[test]
    fn db_klines_postgres_write() {
        use crate::models::klines::{
            map_klinemeta_to_klineinsertdata, KlineDataInsert, KlineMetaData, NewKlineData,
            TEST_KLINE_DATA,
        };
        use crate::schema::klines; // DB table name

        let kmeta_mdata = serde_json::from_str::<KlineMetaData>(TEST_KLINE_DATA).unwrap();
        let kline_data_insert = map_klinemeta_to_klineinsertdata(kmeta_mdata);

        let conn: PgConnection = establish_connection_pg();
        conn.test_transaction::<_, Error, _>(|| {
            diesel::insert_into(klines::table)
                .values(kline_data_insert)
                .execute(&conn)
        });
    }

    #[test]
    fn db_trades_postgres_write() {
        use crate::models::trades::{
            TEST_TRADE_DATA,
            TradeData
        };
        use crate::schema::trades; // DB table name

        let test_data = serde_json::from_str::<TradeData>(TEST_TRADE_DATA).unwrap();

        let conn: PgConnection = establish_connection_pg();
        conn.test_transaction::<_, Error, _>(|| {
            diesel::insert_into(trades::table)
                .values(test_data)
                .execute(&conn)
        });
    }

    #[test]
    fn db_aggregate_trades_postgres_write() {
        use crate::models::aggregate_trades::{
            TEST_AGGTRADE_DATA,
            AggregateTradeData
        };
        use crate::schema::aggregate_trades; // DB table name

        let test_data = serde_json::from_str::<AggregateTradeData>(TEST_AGGTRADE_DATA).unwrap();

        let conn: PgConnection = establish_connection_pg();
        conn.test_transaction::<_, Error, _>(|| {
            diesel::insert_into(aggregate_trades::table)
                .values(test_data)
                .execute(&conn)
        });
    }

    #[test]
    fn db_bookdepth_postgres_write() {
        use crate::models::book_depth::{
            TEST_BOOKDEPTH_DATA,
            BookDepthDataInsert
        };
        use crate::schema::book_depth; // DB table name

        let test_data = serde_json::from_str::<BookDepthDataInsert>(TEST_BOOKDEPTH_DATA).unwrap();

        let conn: PgConnection = establish_connection_pg();
        conn.test_transaction::<_, Error, _>(|| {
            diesel::insert_into(book_depth::table)
                .values(test_data)
                .execute(&conn)
        });
    }
}
