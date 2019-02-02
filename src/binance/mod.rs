
extern crate actix;
extern crate actix_web;
extern crate futures;


use actix::*;
use actix_web::ws;
use futures::Future;

mod book_depth;
use book_depth::{ BookDepthActor, ClientCommand };

mod aggregate_trade;
use aggregate_trade::{ AggregateTradeActor };

mod currency_pairs;
use currency_pairs::{ CurrencyPair, Price };

mod trade;
use trade::{ TradeActor };





pub fn main(CONNECTION: &str) {

    // let url = "https://api.binance.com/api/v3/ticker/price";
    // let mut jsond = reqwest::get(url).unwrap()
    //     .json::<Vec<Price>>().unwrap();
    //
    // for p in &jsond[..10] {
    //     println!("{}", &p.symbol);
    //     // println!("{}: url: {}", &p.symbol, format!("wss://stream.binance.com:9443/ws/{}@depth", p.symbol));
    // }


    let sys = actix::System::new("ws-binance");
    // spawn_aggregate_trade_client(CurrencyPair::ETHBTC);
    // spawn_book_depth_client(CurrencyPair::ETHBTC);
    spawn_trade_client(CurrencyPair::ETHBTC);
    let _ = sys.run();

}


pub fn spawn_book_depth_client(CurrencyPair: CurrencyPair) {

    let ws_url = format!("wss://stream.binance.com:9443/ws/{}@depth", CurrencyPair);
    println!("Endpoint: {}", ws_url);

    actix::Arbiter::spawn(
        ws::Client::new(ws_url)              // Instantiate ws client  -> ws::Client
        .connect()                           // Do websocket handshake -> ws::ClientHandshake
        .map_err(|e| panic!("Error: {}", e)) // requires use futures::Future;
        .map(|(reader, writer): (ws::ClientReader, ws::ClientWriter)| {
            // create an actor
            let addr = BookDepthActor::create(|ctx| {
                BookDepthActor::add_stream(reader, ctx);
                BookDepthActor { clientWriter: writer }
            });
        })
    );
}


pub fn spawn_aggregate_trade_client(CurrencyPair: CurrencyPair) {

    let ws_url = format!("wss://stream.binance.com:9443/ws/{}@aggTrade", CurrencyPair);
    println!("Endpoint: {}", ws_url);

    actix::Arbiter::spawn(
        ws::Client::new(ws_url)              // Instantiate ws client  -> ws::Client
        .connect()                           // Do websocket handshake -> ws::ClientHandshake
        .map_err(|e| panic!("Error: {}", e)) // requires use futures::Future;
        .map(|(reader, writer): (ws::ClientReader, ws::ClientWriter)| {
            // create an actor
            let addr = AggregateTradeActor::create(|ctx| {
                AggregateTradeActor::add_stream(reader, ctx);
                AggregateTradeActor { clientWriter: writer }
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


pub fn spawn_trade_client(CurrencyPair: CurrencyPair) {

    let ws_url = format!("wss://stream.binance.com:9443/ws/{}@trade", CurrencyPair);
    println!("Endpoint: {}", ws_url);

    actix::Arbiter::spawn(
        ws::Client::new(ws_url)              // Instantiate ws client  -> ws::Client
        .connect()                           // Do websocket handshake -> ws::ClientHandshake
        .map_err(|e| panic!("Error: {}", e)) // requires use futures::Future;
        .map(|(reader, writer): (ws::ClientReader, ws::ClientWriter)| {
            // create an actor
            let addr = TradeActor::create(|ctx| {
                TradeActor::add_stream(reader, ctx);
                TradeActor { clientWriter: writer }
            });
        })
    );
}



