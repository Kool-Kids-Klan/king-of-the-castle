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
    pub card_index: usize,
    pub column_index: usize,
}

/// ****SERVER MESSAGES****

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerWsMessage {
    pub message_type: ServerWsMessageType,
    pub content: String,
}

//
// #[derive(Serialize, Deserialize, Message, Debug)]
// #[rtype(result = "()")]
// pub struct ServerWsMessage<T: ServerWsMessages> {
//     pub message_type: ServerWsMessageType,
//     pub content: T,
// }
//
// #[derive(Serialize, Deserialize, Message, Debug)]
// #[rtype(result = "()")]
// pub struct UpdateBoard {
//     pub board: Vec<Column>,
// }
//
// impl ServerWsMessages for UpdateBoard {}
//
// #[derive(Serialize, Deserialize, Message, Debug)]
// #[rtype(result = "()")]
// pub struct UpdateHand {
//     pub hand: Vec<Card>,
// }
//
// impl ServerWsMessages for UpdateHand {}
