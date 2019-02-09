use chrono::NaiveDateTime;
use std::fmt;
use std::time::Duration;

use trading_sys::models::book_depth::{BookDepthDataInsert, DepthLevels, PartialBookDepthData};
use trading_sys::{create_book_depth, establish_connection_pg};

use actix::*;
use actix_web::ws;

pub struct BookDepthActor {
    pub client_writer: ws::ClientWriter,
    pub depth_levels: Option<DepthLevels>,
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
}

/// Handle Websocket messages
impl StreamHandler<ws::Message, ws::ProtocolError> for BookDepthActor {
    fn handle(&mut self, msg: ws::Message, _ctx: &mut Context<Self>) {
        match msg {
            ws::Message::Text(txt) => match &self.depth_levels {
                None => {
                    let book_depth_data: BookDepthDataInsert = serde_json::from_str(&txt).unwrap();

                    println!("{:?}", &book_depth_data);
                    let connection = establish_connection_pg();
                    create_book_depth(&connection, book_depth_data);
                }
                Some(lvl) => {
                    let partial_book: PartialBookDepthData =
                        serde_json::from_str::<PartialBookDepthData>(&txt).unwrap();
                    println!(
                        "Partial Book Depth Streams.\nDepth Lvl:{}\n{:#}",
                        lvl, partial_book
                    );
                }
            },
            ws::Message::Ping(ping) => self.client_writer.pong(&ping),
            ws::Message::Pong(pong) => self.client_writer.ping(&pong),
            _ => (),
        }
    }

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("<book_depth.rs>: Websocket Connected.");
    }

    fn finished(&mut self, ctx: &mut Context<Self>) {
        println!("<book_depth.rs>: Websocket Disconnected.");
        ctx.stop()
    }
}
