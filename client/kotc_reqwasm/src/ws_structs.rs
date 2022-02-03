use std::collections::HashMap;
use crate::server_structs::{Card, Column, Player, Token};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateHand {
    pub hand: Vec<Card>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateColumns {
    pub board: Vec<Column>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateTokens {
    pub tokens: HashMap<String, Vec<Token>>,  // (username, tokens)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartGame {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Success {}

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
    pub players: Vec<String>,
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
    pub results: HashMap<String, u8>  // (username, score)
}
