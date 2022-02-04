use std::collections::HashMap;
use kotc_commons::messages::message_types::ServerWsMessageType;
use serde::{Serialize, Deserialize};

use crate::game::card::Card;
use crate::game::column::Column;
use crate::game::player::Player;

#[derive(Debug)]
pub enum MessageRecipient {
    SingleUser,
    AllUsers
}


#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePlayers {
    pub players: Vec<Player>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateHand {
    pub hand: Vec<Card>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateColumns {
    pub columns: Vec<Column>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinishGame {
    pub winner: String,
    pub results: HashMap<String, u8>  // (username, score)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionLog {
    pub detail: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    pub detail: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StartGame {}

#[derive(Debug)]
pub struct ServerMessage {
    pub message_type: ServerWsMessageType,
    pub recipient: MessageRecipient,
    pub content: String,
}
