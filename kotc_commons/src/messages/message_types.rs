use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientWsMessageType {
    PlayCard,
    Ready,
    Unready,

    Error,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerWsMessageType {
    WsAction,
    UserJoined,
    UserDisconnected,
    YourId,

    UpdateHand,
    UpdateBoard,
    StartGame,

    Error,
}
