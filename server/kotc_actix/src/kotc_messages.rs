use actix::prelude::Message;
use actix::Recipient;
use kotc_commons::messages::message_types::ServerWsMessageType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Message)]
#[rtype(result = "()")]
pub struct ServerWsMessage {
    pub message_type: ServerWsMessageType,
    pub content: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<ServerWsMessage>,
    pub id: usize,
    pub lobby_id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
    pub lobby_id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub session_id: usize,
    pub lobby_id: usize,
    pub msg: String,
}

#[derive(Serialize, Deserialize)]
pub struct Error {
    pub detail: String,
}
