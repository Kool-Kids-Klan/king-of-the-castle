use futures::stream::SplitStream;
use futures::StreamExt;
use log::info;
use reqwasm::websocket::futures::WebSocket;
use reqwasm::websocket::{Message, WebSocketError};
use serde::de;
use kotc_commons::messages::message_types::ServerWsMessageType;
use kotc_commons::messages::ServerWsMessage;
use yew::Callback;
use crate::{KotcWebSocketReader};
use crate::server_structs::Player;
use crate::ws_structs::{ActionLog, Error, FinishGame, StartGame, UpdateColumns, UpdateHand, UpdatePlayers, WsAction, YourId};

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

pub async fn onmessage(ws: KotcWebSocketReader) {
    let mut read = ws.read;
    let set_players = ws.set_players;
    let set_started = ws.set_started;
    while let Some(msg) = read.next().await {
        let server_message = get_server_message(msg);

        match server_message.message_type {
            ServerWsMessageType::UpdateColumns => {
                let update_board: UpdateColumns = get_deserialized(&server_message.content);
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
            // ServerWsMessageType::UserJoined => {
            //     let user_joined: UserJoined = get_deserialized(&server_message.content);
            //     info!("user joined {:?}", user_joined);
            // },
            // ServerWsMessageType::UserDisconnected => {
            //     let user_disconnected: UserDisconnected = get_deserialized(&server_message.content);
            //     info!("user disconnected {:?}", user_disconnected);
            // },
            ServerWsMessageType::UpdatePlayers => {
                let update_players: UpdatePlayers = get_deserialized(&server_message.content);
                info!("update players {:?}", update_players);
                set_players.emit(update_players.players);
            }
            ServerWsMessageType::WsAction => {
                let ws_action: WsAction = get_deserialized(&server_message.content);
                info!("ws action {:?}", ws_action);
            }
            ServerWsMessageType::StartGame => {
                let start_game: StartGame = get_deserialized(&server_message.content);
                info!("start game {:?}", start_game);
                set_started.emit(true);
            },
            ServerWsMessageType::FinishGame => {
                let finish_game: FinishGame = get_deserialized(&server_message.content);
                info!("finish game {:?}", finish_game);
                set_started.emit(false);
            }
            ServerWsMessageType::ActionLog => {
                let action_log: ActionLog = get_deserialized(&server_message.content);
                info!("action log {:?}", action_log);
            }
        };
    }
}
