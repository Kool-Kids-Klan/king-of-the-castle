use serde::{Deserialize, Serialize};
use kotc_commons::messages::message_types::ServerWsMessageType;

use crate::game::card::Card;
use crate::game::column::Column;

pub trait RandomTrait {}

#[derive(Serialize, Deserialize)]
pub struct Error {
    pub detail: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateHand {
    pub hand: Vec<Card>,
}


#[derive(Serialize, Deserialize)]
pub struct UpdateBoard {
    pub board: Vec<Column>,
}

#[derive(Serialize, Deserialize)]
pub struct ServerMessage {
    pub message_type: ServerWsMessageType,
    pub content: String,
}
