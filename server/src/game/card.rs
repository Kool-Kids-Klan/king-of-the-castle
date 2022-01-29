use super::player::Player;
use super::Character;

pub struct Card<'a> {
    owner: &'a Player<'a>, // or just str?
    character: Character,
    strength: f32,
}

impl Card<'_> {
    pub fn new(owner: &Player, character: Character, strength: f32) -> Card {
        Card {
            owner,
            strength,
            character,
        }
    }
}
