use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientWsMessageType {
    PlayCard,
    Ready,
    Unready,
    UserJoined,
    Error,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerWsMessageType {
    WsAction,
    // UserJoined,
    // UserDisconnected,
    YourId,

    UpdatePlayers,
    UpdateHand,
    UpdateColumns,
    UpdateTokens,
    StartGame,
    FinishGame,
    ActionLog,
    Error,
}
