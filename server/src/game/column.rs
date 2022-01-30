use super::card::{Card, Character};
use crate::game::Token;

use anyhow::Result;

#[derive(Clone, Debug)]
pub struct Column {
    token: Token,
    blocked: bool,  // Boure
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

    pub fn is_completed(&self) -> bool {
        self.cards.len() as u8 >= self.token.points || self.blocked
    }

    fn reveal_last_card(&mut self) {
        if !self.cards.is_empty() {
            let last_index = self.cards.len() - 1;
            self.cards[last_index].revealed = true;
        }
    }

    pub fn add_card(&mut self, card: Card)  {
        self.reveal_last_card();
        self.cards.push(card);
    }

    pub fn get_winner(&self) -> String {
        "".to_string()
    }
}
