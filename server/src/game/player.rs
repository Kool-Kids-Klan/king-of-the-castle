use rand::seq::SliceRandom;
use rand::thread_rng;

use super::card::{Card, Character};
use super::User;

#[derive(Clone, Debug)]
pub struct Player<'a> {
    pub user: &'a User,
    hand: Vec<Card>,
    deck: Vec<Card>,
    score: u8,
}

impl Player<'_> {
    pub fn new(user: &User) -> Player {
        let mut player = Player {
            user,
            hand: vec![],
            deck: vec![],
            score: 0,
        };
        player.refill_deck();
        player.draw_card();
        player.draw_card();
        player.draw_card();
        player
    }

    fn refill_deck(&mut self) {
        let except: Vec<Character> = self.hand.clone()
            .into_iter()
            .map(|card| card.character)
            .collect();
        let initial_deck = vec![
            Card::new(self.user.username.clone(), Character::Kral, 20.0),
            Card::new(self.user.username.clone(), Character::Kralovna, 16.0),
            Card::new(self.user.username.clone(), Character::Julie, 14.0),
            Card::new(self.user.username.clone(), Character::Alchymista, 8.0),
            Card::new(self.user.username.clone(), Character::Sermir, 8.0),
            Card::new(self.user.username.clone(), Character::Statkar, 8.0),
            Card::new(self.user.username.clone(), Character::Kupec, 8.0),
            Card::new(self.user.username.clone(), Character::Kardinal, 8.0),
            Card::new(self.user.username.clone(), Character::Trubadur, 8.0),
            Card::new(self.user.username.clone(), Character::Objevitel, 13.0),
            Card::new(self.user.username.clone(), Character::Mordyr, 9.5),
            Card::new(self.user.username.clone(), Character::Boure, 9.0),
            Card::new(self.user.username.clone(), Character::Prevlek, 0.0),
            Card::new(self.user.username.clone(), Character::Zradca, 10.0),
            Card::new(self.user.username.clone(), Character::Musketyri, 11.0),
            Card::new(self.user.username.clone(), Character::Mag, 7.0),
            Card::new(self.user.username.clone(), Character::Carodejnice, 1.0),
            Card::new(self.user.username.clone(), Character::Princ, 14.0),
            Card::new(self.user.username.clone(), Character::Panos, 2.0),
            Card::new(self.user.username.clone(), Character::Poustevnik, 12.0),
            Card::new(self.user.username.clone(), Character::Palecek, 2.0),
            Card::new(self.user.username.clone(), Character::Dvojnik, 0.0),
            Card::new(self.user.username.clone(), Character::Drak, 11.0),
            Card::new(self.user.username.clone(), Character::Romeo, 5.0),
            Card::new(self.user.username.clone(), Character::Zebrak, 4.0),
        ].into_iter().filter(|card| !except.contains(&card.character)).collect();
        let mut rng = thread_rng();
        self.deck = initial_deck;
        self.deck.shuffle(&mut rng);
    }

    fn next_card(&mut self) -> Card {
        match self.deck.pop() {
            Some(card) => card,
            None => {
                self.refill_deck();
                self.next_card()
            }
        }
    }

    pub fn draw_card(&mut self) {
        let card = self.next_card();
        self.hand.push(card);
    }
}
