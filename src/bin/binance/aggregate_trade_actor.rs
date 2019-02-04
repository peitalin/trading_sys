use chrono::NaiveDateTime;
use std::fmt;
use std::time::Duration;

use trading_sys::serde_parsers::deserialize_as_f64;

use actix::*;
use actix_web::ws;

pub struct AggregateTradeActor {
    pub client_writer: ws::ClientWriter,
}

impl Actor for AggregateTradeActor {
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

impl AggregateTradeActor {
    fn hb(&self, ctx: &mut Context<Self>) {
        ctx.run_later(Duration::new(1, 0), |act, ctx| {
            act.client_writer.pong("Heartbeat");
            act.hb(ctx);
            // client should check for a timeout here, similar to server code
        });
    }

    fn handle_ping(&mut self, ctx: &mut Context<Self>, ping: String) {
        println!("{:?}", ws::Message::Ping(ping));
        self.client_writer.pong("Pong from AggregateTradeActor");
        // self.hb(ctx)
        // client should check for a timeout here, similar to server code
    }
}

#[derive(Message)]
pub struct ClientCommand(pub String);

/// Handle stdin commands
impl Handler<ClientCommand> for AggregateTradeActor {
    type Result = ();

    fn handle(&mut self, command: ClientCommand, ctx: &mut Context<Self>) {
        self.client_writer.text(command.0)
    }
}

/// Handle Websocket messages
impl StreamHandler<ws::Message, ws::ProtocolError> for AggregateTradeActor {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Context<Self>) {
        match msg {
            ws::Message::Text(txt) => {
                let aggregate_trade = serde_json::from_str::<AggregateTradeData>(&txt).unwrap();
                println!("{}", aggregate_trade);
            }
            ws::Message::Ping(ping) => {
                ctx.run_later(Duration::new(0, 0), |act, ctx| act.handle_ping(ctx, ping));
            }
            _ => (),
        }
    }

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("<aggregate_trade.rs>: Websocket Connected.");
    }

    fn finished(&mut self, ctx: &mut Context<Self>) {
        println!("<aggregate_trade.rs>: Websocket Disconnected.");
        ctx.stop()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AggregateTradeData {
    #[serde(rename = "e")]
    pub agg_trade: String, // Event type
    #[serde(rename = "E")]
    pub event_time: NaiveDateTime, // Event time
    #[serde(rename = "s")]
    pub symbol: String, // Symbol
    #[serde(rename = "a")]
    pub trade_id: u64, // Trade ID
    #[serde(deserialize_with = "deserialize_as_f64")]
    #[serde(rename = "p")]
    pub price: f64, // Bids to be updated
    #[serde(deserialize_with = "deserialize_as_f64")]
    #[serde(rename = "q")]
    pub quantity: f64, // Asks to be updated
    #[serde(rename = "f")]
    pub first_trade_id: u64, // First update ID in event
    #[serde(rename = "l")]
    pub last_trade_id: u64, // Final update ID in event
    #[serde(rename = "T")]
    pub trade_time: NaiveDateTime, // Final update ID in event
    #[serde(rename = "m")]
    pub buyer_mkt_maker: bool, //  is buyer the market maker?
}

impl fmt::Display for AggregateTradeData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pretty_json = serde_json::to_string_pretty(&self).unwrap();
        write!(f, "{}", pretty_json)
    }
}
