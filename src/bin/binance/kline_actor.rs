use chrono::NaiveDateTime;
use std::fmt;
use std::time::Duration;

use trading_sys::currency_pairs::CurrencyPair;
use trading_sys::serde_parsers::{deserialize_as_f64, deserialize_as_naive_date_time};
use trading_sys::models::kline::{ KlineData, KlineMetaData };

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
        println!("Disconnected");
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

    fn handle_ping(&mut self, ctx: &mut Context<Self>, ping: String) {
        println!("{:?}", ws::Message::Ping(ping));
        self.client_writer.pong("Pong from KlineActor");
        // self.hb(ctx)
        // client should check for a timeout here, similar to server code
    }
}

#[derive(Message)]
pub struct ClientCommand(pub String);

/// Handle stdin commands
impl Handler<ClientCommand> for KlineActor {
    type Result = ();

    fn handle(&mut self, command: ClientCommand, ctx: &mut Context<Self>) {
        self.client_writer.text(command.0)
    }
}

/// Handle Websocket messages
impl StreamHandler<ws::Message, ws::ProtocolError> for KlineActor {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Context<Self>) {
        match msg {
            ws::Message::Text(txt) => {
                let kline_meta_data: KlineMetaData =
                    serde_json::from_str::<KlineMetaData>(&txt).unwrap();

                let connection = trading_sys::establish_connection_pg();
                println!("{}\n", &kline_meta_data);
                trading_sys::create_kline(&connection, kline_meta_data.kline_data);
            }
            ws::Message::Ping(ping) => {
                ctx.run_later(Duration::new(0, 0), |act, ctx| act.handle_ping(ctx, ping));
            }
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


#[derive(Debug, Deserialize, Serialize)]
pub enum KlineInterval {
    _1m,
    _3m,
    _5m,
    _15m,
    _30m,
    _1h,
    _2h,
    _4h,
    _6h,
    _8h,
    _12h,
    _1d,
    _3d,
    _1w,
    _1M,
}
impl std::fmt::Display for KlineInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            KlineInterval::_1m => write!(f, "1m"),
            KlineInterval::_3m => write!(f, "3m"),
            KlineInterval::_5m => write!(f, "5m"),
            KlineInterval::_15m => write!(f, "15m"),
            KlineInterval::_30m => write!(f, "30m"),
            KlineInterval::_1h => write!(f, "1h"),
            KlineInterval::_2h => write!(f, "2h"),
            KlineInterval::_4h => write!(f, "4h"),
            KlineInterval::_6h => write!(f, "6h"),
            KlineInterval::_8h => write!(f, "8h"),
            KlineInterval::_12h => write!(f, "12h"),
            KlineInterval::_1d => write!(f, "1d"),
            KlineInterval::_3d => write!(f, "3d"),
            KlineInterval::_1w => write!(f, "1w"),
            KlineInterval::_1M => write!(f, "1M"),
        }
    }
}

