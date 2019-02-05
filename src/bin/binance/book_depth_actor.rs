use chrono::NaiveDateTime;
use std::fmt;
use std::time::Duration;

use trading_sys::models::BookDepthData;
use trading_sys::{create_book_depth, establish_connection_pg};

use trading_sys::currency_pairs::CurrencyPair;
use trading_sys::serde_parsers::deserialize_as_naive_date_time;

use actix::*;
use actix_web::ws;

pub struct BookDepthActor {
    pub client_writer: ws::ClientWriter,
}

impl Actor for BookDepthActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        // Start heartbeats otherwise server disconnects in 10 seconds
        self.hb(ctx);
    }

    fn stopped(&mut self, _: &mut Context<Self>) {
        println!("Disconnected");
        // Stop application on disconnect
        System::current().stop();
    }
}

impl BookDepthActor {
    fn hb(&self, ctx: &mut Context<Self>) {
        ctx.run_later(std::time::Duration::new(1, 0), |act, ctx| {
            act.client_writer.pong("Heartbeat");
            act.hb(ctx);
            // client should also check for a timeout here, similar to the
            // server code
        });
    }

    fn handle_ping(&mut self, ctx: &mut Context<Self>, ping: String) {
        println!("{:?}", ws::Message::Ping(ping));
        self.client_writer.pong("Pong from BookDepthActor");
        // self.hb(ctx)
        // client should check for a timeout here, similar to server code
    }
}
// fn handle_ping<A: Actor>(act: &mut BookDepthActor, ctx: &mut Context<BookDepthActor>, ping: String)
//     where A: Actor + 'static
// {
//     println!("{:?}", ws::Message::Ping(ping));
//     act.client_writer.pong("Pong from BookDepthActor");
//     act.hb(ctx)
//     // client should check for a timeout here, similar to server code
// }

#[derive(Message)]
pub struct ClientCommand(pub String);

/// Handle stdin commands
impl Handler<ClientCommand> for BookDepthActor {
    type Result = ();

    fn handle(&mut self, command: ClientCommand, ctx: &mut Context<Self>) {
        self.client_writer.text(command.0)
    }
}

/// Handle Websocket messages
impl StreamHandler<ws::Message, ws::ProtocolError> for BookDepthActor {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Context<Self>) {
        match msg {
            ws::Message::Text(txt) => {
                let book_depth_data: BookDepthData = serde_json::from_str::<BookDepthData>(&txt).unwrap();
                let connection = establish_connection_pg();
                create_book_depth(&connection, &book_depth_data);
                println!("{:?}", book_depth_data);
            }
            ws::Message::Ping(ping) => {
                ctx.run_later(Duration::new(0, 0), |act, ctx| act.handle_ping(ctx, ping));
            }
            _ => (),
        }
    }

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("<book_depth.rs>: Websocket Connected.");
    }

    fn finished(&mut self, ctx: &mut Context<Self>) {
        println!("<book_depth.rs>: Websocket Disconnected.");
        ctx.stop()
    }
}


