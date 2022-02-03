mod server_structs;
mod ws_structs;
mod ws_onmessage;
mod ws_send;

use futures::{SinkExt, StreamExt};
use log::{info, Level};
use reqwasm::websocket::{futures::WebSocket, Message};
use wasm_bindgen_futures::spawn_local;
use futures::stream::SplitSink;
use serde::Serialize;

use kotc_commons::messages::{ClientWsMessage};
use crate::ws_onmessage::onmessage;
use crate::ws_send::{play_card};

fn serialize<T: Serialize>(object: T) -> String {
    serde_json::to_string(&object).unwrap()
}

pub struct KotcWebSocket {
    pub write: SplitSink<WebSocket, Message>,
}

impl KotcWebSocket {
    pub fn new(socket_url: &str) -> Self {
        let ws = WebSocket::open(socket_url).unwrap();
        let (write, read) = ws.split();
        spawn_local(async move {
            onmessage(read).await;
        });

        KotcWebSocket {
            write,
        }
    }

    pub async fn send_message(&mut self, client_message: ClientWsMessage) {
        let client_message_serialized = serialize(client_message);
        self.write
            .send(Message::Text(client_message_serialized))
            .await
            .unwrap();
    }
}

pub fn connect_websocket() { // This method is meant to return KotcWebSocket, thus it would be possible to call ws.send_message from anywhere
    console_log::init_with_level(Level::Debug).unwrap();
    let mut ws = KotcWebSocket::new("ws://127.0.0.1:8081/lobby/1234");
    spawn_local(async move {
        ws.send_message(play_card(1, 3)).await;
    })
}
