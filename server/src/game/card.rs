use std::rc::Rc;

use super::player::Player;

#[derive(Clone, Debug, PartialEq)]
pub enum Character {
    Kral,
    Kralovna,
    Julie,
    Alchymista,
    Sermir,
    Statkar,
    Kupec,
    Kardinal,
    Trubadur,
    Objevitel,
    Mordyr,
    Boure,
    Prevlek,
    Zradca,
    Musketyri,
    Mag,
    Carodejnice,
    Princ,
    Panos,
    Poustevnik,
    Palecek,
    Dvojnik,
    Drak,
    Romeo,
    Zebrak,
}

#[derive(Clone, Debug)]
pub struct Card {
    owner: String,
    pub character: Character,
    strength: f32,
    pub revealed: bool
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
