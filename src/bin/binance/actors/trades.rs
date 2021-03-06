use trading_sys::models::trades::TradeData;
use trading_sys::{create_trade, establish_connection_pg};

use std::time::Duration;

use actix::*;
use actix_web::ws;

pub struct TradeActor {
    pub client_writer: ws::ClientWriter,
}

impl Actor for TradeActor {
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

impl TradeActor {
    fn hb(&self, ctx: &mut Context<Self>) {
        ctx.run_later(std::time::Duration::new(1, 0), |act, ctx| {
            act.client_writer.pong("Heartbeat");
            act.hb(ctx);
            // client should check for a timeout here, similar to server code
        });
    }
}

#[derive(Message)]
pub struct ClientCommand(pub String);

/// Handle stdin commands
impl Handler<ClientCommand> for TradeActor {
    type Result = ();

    fn handle(&mut self, command: ClientCommand, _ctx: &mut Context<Self>) {
        self.client_writer.text(command.0)
    }
}

/// Handle Websocket messages
impl StreamHandler<ws::Message, ws::ProtocolError> for TradeActor {
    fn handle(&mut self, msg: ws::Message, _ctx: &mut Context<Self>) {
        match msg {
            ws::Message::Text(txt) => {
                let trade_data: TradeData = serde_json::from_str::<TradeData>(&txt).unwrap();
                let connection = establish_connection_pg();
                create_trade(&connection, &trade_data);
                println!("{:?}", trade_data);
            }
            ws::Message::Ping(ping) => self.client_writer.pong(&ping),
            ws::Message::Pong(pong) => self.client_writer.ping(&pong),
            _ => (),
        }
    }

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("<trade_actor.rs>: Websocket Connected.");
    }

    fn finished(&mut self, ctx: &mut Context<Self>) {
        println!("<trade_actor.rs>: Websocket Disconnected.");
        ctx.stop()
    }
}
