extern crate core;

mod game;

use kotc_actix::start_actix_server;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    start_actix_server().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::game::{card::{Card, Character}, Token, column::Column, Resource};

    #[test]
    fn test_column_eval() {
        let cards = vec![
            Card::new(String::from("ccc"), Character::Dvojnik, 0.0),
            Card::new(String::from("bbb"), Character::Dvojnik, 0.0),
            Card::new(String::from("aaa"), Character::Kral, 20.0),
            Card::new(String::from("aaa"), Character::Kralovna, 16.0),
            Card::new(String::from("aaa"), Character::Julie, 14.0),
            Card::new(String::from("aaa"), Character::Alchymista, 8.0),
            Card::new(String::from("aaa"), Character::Sermir, 8.0),
            Card::new(String::from("bbb"), Character::Statkar, 8.0),
            Card::new(String::from("bbb"), Character::Kupec, 8.0),
            Card::new(String::from("aaa"), Character::Kardinal, 8.0),
            Card::new(String::from("bbb"), Character::Trubadur, 8.0),
            Card::new(String::from("aaa"), Character::Objevitel, 13.0),
            Card::new(String::from("aaa"), Character::Mordyr, 9.5),
            Card::new(String::from("aaa"), Character::Boure, 9.0),
            // Card::new(String::from("ccc"), Character::Prevlek, 0.0),
            // Card::new(String::from("aaa"), Character::Zradca, 10.0),
            // Card::new(String::from("aaa"), Character::Musketyri, 11.0),
            // Card::new(String::from("ccc"), Character::Mag, 7.0),
            // Card::new(String::from("aaa"), Character::Carodejnice, 1.0),
            // Card::new(String::from("bbb"), Character::Princ, 14.0),
            Card::new(String::from("bbb"), Character::Panos, 2.0),
            Card::new(String::from("aaa"), Character::Poustevnik, 12.0),
            Card::new(String::from("ccc"), Character::Palecek, 2.0),
            Card::new(String::from("aaa"), Character::Drak, 11.0),
            Card::new(String::from("aaa"), Character::Romeo, 5.0),
            // Card::new(String::from("aaa"), Character::Zebrak, 4.0),
        ];
        let mut column = Column::new(Token {resource: Resource::Coins, points: 3});
        cards.into_iter().for_each(|card| column.add_card(card));
        println!("AND THE WINNER IS: {}", column.eval());
    }
}
