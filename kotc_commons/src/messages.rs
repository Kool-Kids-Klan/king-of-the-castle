use serde::{Deserialize, Serialize};
use actix::prelude::Message;

use kotc_game::game::card::Card;
use kotc_game::game::column::Column;

#[derive(Serialize, Deserialize, Message)]
#[rtype(result = "()")]
pub struct UpdateBoard {
    pub board: Vec<Column>,
}

#[derive(Serialize, Deserialize, Message)]
#[rtype(result = "()")]
pub struct UpdateHand {
    pub hand: Vec<Card>,
}

#[derive(Serialize, Deserialize, Message)]
#[rtype(result = "()")]
pub struct PlayCard {
    pub card_index: usize,
    pub column_index: usize
}
