

use std::fmt;
use std::time::Duration;

use serde::de;
use serde::de::{ Deserialize, Deserializer };
use crate::serde_parsers::{ deserialize_as_f64, UtcTime };

use actix_web::ws;
use futures::Future; // map_error for ws::Client
use actix::*;


pub struct TradeActor {
    pub clientWriter: ws::ClientWriter,
}

impl Actor for TradeActor {

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


impl TradeActor {

    fn hb(&self, ctx: &mut Context<Self>) {
        ctx.run_later(std::time::Duration::new(1, 0), |act, ctx| {
            act.clientWriter.pong("Heartbeat");
            act.hb(ctx);
            // client should check for a timeout here, similar to server code
        });
    }

    fn handle_ping(&mut self, ctx: &mut Context<Self>, ping: String) {
        println!("{:?}", ws::Message::Ping(ping));
        self.clientWriter.pong("Pong from TradeActor");
        // self.hb(ctx)
        // client should check for a timeout here, similar to server code
    }
}


#[derive(Message)]
pub struct ClientCommand(pub String);

/// Handle stdin commands
impl Handler<ClientCommand> for TradeActor {
    type Result = ();

    fn handle(&mut self, command: ClientCommand, ctx: &mut Context<Self>) {
        self.clientWriter.text(command.0)
    }
}


/// Handle Websocket messages
impl StreamHandler<ws::Message, ws::ProtocolError> for TradeActor {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Context<Self>) {
        match msg {
            ws::Message::Text(txt) => {
                let trade = serde_json::from_str::<TradeData>(&txt).unwrap();
                println!("{}", trade);
                // println!("{:?}", txt);
            },
            ws::Message::Ping(ping) => {
                ctx.run_later(Duration::new(0, 0), |act, ctx| { act.handle_ping(ctx, ping) });
            },
            _ => (),
        }
    }

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("<trade.rs>: Websocket Connected.");
    }

    fn finished(&mut self, ctx: &mut Context<Self>) {
        println!("<trade.rs>: Websocket Disconnected.");
        ctx.stop()
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TradeData {
    #[serde(rename = "e")]
    pub event: String,         // Event type
    #[serde(rename = "E")]
    pub event_time: UtcTime,   // Event time
    #[serde(rename = "s")]
    pub symbol: String,        // Symbol
    #[serde(rename = "t")]
    pub trade_Id: u64,         // Trade ID
    #[serde(deserialize_with="deserialize_as_f64")]
    #[serde(rename = "p")]
    pub price: f64,            // Price
    #[serde(deserialize_with="deserialize_as_f64")]
    #[serde(rename = "q")]
    pub quantity: f64,         // Quantity
    #[serde(rename = "b")]
    pub buyer_order_Id: u64,   // Buyer order ID
    #[serde(rename = "a")]
    pub seller_order_Id: u64,  // Seller order ID
    #[serde(rename = "m")]
    pub buyer_mkt_maker: bool, //  is buyer the market maket?
}

impl fmt::Display for TradeData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pretty_json = serde_json::to_string_pretty(&self).unwrap();
        write!(f, "{}", pretty_json)
    }
}




