use trading_sys::models::tickers::TickerDataInsert;
use trading_sys::{create_tickers, establish_connection_pg};

use std::time::Duration;

use actix::*;
use actix_web::ws;

pub struct TickerActor {
    pub client_writer: ws::ClientWriter,
}

impl Actor for TickerActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        // Start heartbeats otherwise server disconnects in 10 seconds
        self.hb(ctx);
    }

    fn stopped(&mut self, _: &mut Context<Self>) {
        // Stop application on disconnect
        System::current().stop();
    }
}

impl TickerActor {
    fn hb(&self, ctx: &mut Context<Self>) {
        ctx.run_later(std::time::Duration::new(1, 0), |act, ctx| {
            act.client_writer.pong("Heartbeat");
            act.hb(ctx); // client should check for a timeout here
        });
    }
}

impl StreamHandler<ws::Message, ws::ProtocolError> for TickerActor {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Context<Self>) {
        match msg {
            ws::Message::Text(txt) => {
                let ticker_data: TickerDataInsert =
                    serde_json::from_str::<TickerDataInsert>(&txt).unwrap();
                println!("{:?}", &ticker_data);
                let connection = establish_connection_pg();
                create_tickers(&connection, ticker_data);
            }
            ws::Message::Ping(ping) => self.client_writer.pong(&ping),
            ws::Message::Pong(pong) => self.client_writer.ping(&pong),
            ws::Message::Close(maybe_reason) => match maybe_reason {
                Some(reason) => println!("{:?}", reason),
                None => println!("`ws::Message::Close(?)` with no reason provided."),
            },
            _ => (),
        }
    }

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("<ticker.rs>: Websocket Connected.");
    }

    fn finished(&mut self, ctx: &mut Context<Self>) {
        println!("<ticker.rs>: Websocket Stopped.");
        ctx.stop()
    }
}
