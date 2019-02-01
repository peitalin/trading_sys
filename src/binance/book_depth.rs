

use std::fmt;
use std::time::Duration;

use serde::de;
use serde::de::{ Deserialize, Deserializer };

use actix_web::ws;
use actix::*;


pub struct BookDepthActor {
    pub clientWriter: ws::ClientWriter,
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
            act.clientWriter.pong("Heartbeat");
            act.hb(ctx);
            // client should also check for a timeout here, similar to the
            // server code
        });
    }

    fn handlePing(&mut self, ctx: &mut Context<Self>, ping: String) {
        println!("{:?}", ws::Message::Ping(ping));
        self.clientWriter.pong("Pong from BookDepthActor");
        // self.hb(ctx)
        // client should check for a timeout here, similar to server code
    }
}
// fn handlePing<A: Actor>(act: &mut BookDepthActor, ctx: &mut Context<BookDepthActor>, ping: String)
//     where A: Actor + 'static
// {
//     println!("{:?}", ws::Message::Ping(ping));
//     act.clientWriter.pong("Pong from BookDepthActor");
//     act.hb(ctx)
//     // client should check for a timeout here, similar to server code
// }


#[derive(Message)]
pub struct ClientCommand(pub String);

/// Handle stdin commands
impl Handler<ClientCommand> for BookDepthActor {
    type Result = ();

    fn handle(&mut self, command: ClientCommand, ctx: &mut Context<Self>) {
        self.clientWriter.text(command.0)
    }
}




/// Handle Websocket messages
impl StreamHandler<ws::Message, ws::ProtocolError> for BookDepthActor {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Context<Self>) {
        match msg {
            ws::Message::Text(txt) => {
                let order_book = serde_json::from_str::<BookDepthData>(&txt).unwrap();
                println!("{}", order_book);
                // println!("{:?}", txt);
            },
            ws::Message::Ping(ping) => {
                ctx.run_later(Duration::new(0, 0), |act, ctx| { act.handlePing(ctx, ping) });
            },
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


#[derive(Debug, Serialize, Deserialize)]
pub struct BookDepthData {
    #[serde(rename = "e")]
    pub event: String,  // Event type
    #[serde(rename = "E")]
    pub eventTime: UtcTime,     // Event time
    #[serde(rename = "s")]
    pub symbol: String,  // Symbol
    #[serde(rename = "U")]
    pub updateFirst: u64,     // First update ID in event
    #[serde(rename = "u")]
    pub updateFinal: u64,     // Final update ID in event
    #[serde(rename = "b")]
    pub bids: Vec<Quote>, // Bids to be updated
    #[serde(rename = "a")]
    pub asks: Vec<Quote>, // Asks to be updated
}

impl fmt::Display for BookDepthData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pretty_json = serde_json::to_string_pretty(&self).unwrap();
        write!(f, "{}", pretty_json)
    }
}


/// This deserializes nested vector/strings: ["0.00123", "150.4", []]
/// and turns it into Quote { price: 0.00123, quantity: 150.4 }
#[derive(Debug, Serialize, Clone)]
pub struct Quote {
    pub price: f64,
    pub quantity: f64,
}
impl<'de> Deserialize<'de> for Quote {
    fn deserialize<D>(deserializer: D) -> Result<Quote, D::Error>
        where D: Deserializer<'de>
    {
        struct QuoteVisitor;

        impl<'de> de::Visitor<'de> for QuoteVisitor {
            type Value = Quote;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("string or a list of strings")
            }

            fn visit_seq<S>(self, mut visitor: S) -> Result<Self::Value, S::Error>
                where S: de::SeqAccess<'de>
            {
                let mut vec = Vec::new();
                // visitor iterators nested vector/string: ["0.00123", "150.4", []]
                loop {
                    match visitor.next_element()? {
                        Some(StringOrVec::Price(elem)) => vec.push(elem), // convert strings to f64
                        Some(StringOrVec::Vec(elem)) => continue, // skip vectors []
                        None => break, // break, when next_element() is empty
                    }
                }
                Ok(Quote { price: vec[0], quantity: vec[1] })
            }
        }
        deserializer.deserialize_any(QuoteVisitor)
    }
}


/// These functions deserializes:
/// StringOrVec::Price(elem) and StringOrVec::Vec(elem) accordingly
#[derive(Debug, Serialize, Clone)]
pub enum StringOrVec {
    Price(f64),
    Vec(Option<Vec<f64>>),
}
impl<'de> Deserialize<'de> for StringOrVec {
    fn deserialize<D>(deserializer: D) -> Result<StringOrVec, D::Error>
        where D: Deserializer<'de>,
    {
        struct StringOrVecVisitor;

        impl<'de> de::Visitor<'de> for StringOrVecVisitor {
            type Value = StringOrVec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("string or list of strings")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where E: de::Error
            {

                Ok(StringOrVec::Price(value.to_owned().parse::<f64>().unwrap()))
            }

            fn visit_seq<S>(self, mut visitor: S) -> Result<Self::Value, S::Error>
                where S: de::SeqAccess<'de>
            {
                // Ignore empty lists from Binance
                Ok(StringOrVec::Vec(None))
            }
        }
        deserializer.deserialize_any(StringOrVecVisitor)
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


