extern crate chrono;
extern crate clap;

#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate serde_json;

extern crate trading_sys;

pub mod actors;

pub mod spawn_clients;
use spawn_clients::{
    binance_api_url,
    spawn_aggregate_trade_client,
    spawn_book_depth_client,
    spawn_kline_client,
    spawn_mini_ticker_client,
    spawn_trade_client,
    spawn_ticker_client,
};

pub mod db_actions;
use db_actions::{
    get_aggregate_trades_from_postgres,
    get_book_depth_from_postgres,
    get_klines_from_postgres,
    get_trades_from_postgres,
    get_tickers_from_postgres,
};

use trading_sys::currency_pairs::{CurrencyBase, CurrencyPair, CurrencyPrice};
use trading_sys::models::book_depth::DepthLevels;
use trading_sys::models::klines::KlineInterval;
use trading_sys::models::mini_ticker::MiniTickerQueryType;



pub fn main() {

    let currencies: Vec<CurrencyPair> = vec![
        CurrencyPair::ETHBTC,
        CurrencyPair::ETHUSDT,
        CurrencyPair::BNBETH,
        CurrencyPair::LINKETH,
        CurrencyPair::XLMETH,
        CurrencyPair::XMRETH,
        CurrencyPair::ZILETH,
    ];


    // let sys = actix::System::new("ws-binance");

    // spawn_aggregate_trade_client(CurrencyPair::ETHBTC);

    // spawn_book_depth_client(CurrencyPair::ETHBTC, Some(DepthLevels::_10));
    // spawn_book_depth_client(CurrencyPair::ETHBTC, None);

    // for currency in currencies.into_iter() {
    //     spawn_kline_client(&currency, KlineInterval::_1m);
    //     spawn_trade_client(&currency);
    //     spawn_ticker_client(&currency);
    // }

    // spawn_trade_client(CurrencyPair::ETHBTC);
    //
    // spawn_kline_client(CurrencyPair::ETHBTC, KlineInterval::_1m);
    //
    // spawn_mini_ticker_client(CurrencyPair::ETHBTC, Some(MiniTickerQueryType::SingleMarket));
    //
    // spawn_ticker_client(CurrencyPair::ETHBTC);

    get_book_depth_from_postgres();
    get_klines_from_postgres();
    get_trades_from_postgres();
    get_aggregate_trades_from_postgres();

    // let _ = sys.run();

    raw_sql_query();

}


pub fn raw_sql_query() {
    use diesel::prelude::*;
    use trading_sys::models::trades::TradeData;
    use trading_sys::schema::trades::dsl::*; // .get_result trait
    use diesel::sql_types::{Float, Numeric};

    let connection = trading_sys::establish_connection_pg();


    // let results = trades.select((price, quantity))
    //     .filter(symbol.eq("ETHBTC"))
    //     .load::<(f32, f32)>(&connection)
    //     .unwrap();

    let time_cutoff = chrono::NaiveDate::from_ymd(2019, 2, 11).and_hms(4, 38, 38);
    let results = trades
        .filter(event_time.gt(time_cutoff))
        .load::<TradeData>(&connection)
        .unwrap();


    // let results = diesel::sql_query("SELECT price FROM trades")
    //     .execute(&connection)
    //     .unwrap();


    // let results = trades
    //     .filter(quantity.gt(100))
    //     .select(price)
    //     .first(&connection)
    //     .unwrap();

    for r in results {
        println!("{:?}", r);
    }
    // println!("{:?}", results);

}

pub fn get_all_base_pairs() {
    let url = "https://api.binance.com/api/v3/ticker/price";
    let jsond: Vec<CurrencyPrice> = reqwest::get(url)
        .unwrap()
        .json::<Vec<CurrencyPrice>>()
        .unwrap();

    let filtered: Vec<CurrencyPrice> = jsond
        .into_iter()
        .filter(|x| x.symbol.filter_base_pair(CurrencyBase::ETH))
        .collect();
    println!("{:?}\nOnly ETH base pairs", &filtered);

    for p in &filtered {
        match p.symbol {
            CurrencyPair::ETHBTC => println!("{:?}", p.symbol),
            _ => continue,
        }
    }
}
