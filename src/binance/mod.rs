
extern crate actix;
extern crate actix_web;
extern crate futures;

use actix::*;
use actix_web::ws;
use futures::Future;

mod BookDepthStreams;
use BookDepthStreams::BookDepthClient;





pub fn main(CONNECTION: &str) {

    let sys = actix::System::new("ws-binance");

    actix::Arbiter::spawn(
        ws::Client::new(CONNECTION)
        .connect()
        .map_err(|e| panic!("Error: {}", e))
        .map(|(reader, writer): (ws::ClientReader, ws::ClientWriter)| {
            // create an actor
            let addr = BookDepthClient::create(|ctx| {
                BookDepthClient::add_stream(reader, ctx);
                BookDepthClient(writer)
            });
        }),
    );


    let _ = sys.run();

}

