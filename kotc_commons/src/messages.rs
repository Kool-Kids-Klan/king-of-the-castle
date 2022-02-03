pub mod message_types;

use serde::{Deserialize, Serialize};

use crate::messages::message_types::{ClientWsMessageType, ServerWsMessageType};

/// ****CLIENT MESSAGES****

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientWsMessage {
    pub message_type: ClientWsMessageType,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayCard {
    pub user_id: i32,
    pub card_index: usize,
    pub column_index: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ready {
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnReady {
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserJoined {
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    pub detail: String,
}

/// ****SERVER MESSAGES****

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerWsMessage {
    pub message_type: ServerWsMessageType,
    pub content: String,
}
