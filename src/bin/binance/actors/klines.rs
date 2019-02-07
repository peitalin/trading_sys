use chrono::NaiveDateTime;
use std::fmt;
use std::time::Duration;

use trading_sys::currency_pairs::CurrencyPair;
use trading_sys::serde_parsers::{deserialize_as_f64, deserialize_as_naive_date_time};
use trading_sys::models::klines::{ KlineMetaData, KlineInterval, KlineDataInsert };

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
                let kline_raw_data = kline_meta_data.kline_data;
                let connection = trading_sys::establish_connection_pg();
                let new_kline_data = KlineDataInsert {
                    event:          kline_meta_data.event,
                    event_time:     kline_meta_data.event_time,
                    start_time:     kline_raw_data.start_time,
                    close_time:     kline_raw_data.close_time,
                    symbol:         kline_raw_data.symbol,
                    interval:       kline_raw_data.interval,
                    first_trade_id: kline_raw_data.first_trade_id,
                    last_trade_id:  kline_raw_data.last_trade_id,
                    open:           kline_raw_data.open,
                    close:          kline_raw_data.close,
                    high:           kline_raw_data.high,
                    low:            kline_raw_data.low,
                    volume:         kline_raw_data.volume,
                    num_of_trades:  kline_raw_data.num_of_trades,
                    is_kline_closed:     kline_raw_data.is_kline_closed,
                    quote_asset_vol:     kline_raw_data.quote_asset_vol,
                    taker_buy_base_vol:  kline_raw_data.taker_buy_base_vol,
                    taker_buy_quote_vol: kline_raw_data.taker_buy_quote_vol,
                };
                println!("{:?}\n", &new_kline_data);
                trading_sys::create_kline(&connection, new_kline_data);

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

