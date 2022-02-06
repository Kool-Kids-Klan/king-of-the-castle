pub mod endpoints;
pub mod server_structs;
pub mod ws_structs;
pub mod ws_onmessage;
pub mod ws_send;

use std::cell::RefCell;
use std::rc::Rc;
use futures::{SinkExt, StreamExt};
use log::{info, Level};
use reqwasm::websocket::{futures::WebSocket, Message};
use server_structs::{Player, Column, Card};
use wasm_bindgen_futures::spawn_local;
use futures::stream::{SplitSink, SplitStream};
use serde::Serialize;

use kotc_commons::messages::{ClientWsMessage, Ready, UserJoined};
use kotc_commons::messages::message_types::ClientWsMessageType;
use yew::Callback;
use crate::ws_onmessage::onmessage;
use crate::ws_send::{play_card, ready, user_joined};

fn serialize<T: Serialize>(object: T) -> String {
    serde_json::to_string(&object).unwrap()
}

pub struct GameStateSetters {
    pub set_players: Callback<Vec<Player>>,
    pub set_started: Callback<bool>,
    pub set_columns: Callback<Vec<Column>>,
    pub set_hand: Callback<Vec<Card>>,
}

pub struct KotcWebSocket {
    pub write: SplitSink<WebSocket, Message>,
}

impl KotcWebSocket {
    pub fn new(socket_url: &str, setters: GameStateSetters) -> KotcWebSocket {
        let ws = WebSocket::open(socket_url).unwrap();
        let (write, read) = ws.split();

        spawn_local(async move {
            onmessage(read, setters).await;
        });

        KotcWebSocket { write }
    }

    pub async fn send_message(&mut self, client_message: ClientWsMessage) {
        let client_message_serialized = serialize(client_message);
        self.write
            .send(Message::Text(client_message_serialized))
            .await
            .unwrap();
    }
}

pub fn connect_websocket(lobby_id: String, setters: GameStateSetters) -> KotcWebSocket { // This method is meant to return KotcWebSocket, thus it would be possible to call ws.send_message from anywhere
    // console_log::init_with_level(Level::Debug).unwrap();
    let ws = KotcWebSocket::new(&format!("ws://127.0.0.1:8081/lobby/{}", lobby_id), setters);
    // spawn_local(async move {
    //     ws.send_message(user_joined(19)).await;
    //     ws.send_message(ready(19)).await;
    //     ws.send_message(play_card(19, 1, 3)).await;
    // });
    ws
}

pub fn send_ready(id: i32, ws: Rc<RefCell<KotcWebSocket>>) {
    spawn_local(async move {
        Rc::clone(&ws).borrow_mut().send_message(ready(id)).await;
    });
}

pub fn send_join(id: i32, ws: Rc<RefCell<KotcWebSocket>>) {
    spawn_local(async move {
        Rc::clone(&ws).borrow_mut().send_message(user_joined(id)).await;
    });
}