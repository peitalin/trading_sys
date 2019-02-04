use chrono::NaiveDateTime;
use std::fmt;
use std::time::Duration;

use trading_sys::currency_pairs::CurrencyPair;
use trading_sys::serde_parsers::{deserialize_as_f64, deserialize_as_naive_date_time};

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
                // let timestamp: i64 = 1549193380333;
                // let millisecs = (timestamp % 1000) * 1_000_000 ;
                // println!("TEST TIMESTAMP: {:?}", chrono::NaiveDateTime::from_timestamp(timestamp / 1_000, millisecs as u32));

                let kline_data: KlineMetaData =
                    serde_json::from_str::<KlineMetaData>(&txt).unwrap();

                // let connection = trading_sys::establish_connection();
                // trading_sys::create_post(&connection, &kline_data.symbol, &txt);
                println!("{}\n", kline_data);
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

#[derive(Debug, Serialize, Deserialize)]
pub struct KlineMetaData {
    #[serde(rename = "e")]
    pub event: String, // Event type
    #[serde(rename = "E")]
    #[serde(deserialize_with = "deserialize_as_naive_date_time")]
    pub event_time: NaiveDateTime, // Event time
    #[serde(rename = "s")]
    pub symbol: String, // Symbol
    #[serde(rename = "k")]
    pub kline_data: KlineData, // Trade ID
}

impl fmt::Display for KlineMetaData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KlineData {
    #[serde(rename = "t")]
    #[serde(deserialize_with = "deserialize_as_naive_date_time")]
    pub start_time: NaiveDateTime, // Kline start time
    #[serde(rename = "T")]
    #[serde(deserialize_with = "deserialize_as_naive_date_time")]
    pub close_time: NaiveDateTime, // Kline close time
    #[serde(rename = "s")]
    pub symbol: CurrencyPair, // Symbol
    #[serde(rename = "i")]
    pub interval: String, // Kline Intervel
    #[serde(rename = "f")]
    pub first_trade_id: i32, // First trade ID
    #[serde(rename = "L")]
    pub last_trade_id: i32, // Last trade ID
    #[serde(rename = "o")]
    #[serde(deserialize_with = "deserialize_as_f64")]
    pub open: f64, // Open price
    #[serde(rename = "c")]
    #[serde(deserialize_with = "deserialize_as_f64")]
    pub close: f64, // Close price
    #[serde(rename = "h")]
    #[serde(deserialize_with = "deserialize_as_f64")]
    pub high: f64, // High price
    #[serde(rename = "l")]
    #[serde(deserialize_with = "deserialize_as_f64")]
    pub low: f64, // Low price
    #[serde(rename = "v")]
    #[serde(deserialize_with = "deserialize_as_f64")]
    pub volume: f64, // Volume
    #[serde(rename = "n")]
    pub num_of_trades: i32, // Number of trades
    #[serde(rename = "x")]
    pub is_kline_closed: bool, // Is this Kline closed?
    #[serde(rename = "q")]
    #[serde(deserialize_with = "deserialize_as_f64")]
    pub quote_asset_vol: f64, // Quote asset volume
    #[serde(rename = "V")]
    #[serde(deserialize_with = "deserialize_as_f64")]
    pub taker_buy_base_vol: f64, // Taker buy base asset volume
    #[serde(rename = "Q")]
    #[serde(deserialize_with = "deserialize_as_f64")]
    pub taker_buy_quote_vol: f64, // Taker buy quote asset volume
}

impl fmt::Display for KlineData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pretty_json = serde_json::to_string_pretty(&self).unwrap();
        write!(f, "{:#}", pretty_json)
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
