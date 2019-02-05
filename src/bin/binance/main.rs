#![allow(unused_variables)]
extern crate chrono;
extern crate clap;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use actix::*;
use actix_web::ws;
use futures::Future;

extern crate trading_sys;
use trading_sys::currency_pairs::{CurrencyBase, CurrencyPair, CurrencyPrice};

pub mod book_depth_actor;
use book_depth_actor::{BookDepthActor, ClientCommand};

pub mod aggregate_trade_actor;
use aggregate_trade_actor::AggregateTradeActor;

pub mod trade_actor;
use trade_actor::TradeActor;

pub mod kline_actor;
use kline_actor::{KlineActor, KlineInterval};



pub fn binance_api_url(query: String) -> String {
    let api_url = "wss://stream.binance.com:9443/ws/";
    format!("{api_url}{query}", api_url = api_url, query = query)
}


pub fn main() {
    // let url = "https://api.binance.com/api/v3/ticker/price";
    // let mut jsond: Vec<CurrencyPrice> = reqwest::get(url).unwrap()
    //     .json::<Vec<CurrencyPrice>>().unwrap();
    //
    // let fitler: Vec<CurrencyPrice> = jsond.into_iter()
    //     .filter(|x| x.symbol.filter_base_pair(CurrencyBase::ETH)).collect();
    // println!("{:?}\nOnly ETH base pairs", fitler);

    // for p in &jsond {
    //     match p.symbol {
    //         CurrencyPair::ETHBTC => println!("{:?}", p.symbol),
    //         _ => continue,
    //     }
    //     // println!("{}: url: {}", &p.symbol, format!("wss://stream.binance.com:9443/ws/{}@depth", p.symbol));
    // }

    let sys = actix::System::new("ws-binance");

    get_book_depth_from_postgres();

    // spawn_aggregate_trade_client(CurrencyPair::ETHBTC);
    // spawn_book_depth_client(CurrencyPair::ETHBTC);
    // spawn_trade_client(CurrencyPair::ETHBTC);
    // spawn_kline_client(CurrencyPair::ETHBTC, KlineInterval::_1m);

    let _ = sys.run();

}

fn get_book_depth_from_postgres() {
    use diesel::prelude::*;
    use trading_sys::models::BookDepthDataQuery;
    use trading_sys::schema::book_depth::dsl::*; // .get_result trait

    let connection = trading_sys::establish_connection_pg();

    println!("{:?}", book_depth);
    let results = book_depth
        .limit(6)
        .load::<BookDepthDataQuery>(&connection)
        .expect("Error in deserializing Jsonb to BookDepthDataQuery.");

    for trade_result in results {
        println!("\n{}", trade_result.event_time);
        println!(
            "${}: {:?} {:?}",
            trade_result.symbol.to_uppercase(),
            trade_result.asks,
            trade_result.bids
        );
        println!("-------\n");
    }
}

fn get_trades_from_postgres() {
    use diesel::prelude::*;
    use trading_sys::models::TradeData;
    use trading_sys::schema::trades::dsl::*; // .get_result trait

    let connection = trading_sys::establish_connection_pg();

    let results = trades
        .filter(quantity.gt(1.0))
        .limit(5)
        .load::<TradeData>(&connection)
        .expect("Error loading posts");

    println!(
        "Displaying {} trades, each greater than 1.0 ETH",
        results.len()
    );

    for trade_result in results {
        println!("\n{}", trade_result.event_time);
        println!(
            "${}: {} {}",
            trade_result.price,
            trade_result.quantity,
            trade_result.symbol.to_uppercase()
        );
        println!("-------\n");
    }
}


pub fn spawn_book_depth_client(currency_pair: CurrencyPair) {

    let ws_url = binance_api_url(format!("{}@depth", currency_pair));
    println!("Endpoint: {}", ws_url);

    actix::Arbiter::spawn(
        ws::Client::new(ws_url) // Instantiate ws client  -> ws::Client
            .connect() // Do websocket handshake -> ws::ClientHandshake
            .map_err(|e| panic!("Error: {}", e)) // requires use futures::Future;
            .map(|(reader, writer): (ws::ClientReader, ws::ClientWriter)| {
                // create an actor
                let addr = BookDepthActor::create(|ctx| {
                    BookDepthActor::add_stream(reader, ctx);
                    BookDepthActor {
                        client_writer: writer,
                    }
                });
            }),
    );
}

pub fn spawn_aggregate_trade_client(currency_pair: CurrencyPair) {

    let ws_url = binance_api_url(format!("{}@aggTrade", currency_pair));
    println!("Endpoint: {}", ws_url);

    actix::Arbiter::spawn(
        ws::Client::new(ws_url) // Instantiate ws client  -> ws::Client
            .connect() // Do websocket handshake -> ws::ClientHandshake
            .map_err(|e| panic!("Error: {}", e)) // requires use futures::Future;
            .map(|(reader, writer): (ws::ClientReader, ws::ClientWriter)| {
                // create an actor
                let addr = AggregateTradeActor::create(|ctx| {
                    AggregateTradeActor::add_stream(reader, ctx);
                    AggregateTradeActor {
                        client_writer: writer,
                    }
                });
                // // spawn a new thread and console loop for new actor
                // std::thread::spawn(move || loop {
                //     let mut cmd = String::new();
                //     if std::io::stdin().read_line(&mut cmd).is_err() {
                //         println!("error");
                //         return;
                //     }
                //     addr.do_send(aggregate_trade_actor::ClientCommand(cmd));
                // });
            }),
    );
}

pub fn spawn_trade_client(currency_pair: CurrencyPair) {

    let ws_url = binance_api_url(format!("{}@trade", currency_pair));
    println!("Endpoint: {}", ws_url);

    actix::Arbiter::spawn(
        ws::Client::new(ws_url) // Instantiate ws client  -> ws::Client
            .connect() // Do websocket handshake -> ws::ClientHandshake
            .map_err(|e| panic!("Error: {}", e)) // requires use futures::Future;
            .map(|(reader, writer): (ws::ClientReader, ws::ClientWriter)| {
                // create an actor
                let addr = TradeActor::create(|ctx| {
                    TradeActor::add_stream(reader, ctx);
                    TradeActor {
                        client_writer: writer,
                    }
                });
            }),
    );
}

fn spawn_kline_client(currency_pair: CurrencyPair, interval: KlineInterval) {

    let ws_url = binance_api_url(format!("{}@kline_{}", currency_pair, interval));
    println!("Endpoint: {}", ws_url);

    actix::Arbiter::spawn(
        ws::Client::new(ws_url) // Instantiate ws client  -> ws::Client
            .connect() // Do websocket handshake -> ws::ClientHandshake
            .map_err(|e| panic!("Error: {}", e)) // requires use futures::Future;
            .map(|(reader, writer): (ws::ClientReader, ws::ClientWriter)| {
                // create an actor
                let addr = KlineActor::create(|ctx| {
                    KlineActor::add_stream(reader, ctx);
                    KlineActor {
                        client_writer: writer,
                    }
                });
            }),
    );
}
