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
    let sys = actix::System::new("ws-binance");

    // spawn_aggregate_trade_client(CurrencyPair::ETHBTC);

    // spawn_book_depth_client(CurrencyPair::ETHBTC, Some(DepthLevels::_10));
    // spawn_book_depth_client(CurrencyPair::ETHBTC, None);

    spawn_trade_client(CurrencyPair::ETHBTC);

    // spawn_kline_client(CurrencyPair::ETHBTC, KlineInterval::_1m);

    // spawn_mini_ticker_client(CurrencyPair::ETHBTC);
    // spawn_mini_ticker_client(CurrencyPair::ETHBTC, Some(MiniTickerQueryType::AllMarkets));

    // spawn_ticker_client(CurrencyPair::ETHBTC);

    // get_book_depth_from_postgres();
    // get_klines_from_postgres();
    // get_trades_from_postgres();
    // get_aggregate_trades_from_postgres();

    let _ = sys.run();
}

pub fn get_all_base_pairs() {
    let url = "https://api.binance.com/api/v3/ticker/price";
    let mut jsond: Vec<CurrencyPrice> = reqwest::get(url)
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
