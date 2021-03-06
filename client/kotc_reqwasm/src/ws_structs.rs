use crate::server_structs::{Card, Color, Column, Player, Token};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateHand {
    pub hand: Vec<Card>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTokens {
    pub tokens: HashMap<String, (Color, Vec<Token>)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateColumns {
    pub columns: Vec<Column>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartGame {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub detail: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YourId {
    pub id: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePlayers {
    pub players: Vec<Player>,
    pub player_on_turn: Player,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDisconnected {
    pub player: Vec<Player>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WsAction {
    pub detail: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionLog {
    pub detail: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinishGame {
    pub winner: String,
    pub results: HashMap<String, (Color, u8)>, // (username, score)
}
