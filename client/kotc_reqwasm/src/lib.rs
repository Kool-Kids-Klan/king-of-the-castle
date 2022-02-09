pub mod endpoints;
pub mod server_structs;
pub mod ws_onmessage;
pub mod ws_send;
pub mod ws_structs;

use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use reqwasm::websocket::{futures::WebSocket, Message};
use serde::Serialize;
use server_structs::{Card, Column, Player, Token};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;

use crate::ws_onmessage::onmessage;
use crate::ws_send::{ready, user_joined};
use kotc_commons::messages::ClientWsMessage;
use yew::Callback;
use crate::server_structs::Color;

fn serialize<T: Serialize>(object: T) -> String {
    serde_json::to_string(&object).unwrap()
}

pub struct GameStateSetters {
    pub set_players: Callback<Vec<Player>>,
    pub set_started: Callback<bool>,
    pub set_columns: Callback<Vec<Column>>,
    pub set_hand: Callback<Vec<Card>>,
    pub set_logs: Callback<String>,
    pub set_tokens: Callback<HashMap<String, (Color, Vec<Token>)>>,
    pub set_player_on_turn: Callback<Player>,
    pub set_final_results: Callback<HashMap<String, (Color, u8)>>,
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

pub fn connect_websocket(lobby_id: String, setters: GameStateSetters) -> KotcWebSocket {
    // This method is meant to return KotcWebSocket, thus it would be possible to call ws.send_message from anywhere
    // console_log::init_with_level(Level::Debug).unwrap();
    let ws = KotcWebSocket::new(&format!("ws://127.0.0.1:8081/lobby/{}", lobby_id), setters);
    // spawn_local(async move {
    //     ws.send_message(user_joined(19)).await;
    //     ws.send_message(ready(19)).await;
    //     ws.send_message(play_card(19, 1, 3)).await;
    // });
    ws
}

pub fn send_ready(id: i32, ws: Rc<RefCell<Option<KotcWebSocket>>>) {
    spawn_local(async move {
        match Rc::clone(&ws).borrow_mut().as_mut() {
            Some(ws) => ws.send_message(ready(id)).await,
            None => log::warn!("Websocket was not initialized"),
        }
    });
}

pub fn send_join(id: i32, ws: Rc<RefCell<Option<KotcWebSocket>>>) {
    spawn_local(async move {
        // Rc::clone(&ws).borrow_mut().send_message(user_joined(id)).await;
        match Rc::clone(&ws).borrow_mut().as_mut() {
            Some(ws) => ws.send_message(user_joined(id)).await,
            None => log::warn!("Websocket was not initialized"),
        }
    });
}
