use actix::*;
use actix_web::ws;
use futures::Future;

use trading_sys::currency_pairs::{CurrencyBase, CurrencyPair, CurrencyPrice};
use trading_sys::models::klines::KlineInterval;

use crate::actors::book_depth::BookDepthActor;
use crate::actors::aggregate_trade::AggregateTradeActor;
use crate::actors::trades::TradeActor;
use crate::actors::klines::KlineActor;
use crate::actors::mini_ticker::MiniTickerActor;



pub fn binance_api_url(query: String) -> String {
    let api_url = "wss://stream.binance.com:9443/ws/";
    format!("{api_url}{query}", api_url = api_url, query = query)
}


/////////////////////////////////////////////////////////////////
/// Spawn new Actor scraper clients
/////////////////////////////////////////////////////////////////

pub fn spawn_book_depth_client(currency_pair: CurrencyPair) {
    let ws_url = binance_api_url(format!("{}@depth", currency_pair));
    println!("Endpoint: {}", ws_url);

    actix::Arbiter::spawn(
        ws::Client::new(ws_url) // Instantiate ws client  -> ws::Client
            .connect() // Do websocket handshake -> ws::ClientHandshake
            .map_err(|e| panic!("Error: {}", e)) // requires use futures::Future;
            .map(|(reader, writer): (ws::ClientReader, ws::ClientWriter)| {
                // create an actor
                let addr: actix::Addr<BookDepthActor> = BookDepthActor::create(|ctx| {
                    BookDepthActor::add_stream(reader, ctx);
                    BookDepthActor {
                        client_writer: writer,
                    }
                });
            })
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
                let addr: actix::Addr<AggregateTradeActor> = AggregateTradeActor::create(|ctx| {
                    AggregateTradeActor::add_stream(reader, ctx);
                    AggregateTradeActor {
                        client_writer: writer,
                    }
                });
            })
    );
}

pub fn spawn_trade_client(currency_pair: CurrencyPair) {

    use crate::actors::trades::ClientCommand;

    let ws_url = binance_api_url(format!("{}@trade", currency_pair));
    println!("Endpoint: {}", ws_url);

    actix::Arbiter::spawn(
        ws::Client::new(ws_url) // Instantiate ws client  -> ws::Client
            .connect() // Do websocket handshake -> ws::ClientHandshake
            .map_err(|e| panic!("Error: {}", e)) // requires use futures::Future;
            .map(|stream| {
                let (reader, writer): (ws::ClientReader, ws::ClientWriter) = stream;
                // create an actor
                let addr: actix::Addr<TradeActor> = TradeActor::create(|ctx: &mut Context<TradeActor>| {
                    TradeActor::add_stream(reader, ctx);
                    TradeActor {
                        client_writer: writer,
                    }
                });

                // spawn a new thread and console loop for new actor
                std::thread::spawn(move || loop {
                    let mut cmd = String::new();
                    if std::io::stdin().read_line(&mut cmd).is_err() {
                        println!("error");
                        return;
                    }
                    addr.do_send(ClientCommand(cmd));
                });
            })
    );
}

pub fn spawn_kline_client(currency_pair: CurrencyPair, interval: KlineInterval) {


    let ws_url = binance_api_url(format!("{}@kline_{}", currency_pair, interval));
    println!("Endpoint: {}", ws_url);

    actix::Arbiter::spawn(
        ws::Client::new(ws_url) // Instantiate ws client  -> ws::Client
            .connect() // Do websocket handshake -> ws::ClientHandshake
            .map_err(|e| panic!("Error: {}", e)) // requires use futures::Future;
            .map(|(reader, writer): (ws::ClientReader, ws::ClientWriter)| {
                // create an actor
                let addr: actix::Addr<KlineActor> = KlineActor::create(|ctx| {
                    KlineActor::add_stream(reader, ctx);
                    KlineActor {
                        client_writer: writer,
                    }
                });
            })
    );
}

pub fn spawn_mini_ticker_client(currency_pair: CurrencyPair) {
    let ws_url = binance_api_url(format!("{}@miniTicker", currency_pair));
    println!("Endpoint: {}", ws_url);

    actix::Arbiter::spawn(
        ws::Client::new(ws_url) // Instantiate ws client  -> ws::Client
            .connect() // Do websocket handshake -> ws::ClientHandshake
            .map_err(|e| panic!("Error: {}", e)) // requires use futures::Future;
            .map(|stream| {
                let (reader, writer): (ws::ClientReader, ws::ClientWriter) = stream;
                // create an actor
                let addr: actix::Addr<MiniTickerActor> = MiniTickerActor::create(|ctx: &mut Context<MiniTickerActor>| {
                    MiniTickerActor::add_stream(reader, ctx);
                    MiniTickerActor {
                        client_writer: writer,
                    }
                });
            })
    );
}
