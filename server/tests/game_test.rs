extern crate kotc_game;

use kotc_game::game::{
    card::{Card, Character},
    column::Column,
    player::Color,
    Resource, Token,
};

#[test]
fn test_column_eval() {
    let cards = vec![
        Card::new(
            String::from("ccc"),
            Color::Black,
            Character::Doppelganger,
            0.0,
        ),
        Card::new(
            String::from("bbb"),
            Color::Black,
            Character::Doppelganger,
            0.0,
        ),
        Card::new(String::from("aaa"), Color::Black, Character::King, 20.0),
        Card::new(String::from("aaa"), Color::Black, Character::Queen, 16.0),
        Card::new(String::from("aaa"), Color::Black, Character::Julia, 14.0),
        Card::new(String::from("aaa"), Color::Black, Character::Alchemist, 8.0),
        Card::new(String::from("aaa"), Color::Black, Character::Swordsman, 8.0),
        Card::new(String::from("bbb"), Color::Black, Character::Landlord, 8.0),
        Card::new(String::from("bbb"), Color::Black, Character::Merchant, 8.0),
        Card::new(String::from("aaa"), Color::Black, Character::Cardinal, 8.0),
        Card::new(
            String::from("bbb"),
            Color::Black,
            Character::Troubadour,
            8.0,
        ),
        Card::new(String::from("aaa"), Color::Black, Character::Explorer, 13.0),
        Card::new(String::from("aaa"), Color::Black, Character::Killer, 9.5),
        Card::new(String::from("aaa"), Color::Black, Character::Storm, 9.0),
        // Card::new(String::from("ccc"), Color::Black, Character::Prevlek, 0.0),
        // Card::new(String::from("aaa"), Color::Black, Character::Zradca, 10.0),
        // Card::new(String::from("aaa"), Color::Black, Character::Musketyri, 11.0),
        // Card::new(String::from("ccc"), Color::Black, Character::Mag, 7.0),
        // Card::new(String::from("aaa"), Color::Black, Character::Carodejnice, 1.0),
        // Card::new(String::from("bbb"), Color::Black, Character::Princ, 14.0),
        Card::new(String::from("bbb"), Color::Black, Character::Squire, 2.0),
        Card::new(String::from("aaa"), Color::Black, Character::Hermit, 12.0),
        Card::new(String::from("ccc"), Color::Black, Character::Thumb, 2.0),
        Card::new(String::from("aaa"), Color::Black, Character::Dragon, 11.0),
        Card::new(String::from("aaa"), Color::Black, Character::Romeo, 5.0),
        // Card::new(String::from("aaa"), Color::Black, Character::Zebrak, 4.0),
    ];
    let mut column = Column::new(Token {
        resource: Resource::Coins,
        points: 3,
    });
    cards.into_iter().for_each(|card| column.add_card(card));
    println!("AND THE WINNER IS: {}", column.eval());
}
