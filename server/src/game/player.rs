use rand::seq::SliceRandom;
use rand::thread_rng;

use super::card::Card;
use super::User;
use crate::game::Character;

#[derive(Debug)]
pub struct Player<'a> {
    user: &'a User,
    deck: Vec<Card<'a>>,
    score: u8,
}

impl Player<'_> {
    pub fn new(user: &User) -> Player {
        let mut player = Player {
            user,
            deck: vec![],
            score: 0,
        };
        player.refill_deck();
        player
    }

    fn refill_deck(&mut self) {
        let initial_deck: Vec<Card> = vec![
            Card::new(&self, Character::Kral, 20.0),
            Card::new(&self, Character::Kralovna, 16.0),
            Card::new(&self, Character::Julie, 14.0),
            Card::new(&self, Character::Alchymista, 8.0),
            Card::new(&self, Character::Sermir, 8.0),
            Card::new(&self, Character::Statkar, 8.0),
            Card::new(&self, Character::Kupec, 8.0),
            Card::new(&self, Character::Kardinal, 8.0),
            Card::new(&self, Character::Trubadur, 8.0),
            Card::new(&self, Character::Objevitel, 13.0),
            Card::new(&self, Character::Mordyr, 9.5),
            Card::new(&self, Character::Boure, 9.0),
            Card::new(&self, Character::Prevlek, 0.0),
            Card::new(&self, Character::Zradca, 10.0),
            Card::new(&self, Character::Musketyri, 11.0),
            Card::new(&self, Character::Mag, 7.0),
            Card::new(&self, Character::Carodejnice, 1.0),
            Card::new(&self, Character::Princ, 14.0),
            Card::new(&self, Character::Panos, 2.0),
            Card::new(&self, Character::Poustevnik, 12.0),
            Card::new(&self, Character::Palecek, 2.0),
            Card::new(&self, Character::Dvojnik, 0.0),
            Card::new(&self, Character::Drak, 11.0),
            Card::new(&self, Character::Romeo, 5.0),
            Card::new(&self, Character::Zebrak, 4.0),
        ];
        //  TODO: shuffle
        let mut rng = thread_rng();
        self.deck = initial_deck;
        self.deck.shuffle(&mut rng);
    }
}
