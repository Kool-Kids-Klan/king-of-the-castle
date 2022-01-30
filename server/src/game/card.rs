use std::rc::Rc;

use super::player::Player;
use super::Character;

#[derive(Clone, Debug)]
pub struct Card {
    owner: String,
    character: Character,
    strength: f32,
    revealed: bool
}

impl Card {
    pub fn new(owner: String, character: Character, strength: f32) -> Card {
        Card {
            owner,
            strength,
            character,
            revealed: false
        }
    }
}
