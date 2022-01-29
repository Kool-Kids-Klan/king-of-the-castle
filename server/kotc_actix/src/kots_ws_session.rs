use actix::{
    fut, Actor, ActorContext, ActorFuture, Addr, AsyncContext, ContextFutureSpawner, Handler,
    Running, StreamHandler, WrapFuture,
};
use actix_web_actors::ws;
use actix_web_actors::ws::Message::Text;
use rand::Rng;
use std::time::{Duration, Instant};

use crate::kotc_messages::{ClientMessage, Connect, Disconnect, KotcMessage};
use crate::kotc_ws_server::KotcWsServer;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct KotcWsSession {
    id: usize,
    lobby_id: usize,
    server_addr: Addr<KotcWsServer>,
    hb: Instant,
}

impl KotcWsSession {
    pub fn new(lobby_id: usize, server_addr: Addr<KotcWsServer>) -> Self {
        let mut rng = rand::thread_rng();
        KotcWsSession {
            id: rng.gen(),
            lobby_id,
            server_addr,
            hb: Instant::now(),
        }
    }

    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |session, ctx| {
            if Instant::now().duration_since(session.hb) > CLIENT_TIMEOUT {
                println!("Disconnecting, failed heartbeat!");
                session.server_addr.do_send(Disconnect {
                    id: session.id,
                    lobby_id: session.lobby_id,
                });
                ctx.stop();
                return;
            }

            ctx.ping(b"Ping");
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for KotcWsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(message) => match message {
                ws::Message::Ping(msg) => {
                    self.hb = Instant::now();
                    ctx.pong(&msg);
                },
                ws::Message::Pong(_) => self.hb = Instant::now(),
                ws::Message::Binary(bin) => ctx.binary(bin),
                ws::Message::Close(reason) => {
                    ctx.close(reason);
                    ctx.stop();
                },
                ws::Message::Continuation(_) => ctx.stop(),
                ws::Message::Nop => (),
                Text(s) => self.server_addr.do_send(ClientMessage {
                    session_id: self.id,
                    lobby_id: self.lobby_id,
                    msg: s, // TODO: can we use ByteString in ClientMessage
                }),
            },
            Err(e) => panic!("WebSocket error: {:?}", e),
        }
    }
}

impl Handler<KotcMessage> for KotcWsSession {
    type Result = ();

    fn handle(&mut self, msg: KotcMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
    }
}

impl Actor for KotcWsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        self.server_addr
            .send(Connect {
                addr: addr.recipient(),
                lobby_id: self.lobby_id,
                id: self.id,
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_res) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.server_addr.do_send(Disconnect {
            id: self.id,
            lobby_id: self.lobby_id,
        });
        Running::Stop
    }
}
