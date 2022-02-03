use kotc_commons::messages::{ClientWsMessage, Error, PlayCard, Ready, UnReady, UserJoined};
use kotc_commons::messages::message_types::ClientWsMessageType;
use crate::serialize;

pub fn play_card(user_id: i32, card_index: usize, column_index: usize) -> ClientWsMessage {
    let play_card = PlayCard {
        user_id,
        card_index,
        column_index,
    };
    let play_card_serialized = serialize(play_card);

    ClientWsMessage {
        message_type: ClientWsMessageType::PlayCard,
        content: play_card_serialized,
    }
}

pub fn user_joined(user_id: i32) -> ClientWsMessage {
    let user_joined = UserJoined {
        user_id,
    };
    let user_joined_serialized = serialize(user_joined);

    ClientWsMessage {
        message_type: ClientWsMessageType::UserJoined,
        content: user_joined_serialized,
    }
}

pub fn ready(user_id: i32) -> ClientWsMessage {
    let ready = Ready {
        user_id,
    };
    let ready_serialized = serialize(&ready);

    ClientWsMessage {
        message_type: ClientWsMessageType::Ready,
        content: ready_serialized,
    }
}

pub fn unready(user_id: i32) -> ClientWsMessage {
    let unready = UnReady {
        user_id,
    };
    let unready_serialized = serialize(&unready);

    ClientWsMessage {
        message_type: ClientWsMessageType::Unready,
        content: unready_serialized,
    }
}

pub fn error(detail: String) -> ClientWsMessage {
    let error = Error {
        detail,
    };
    let error_serialized = serialize(&error);

    ClientWsMessage {
        message_type: ClientWsMessageType::Error,
        content: error_serialized,
    }
}
