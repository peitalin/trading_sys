use trading_sys::models::AggregateTradeData;
use trading_sys::{create_aggregate_trade, establish_connection_pg};

use std::time::Duration;
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
                let aggregate_trade_data = serde_json::from_str::<AggregateTradeData>(&txt).unwrap();
                let connection = establish_connection_pg();
                create_aggregate_trade(&connection, &aggregate_trade_data);
                println!("{}", aggregate_trade_data);
            }
            ws::Message::Ping(ping) => {
                ctx.run_later(Duration::new(0, 0), |act, ctx| act.handle_ping(ctx, ping));
            }
            _ => (),
        }
    }

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("<aggregate_trade_actor.rs>: Websocket Connected.");
    }

    fn finished(&mut self, ctx: &mut Context<Self>) {
        println!("<aggregate_trade_actor.rs>: Websocket Disconnected.");
        ctx.stop()
    }
}

