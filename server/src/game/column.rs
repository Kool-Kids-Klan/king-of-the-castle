use super::card::Card;
use super::token::Token;

pub struct Column<'a> {
    token: &'a Token,
    cards: Vec<Card<'a>>,
}

impl Column<'_> {
    pub fn new(token: &Token) -> Column {
        Column {
            token,
            cards: Vec::new(),
        }
    }
}
