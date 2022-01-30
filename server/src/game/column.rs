use super::card::Card;
use crate::game::Token;

#[derive(Clone, Debug)]
pub struct Column {
    token: Token,
    blocked: bool,  // Bouře
    cards: Vec<Card>,
}

impl Column {
    pub fn new(token: Token) -> Column {
        Column {
            token,
            blocked: false,
            cards: vec![],
        }
    }

    pub fn get_points(&self) -> u8 {
        // TODO
        0
    }

    pub fn is_completed(&self) -> bool {
        self.cards.len() as u8 >= self.get_points() || self.blocked
    }
}
