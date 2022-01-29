use std::io::Bytes;
use reqwasm::websocket::{Message, futures::WebSocket};
use wasm_bindgen_futures::spawn_local;
use futures::{SinkExt, StreamExt};
use log::{info, Level};
use reqwasm::websocket::State::Connecting;
use gloo_timers::callback::Timeout;

pub fn connect_websocket() {
    console_log::init_with_level(Level::Debug).unwrap();
    info!("I AM NOOT HEREE");
    let mut ws = WebSocket::open("ws://127.0.0.1:8081/ws/").unwrap();
    info!("break robi bludy");
    for _ in 0..1000 {
        match ws.state() {
            Connecting => {
                info!("still connecting");
                continue;
            }
            _ => {
                break
            }
        };
    };
    info!("this is ws {:?}", ws.state());
    let (mut write, mut read) = ws.split();

    spawn_local(async move {
        info!("FIRST SPAWN LOCAL, state");
        write.send(Message::Text(String::from("test"))).await.unwrap();
        write.send(Message::Text(String::from("test 2"))).await.unwrap();
        info!("FINISHING FIRST LOCAL");
    });

    spawn_local(async move {
        info!("SECOND SPAWN LOCAL");
        while let Some(msg) = read.next().await {
            info!("this is fucking message {:?}", msg)
            // log!(format!("1. {:?}", msg));
        }
        // log!("WebSocket Closed");
    })
}
