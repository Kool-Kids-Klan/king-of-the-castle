use futures::stream::SplitStream;
use futures::StreamExt;
use log::info;
use reqwasm::websocket::futures::WebSocket;
use reqwasm::websocket::{Message, WebSocketError};
use serde::de;
use kotc_commons::messages::message_types::ServerWsMessageType;
use kotc_commons::messages::ServerWsMessage;
use crate::ws_structs::{Error, StartGame, UpdateBoard, UpdateHand, UserDisconnected, UserJoined, WsAction, YourId};

fn get_server_message(msg: Result<Message, WebSocketError>) -> ServerWsMessage {
    match msg {
        Ok(message) => {
            match message {
                Message::Text(content) => match serde_json::from_str(&content) {
                    Ok(n) => n,
                    Err(err) => panic!("Could not parse server message. {}", err),
                },
                _ => panic!("WebSocket message is not in TEXT format"),
            }
        },
        Err(err) => panic!("WebSocket error {}", err),
    }
}

fn get_deserialized<T: de::DeserializeOwned>(content: &String) -> T {
    serde_json::from_str::<T>(content).unwrap()
}

pub async fn onmessage(mut read: SplitStream<WebSocket>) {
    while let Some(msg) = read.next().await {
        let server_message = get_server_message(msg);

        match server_message.message_type {
            ServerWsMessageType::UpdateBoard => {
                let update_board: UpdateBoard = get_deserialized(&server_message.content);
                info!("update board {:?}", update_board);
            },
            ServerWsMessageType::UpdateHand => {
                let update_hand: UpdateHand = get_deserialized(&server_message.content);
                info!("update hand {:?}", update_hand);
            },
            ServerWsMessageType::Error => {
                let error: Error = get_deserialized(&server_message.content);
                info!("error {:?}", error);
            },
            ServerWsMessageType::YourId => {
                let your_id: YourId = get_deserialized(&server_message.content);
                info!("your id {:?}", your_id);
            },
            ServerWsMessageType::UserJoined => {
                let user_joined: UserJoined = get_deserialized(&server_message.content);
                info!("user joined {:?}", user_joined);
            },
            ServerWsMessageType::UserDisconnected => {
                let user_disconnected: UserDisconnected = get_deserialized(&server_message.content);
                info!("user disconnected {:?}", user_disconnected);
            },
            ServerWsMessageType::WsAction => {
                let ws_action: WsAction = get_deserialized(&server_message.content);
                info!("ws action {:?}", ws_action);
            }
            ServerWsMessageType::StartGame => {
                let start_game: StartGame = get_deserialized(&server_message.content);
                info!("start game {:?}", start_game);
            },
        };
    }
}
