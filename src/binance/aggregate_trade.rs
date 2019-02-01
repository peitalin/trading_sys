

use std::fmt;
use std::time::Duration;

use serde::de;
use serde::de::{ Deserialize, Deserializer };

use crate::serde_parsers::deserialize_as_f64;

use actix_web::ws;
use actix::*;


pub struct AggregateTradeActor {
    pub clientWriter: ws::ClientWriter,
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
            act.clientWriter.pong("Heartbeat");
            act.hb(ctx);
            // client should also check for a timeout here, similar to the
            // server code
        });
    }

    fn handlePing(&mut self, ctx: &mut Context<Self>, ping: String) {
        println!("{:?}", ws::Message::Ping(ping));
        self.clientWriter.pong("Pong from AggregateTradeActor");
        // self.hb(ctx)
        // client should check for a timeout here, similar to server code
    }
}
// fn handlePing<A: Actor>(act: &mut AggregateTradeActor, ctx: &mut Context<AggregateTradeActor>, ping: String)
//     where A: Actor + 'static
// {
//     println!("{:?}", ws::Message::Ping(ping));
//     act.clientWriter.pong("Pong from AggregateTradeActor");
//     act.hb(ctx)
//     // client should check for a timeout here, similar to server code
// }


#[derive(Message)]
pub struct ClientCommand(pub String);

/// Handle stdin commands
impl Handler<ClientCommand> for AggregateTradeActor {
    type Result = ();

    fn handle(&mut self, command: ClientCommand, ctx: &mut Context<Self>) {
        self.clientWriter.text(command.0)
    }
}




/// Handle Websocket messages
impl StreamHandler<ws::Message, ws::ProtocolError> for AggregateTradeActor {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Context<Self>) {
        match msg {
            ws::Message::Text(txt) => {
                let aggregate_trade = serde_json::from_str::<AggregateTradeData>(&txt).unwrap();
                println!("{}", aggregate_trade);
            },
            ws::Message::Ping(ping) => {
                ctx.run_later(Duration::new(0, 0), |act, ctx| { act.handlePing(ctx, ping) });
            },
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
    pub aggTrade: String,     // Event type
    #[serde(rename = "E")]
    pub eventTime: UtcTime,   // Event time
    #[serde(rename = "s")]
    pub symbol: String,       // Symbol
    #[serde(rename = "a")]
    pub tradeId: u64,         // Trade ID
    #[serde(deserialize_with="deserialize_as_f64")]
    #[serde(rename = "p")]
    pub price: f64,           // Bids to be updated
    #[serde(deserialize_with="deserialize_as_f64")]
    #[serde(rename = "q")]
    pub quantity: f64,        // Asks to be updated
    #[serde(rename = "f")]
    pub firstTradeId: u64,    // First update ID in event
    #[serde(rename = "l")]
    pub lastTradeId: u64,     // Final update ID in event
    #[serde(rename = "T")]
    pub tradeTime: UtcTime,     // Final update ID in event
    #[serde(rename = "m")]
    pub buyerMarketMaker: bool,  //  is buyer the market maket?
}

impl fmt::Display for AggregateTradeData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pretty_json = serde_json::to_string_pretty(&self).unwrap();
        write!(f, "{}", pretty_json)
    }
}



/// Implement traits fors UtcTime data structure
/// Used for serde_json to deserialize Unix timestamps (int)
/// into chrono::DateTime types
#[derive(Debug, Serialize, Clone)]
pub struct UtcTime(chrono::DateTime<chrono::Utc>);

impl<'de> Deserialize<'de> for UtcTime {
    fn deserialize<D>(deserializer: D) -> Result<UtcTime, D::Error>
    where D: Deserializer<'de>
    {
        let d: i64 = Deserialize::deserialize(deserializer)?;
        let naive = chrono::NaiveDateTime::from_timestamp(d/1000, 0);
        // Create a normal DateTime from the NaiveDateTime
        let datetime = chrono::DateTime::from_utc(naive, chrono::Utc);
        // // Format the datetime how you want
        // let newdate = datetime.format("%Y-%m-%d %H:%M:%S");
        Ok(UtcTime(datetime))
    }
}

impl fmt::Display for UtcTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let datetime = self.0;
        let newdate = datetime.format("%Y-%m-%d %H:%M:%S");
        write!(f, "{}", newdate)
    }
}


