use crate::ws_structs::{
    ActionLog, Error, FinishGame, StartGame, UpdateColumns, UpdateHand, UpdatePlayers,
    UpdateTokens, WsAction, YourId,
};
use crate::GameStateSetters;
use futures::stream::SplitStream;
use futures::StreamExt;
use kotc_commons::messages::message_types::ServerWsMessageType;
use kotc_commons::messages::ServerWsMessage;
use log::info;
use reqwasm::websocket::futures::WebSocket;
use reqwasm::websocket::{Message, WebSocketError};
use serde::de;

fn get_server_message(msg: Result<Message, WebSocketError>) -> ServerWsMessage {
    match msg {
        Ok(message) => match message {
            Message::Text(content) => match serde_json::from_str(&content) {
                Ok(n) => n,
                Err(err) => panic!("Could not parse server message. {}", err),
            },
            _ => panic!("WebSocket message is not in TEXT format"),
        },
        Err(err) => panic!("WebSocket error {}", err),
    }
}

fn get_deserialized<T: de::DeserializeOwned>(content: &str) -> T {
    serde_json::from_str::<T>(content).unwrap()
}

pub async fn onmessage(read: SplitStream<WebSocket>, ws: GameStateSetters) {
    let mut read = read;
    let set_players = ws.set_players;
    let set_started = ws.set_started;
    let set_columns = ws.set_columns;
    let set_hand = ws.set_hand;
    let set_logs = ws.set_logs;
    let set_tokens = ws.set_tokens;
    let set_player_on_turn = ws.set_player_on_turn;
    let set_final_results = ws.set_final_results;

    while let Some(msg) = read.next().await {
        let server_message = get_server_message(msg);

        match server_message.message_type {
            ServerWsMessageType::UpdateColumns => {
                let update_board: UpdateColumns = get_deserialized(&server_message.content);
                info!("update board {:?}", update_board);
                set_columns.emit(update_board.columns);
            }
            ServerWsMessageType::UpdateHand => {
                let update_hand: UpdateHand = get_deserialized(&server_message.content);
                info!("update hand {:?}", update_hand);
                set_hand.emit(update_hand.hand);
            }
            ServerWsMessageType::Error => {
                let error: Error = get_deserialized(&server_message.content);
                info!("error {:?}", error);
            }
            ServerWsMessageType::YourId => {
                let your_id: YourId = get_deserialized(&server_message.content);
                info!("your id {:?}", your_id);
            }
            // ServerWsMessageType::UserJoined => {
            //     let user_joined: UserJoined = get_deserialized(&server_message.content);
            //     info!("user joined {:?}", user_joined);
            // },
            // ServerWsMessageType::UserDisconnected => {
            //     let user_disconnected: UserDisconnected = get_deserialized(&server_message.content);
            //     info!("user disconnected {:?}", user_disconnected);
            // },
            ServerWsMessageType::UpdatePlayers => {
                let UpdatePlayers {
                    players,
                    player_on_turn,
                } = get_deserialized(&server_message.content);
                info!("update players {:?} {:?}", players, player_on_turn);
                set_players.emit(players);
                set_player_on_turn.emit(player_on_turn);
            }
            ServerWsMessageType::UpdateTokens => {
                let update_tokens: UpdateTokens = get_deserialized(&server_message.content);
                info!("update tokens {:?}", update_tokens);
                set_tokens.emit(update_tokens.tokens);
            }
            ServerWsMessageType::WsAction => {
                let ws_action: WsAction = get_deserialized(&server_message.content);
                info!("ws action {:?}", ws_action);
            }
            ServerWsMessageType::StartGame => {
                let start_game: StartGame = get_deserialized(&server_message.content);
                info!("start game {:?}", start_game);
                set_started.emit(true);
            }
            ServerWsMessageType::FinishGame => {
                let FinishGame { winner: _, results } = get_deserialized(&server_message.content);
                info!("finish game {:?}", results);
                set_started.emit(false);
                set_final_results.emit(results);
            }
            ServerWsMessageType::ActionLog => {
                let action_log: ActionLog = get_deserialized(&server_message.content);
                info!("action log {:?}", action_log);
                set_logs.emit(action_log.detail);
            }
        };
    }
}
