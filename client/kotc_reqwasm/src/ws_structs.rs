use crate::server_structs::{Card, Player};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateHand {
    pub hand: Vec<Card>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateBoard {
    pub board: Vec<Card>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct UserJoined {
    pub player: Vec<Player>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDisconnected {
    pub player: Vec<Player>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WsAction {
    pub detail: String,
}
