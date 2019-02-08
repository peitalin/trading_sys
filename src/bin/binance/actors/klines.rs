use chrono::NaiveDateTime;
use std::fmt;
use std::time::Duration;

use trading_sys::currency_pairs::CurrencyPair;
use trading_sys::serde_parsers::{deserialize_as_f64};
use trading_sys::models::klines::{
    KlineMetaData,
    KlineInterval,
    KlineDataInsert,
    map_klinemeta_to_klineinsertdata,
};

use actix::*;
use actix_web::ws;

pub struct KlineActor {
    pub client_writer: ws::ClientWriter,
}

impl Actor for KlineActor {
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

impl KlineActor {
    fn hb(&self, ctx: &mut Context<Self>) {
        ctx.run_later(std::time::Duration::new(1, 0), |act, ctx| {
            act.client_writer.pong("Heartbeat");
            act.hb(ctx);
            // client should check for a timeout here, similar to server code
        });
    }
}

/// Handle Websocket messages
impl StreamHandler<ws::Message, ws::ProtocolError> for KlineActor {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Context<Self>) {
        match msg {
            ws::Message::Text(txt) => {
                let kline_meta_data: KlineMetaData =
                    serde_json::from_str::<KlineMetaData>(&txt).unwrap();

                let kline_data_insert = map_klinemeta_to_klineinsertdata(kline_meta_data);
                let connection = trading_sys::establish_connection_pg();

                println!("{:?}\n", &kline_data_insert);
                trading_sys::create_kline(&connection, kline_data_insert);

            },
            ws::Message::Ping(ping) => self.client_writer.pong(&ping),
            ws::Message::Pong(pong) => self.client_writer.ping(&pong),
            _ => (),
        }
    }

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("<kline.rs>: Websocket Connected.");
    }

    fn finished(&mut self, ctx: &mut Context<Self>) {
        println!("<kline.rs>: Websocket Disconnected.");
        ctx.stop()
    }
}

