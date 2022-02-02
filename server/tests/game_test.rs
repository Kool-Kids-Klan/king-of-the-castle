extern crate kotc_game;

use kotc_game::game::{card::{Card, Character}, Token, column::Column, Resource};

#[test]
fn test_column_eval() {
    let cards = vec![
        Card::new(String::from("ccc"), Character::Doppelganger, 0.0),
        Card::new(String::from("bbb"), Character::Doppelganger, 0.0),
        Card::new(String::from("aaa"), Character::King, 20.0),
        Card::new(String::from("aaa"), Character::Queen, 16.0),
        Card::new(String::from("aaa"), Character::Julia, 14.0),
        Card::new(String::from("aaa"), Character::Alchemist, 8.0),
        Card::new(String::from("aaa"), Character::Swordsman, 8.0),
        Card::new(String::from("bbb"), Character::Landlord, 8.0),
        Card::new(String::from("bbb"), Character::Merchant, 8.0),
        Card::new(String::from("aaa"), Character::Cardinal, 8.0),
        Card::new(String::from("bbb"), Character::Troubadour, 8.0),
        Card::new(String::from("aaa"), Character::Explorer, 13.0),
        Card::new(String::from("aaa"), Character::Killer, 9.5),
        Card::new(String::from("aaa"), Character::Storm, 9.0),
        // Card::new(String::from("ccc"), Character::Prevlek, 0.0),
        // Card::new(String::from("aaa"), Character::Zradca, 10.0),
        // Card::new(String::from("aaa"), Character::Musketyri, 11.0),
        // Card::new(String::from("ccc"), Character::Mag, 7.0),
        // Card::new(String::from("aaa"), Character::Carodejnice, 1.0),
        // Card::new(String::from("bbb"), Character::Princ, 14.0),
        Card::new(String::from("bbb"), Character::Squire, 2.0),
        Card::new(String::from("aaa"), Character::Hermit, 12.0),
        Card::new(String::from("ccc"), Character::Thumb, 2.0),
        Card::new(String::from("aaa"), Character::Dragon, 11.0),
        Card::new(String::from("aaa"), Character::Romeo, 5.0),
        // Card::new(String::from("aaa"), Character::Zebrak, 4.0),
    ];
    let mut column = Column::new(Token {resource: Resource::Coins, points: 3});
    cards.into_iter().for_each(|card| column.add_card(card));
    println!("AND THE WINNER IS: {}", column.eval());
}
