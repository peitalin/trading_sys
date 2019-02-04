

#![allow(unused_variables)]
extern crate chrono;
extern crate clap;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

use actix::*;
use actix_web::ws;
use futures::Future;

extern crate trading_sys;
use trading_sys::currency_pairs::{ CurrencyPair, CurrencyPrice, CurrencyBase };

pub mod book_depth;
use book_depth::{ BookDepthActor, ClientCommand };

pub mod aggregate_trade;
use aggregate_trade::{ AggregateTradeActor };

pub mod trade;
use trade::{ TradeActor };

pub mod kline;
use kline::{ KlineActor, KlineInterval };




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
    // spawn_aggregate_trade_client(CurrencyPair::ETHBTC);
    // spawn_book_depth_client(CurrencyPair::ETHBTC);
    spawn_trade_client(CurrencyPair::ETHBTC);
    // spawn_kline_client(CurrencyPair::ETHBTC, KlineInterval::_1m);
    let _ = sys.run();

    // let connection = establish_connection_pg();
    // let results = posts.filter(published.eq(true))
    //     .limit(5)
    //     .load::<Post>(&connection)
    //     .expect("Error loading posts");
    //
    // println!("Displaying {} posts", results.len());
    // for post in results {
    //     println!("\n{}", post.title);
    //     println!("-------\n");
    //     println!("{}\n", post.body);
    // }

}


pub fn spawn_book_depth_client(currency_pair: CurrencyPair) {

    let ws_url = format!("wss://stream.binance.com:9443/ws/{}@depth", currency_pair);
    println!("Endpoint: {}", ws_url);

    actix::Arbiter::spawn(
        ws::Client::new(ws_url)              // Instantiate ws client  -> ws::Client
        .connect()                           // Do websocket handshake -> ws::ClientHandshake
        .map_err(|e| panic!("Error: {}", e)) // requires use futures::Future;
        .map(|(reader, writer): (ws::ClientReader, ws::ClientWriter)| {
            // create an actor
            let addr = BookDepthActor::create(|ctx| {
                BookDepthActor::add_stream(reader, ctx);
                BookDepthActor { client_writer: writer }
            });
        })
    );
}


pub fn spawn_aggregate_trade_client(currency_pair: CurrencyPair) {

    let ws_url = format!("wss://stream.binance.com:9443/ws/{}@aggTrade", currency_pair);
    println!("Endpoint: {}", ws_url);

    actix::Arbiter::spawn(
        ws::Client::new(ws_url)              // Instantiate ws client  -> ws::Client
        .connect()                           // Do websocket handshake -> ws::ClientHandshake
        .map_err(|e| panic!("Error: {}", e)) // requires use futures::Future;
        .map(|(reader, writer): (ws::ClientReader, ws::ClientWriter)| {
            // create an actor
            let addr = AggregateTradeActor::create(|ctx| {
                AggregateTradeActor::add_stream(reader, ctx);
                AggregateTradeActor { client_writer: writer }
            });
            // spawn a new thread and console loop for new actor
            std::thread::spawn(move || loop {
                let mut cmd = String::new();
                if std::io::stdin().read_line(&mut cmd).is_err() {
                    println!("error");
                    return;
                }
                addr.do_send(aggregate_trade::ClientCommand(cmd));
            });
        })
    );
}


pub fn spawn_trade_client(currency_pair: CurrencyPair) {

    let ws_url = format!("wss://stream.binance.com:9443/ws/{}@trade", currency_pair);
    println!("Endpoint: {}", ws_url);

    actix::Arbiter::spawn(
        ws::Client::new(ws_url)              // Instantiate ws client  -> ws::Client
        .connect()                           // Do websocket handshake -> ws::ClientHandshake
        .map_err(|e| panic!("Error: {}", e)) // requires use futures::Future;
        .map(|(reader, writer): (ws::ClientReader, ws::ClientWriter)| {
            // create an actor
            let addr = TradeActor::create(|ctx| {
                TradeActor::add_stream(reader, ctx);
                TradeActor { client_writer: writer }
            });
        })
    );
}


fn spawn_kline_client(currency_pair: CurrencyPair, interval: KlineInterval) {

    let ws_url = format!("wss://stream.binance.com:9443/ws/{}@kline_{}", currency_pair, interval);
    println!("Endpoint: {}", ws_url);

    actix::Arbiter::spawn(
        ws::Client::new(ws_url)              // Instantiate ws client  -> ws::Client
        .connect()                           // Do websocket handshake -> ws::ClientHandshake
        .map_err(|e| panic!("Error: {}", e)) // requires use futures::Future;
        .map(|(reader, writer): (ws::ClientReader, ws::ClientWriter)| {
            // create an actor
            let addr = KlineActor::create(|ctx| {
                KlineActor::add_stream(reader, ctx);
                KlineActor { client_writer: writer }
            });
        })
    );
}


