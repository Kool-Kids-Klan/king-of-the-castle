use kotc_commons::messages::{ClientWsMessage, PlayCard};
use kotc_commons::messages::message_types::ClientWsMessageType;
use crate::serialize;

pub fn play_card(card_index: usize, column_index: usize) -> ClientWsMessage {
    let play_card = PlayCard {
        card_index,
        column_index,
    };
    let play_card_serialized = serialize(play_card);

    ClientWsMessage {
        message_type: ClientWsMessageType::PlayCard,
        content: play_card_serialized,
    }
}
