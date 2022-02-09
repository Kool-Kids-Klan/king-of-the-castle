use kotc_commons::messages::message_types::ServerWsMessageType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::game::card::Card;
use crate::game::column::Column;
use crate::game::player::{Color, Player};
use crate::game::Token;

#[derive(Debug)]
pub enum MessageRecipient {
    SingleUser,
    AllUsers,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePlayers {
    pub players: Vec<Player>,
    pub player_on_turn: Player,
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
pub struct UpdateTokens {
    pub tokens: HashMap<String, (Color, Vec<Token>)>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinishGame {
    pub winner: String,
    pub results: HashMap<String, u8>, // (username, score)
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
