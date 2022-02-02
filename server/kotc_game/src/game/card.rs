use super::player::Player;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Copy, Serialize, Deserialize)]
pub enum Character {
    Unknown, // not a real character, used for hidden cards etc.
    King,
    Queen,
    Julia,
    Alchemist,
    Swordsman,
    Landlord,
    Merchant,
    Cardinal,
    Troubadour,
    Explorer,
    Killer,
    Storm,
    Cloak,
    Traitor,
    Musketeers,
    Mage,
    Witch,
    Prince,
    Squire,
    Hermit,
    Thumb,
    Doppelganger,
    Dragon,
    Romeo,
    Beggar,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Card {
    pub owner: String,
    pub character: Character,
    pub strength: f32,
    pub revealed: bool,
}

impl Card {
    pub fn new(owner: String, character: Character, strength: f32) -> Card {
        Card {
            owner,
            strength,
            character,
            revealed: false,
        }
    }

    pub fn dummy_card() -> Card {
        Card::new("".to_string(), Character::Unknown, 0.0)
    }
}
