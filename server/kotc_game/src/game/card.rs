use serde::{Deserialize, Serialize};

use super::player::Color;

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
    pub color: Color,
    pub character: Character,
    pub strength: f32,
    pub revealed: bool,
}

impl Card {
    pub fn new(owner: String, color: Color, character: Character, strength: f32) -> Card {
        Card {
            owner,
            color,
            strength,
            character,
            revealed: false,
        }
    }

    pub fn dummy_card(owner: String, color: Color) -> Card {
        Card::new(owner, color, Character::Unknown, 0.0)
    }
}
