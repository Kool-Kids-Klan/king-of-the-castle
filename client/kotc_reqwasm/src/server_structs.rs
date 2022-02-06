use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Resource {
    Coins,
    Corn,
    Hat,
    Lute,
    Swords,
    Flask,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Token {
    pub resource: Resource,
    pub points: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Color {
    Black,
    White,
    Yellow,
    Red,
    Green,
    Blue,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub user_id: i32,
    pub username: String,
    pub color: Color,
    pub hand: Vec<Card>,
    deck: Vec<Card>,
    tokens: Vec<Token>,
    pub ready: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Column {
    pub token: Token,
    pub is_blocked: bool,
    pub cards: Vec<Card>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Card {
    pub owner: String,
    pub color: Color,
    pub character: Character,
    pub strength: f32,
    pub revealed: bool,
}

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
