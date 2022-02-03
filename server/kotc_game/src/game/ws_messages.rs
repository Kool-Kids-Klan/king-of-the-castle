use std::collections::HashMap;
use kotc_commons::messages::message_types::ServerWsMessageType;
use serde::{Serialize};

use crate::game::card::Card;
use crate::game::column::Column;
use crate::game::Token;

pub enum MessageRecipient {
    SingleUser,
    AllUsers
}

pub trait ServerMessageTrait {}

#[derive(Serialize)]
pub struct UpdatePlayers {
    pub players: Vec<String>,
}
impl ServerMessageTrait for UpdatePlayers {}


#[derive(Serialize)]
pub struct UpdateHand {
    pub hand: Vec<Card>,
}
impl ServerMessageTrait for UpdateHand {}


#[derive(Serialize)]
pub struct UpdateTokens {
    pub tokens: HashMap<String, Vec<Token>>,  // (username, tokens)
}
impl ServerMessageTrait for UpdateTokens {}


#[derive(Serialize)]
pub struct UpdateColumns {
    pub columns: Vec<Column>,
}
impl ServerMessageTrait for UpdateColumns {}


#[derive(Serialize)]
pub struct FinishGame {
    pub winner: String,
    pub results: HashMap<String, u8>  // (username, score)
}
impl ServerMessageTrait for FinishGame {}


#[derive(Serialize)]
pub struct ActionLog {
    pub detail: String,
}
impl ServerMessageTrait for ActionLog {}


#[derive(Serialize)]
pub struct Error {
    pub detail: String,
}
impl ServerMessageTrait for Error {}


#[derive(Serialize)]
pub struct Success {}
impl ServerMessageTrait for Success {}


pub struct ServerMessage {
    pub message_type: ServerWsMessageType,
    pub recipient: MessageRecipient,
    pub content: String,
}




