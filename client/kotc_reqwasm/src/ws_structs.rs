use serde::{Deserialize, Serialize};
use crate::server_structs::Card;

#[derive(Debug, Serialize, Deserialize)]
pub struct KotcMessage(pub String);

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
