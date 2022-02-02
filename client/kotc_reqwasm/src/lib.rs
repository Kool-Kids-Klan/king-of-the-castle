mod server_structs;
mod ws_structs;

use futures::{SinkExt, StreamExt};
use log::{info, Level};
use reqwasm::websocket::{futures::WebSocket, Message};
use wasm_bindgen_futures::spawn_local;

use crate::ws_structs::KotcMessage;
use kotc_commons::messages::message_types::ClientWsMessageType;
use kotc_commons::messages::{ClientWsMessage, PlayCard, ServerWsMessage};

pub fn connect_websocket() {
    console_log::init_with_level(Level::Debug).unwrap();
    let ws = WebSocket::open("ws://127.0.0.1:8081/lobby/1234").unwrap();
    let (mut write, mut read) = ws.split();

    spawn_local(async move {
        info!("FIRST SPAWN LOCAL, state");
        let play_card = PlayCard {
            card_index: 1,
            column_index: 1,
        };
        let play_card_serialized = serde_json::to_string(&play_card).unwrap();
        let client_message = ClientWsMessage {
            message_type: ClientWsMessageType::PlayCard,
            content: play_card_serialized,
        };
        let client_message_serialized = serde_json::to_string(&client_message).unwrap();
        write
            .send(Message::Text(client_message_serialized))
            .await
            .unwrap();
        info!("FINISHING FIRST LOCAL");
    });

    spawn_local(async move {
        info!("SECOND SPAWN LOCAL");
        while let Some(msg) = read.next().await {
            info!("this is message {:?}", msg);
            let kotc_message: Option<KotcMessage> = match msg {
                Ok(message) => match message {
                    Message::Text(content) => match serde_json::from_str(&content) {
                        Ok(n) => Some(n),
                        Err(_) => None,
                    },
                    _ => panic!("fuck you"),
                },
                Err(_) => panic!("fuck you again"),
            };

            if let Some(kotc_msg) = kotc_message {
                info!("deserialized message {:?}", kotc_msg);
            }
            // let server_message: ServerWsMessage = serde_json::from_str(&msg.unwrap()).unwrap();
            // log!(format!("1. {:?}", msg));
        }
        // log!("WebSocket Closed");
    })
}
