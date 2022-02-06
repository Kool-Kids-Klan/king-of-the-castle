use crate::game::{get_all_resources, Resource};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

use super::card::{Card, Character};
use super::{Token, User};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub user_id: i32,
    pub username: String,
    pub hand: Vec<Card>,
    pub deck: Vec<Card>,
    pub tokens: Vec<Token>,
    pub ready: bool,
}

impl Player {
    pub fn new(user: User) -> Player {
        let mut player = Player {
            user_id: user.id,
            username: user.username,
            hand: vec![],
            deck: vec![],
            tokens: vec![],
            ready: false,
        };
        player.refill_deck();
        player.draw_card();
        player.draw_card();
        player.draw_card();
        player
    }

    pub fn flip_ready(&mut self) {
        self.ready = !self.ready;
    }

    fn refill_deck(&mut self) {
        let except: Vec<Character> = self
            .hand
            .clone()
            .into_iter()
            .map(|card| card.character)
            .collect();
        let refilled_deck = vec![
            Card::new(self.username.clone(), Character::King, 20.0),
            Card::new(self.username.clone(), Character::Queen, 16.0),
            Card::new(self.username.clone(), Character::Julia, 14.0),
            Card::new(self.username.clone(), Character::Alchemist, 8.0),
            Card::new(self.username.clone(), Character::Swordsman, 8.0),
            Card::new(self.username.clone(), Character::Landlord, 8.0),
            Card::new(self.username.clone(), Character::Merchant, 8.0),
            Card::new(self.username.clone(), Character::Cardinal, 8.0),
            Card::new(self.username.clone(), Character::Troubadour, 8.0),
            Card::new(self.username.clone(), Character::Explorer, 13.0),
            Card::new(self.username.clone(), Character::Killer, 9.5),
            Card::new(self.username.clone(), Character::Storm, 9.0),
            // Card::new(self.username.clone(), Character::Prevlek, 0.0),
            // Card::new(self.username.clone(), Character::Zradca, 10.0),
            Card::new(self.username.clone(), Character::Musketeers, 11.0),
            Card::new(self.username.clone(), Character::Mage, 7.0),
            Card::new(self.username.clone(), Character::Witch, 1.0),
            Card::new(self.username.clone(), Character::Prince, 14.0),
            Card::new(self.username.clone(), Character::Squire, 2.0),
            Card::new(self.username.clone(), Character::Hermit, 12.0),
            Card::new(self.username.clone(), Character::Thumb, 2.0),
            Card::new(self.username.clone(), Character::Doppelganger, 0.0),
            Card::new(self.username.clone(), Character::Dragon, 11.0),
            Card::new(self.username.clone(), Character::Romeo, 5.0),
            Card::new(self.username.clone(), Character::Beggar, 4.0),
        ]
        .into_iter()
        .filter(|card| !except.contains(&card.character))
        .collect();
        let mut rng = thread_rng();
        self.deck = refilled_deck;
        self.deck.shuffle(&mut rng);
    }

    fn next_card(&mut self) -> (Card, bool) {
        // returns (card, refilled?)
        match self.deck.pop() {
            Some(card) => (card, false),
            None => {
                self.refill_deck();
                (self.next_card().0, true)
            }
        }
    }

    pub fn draw_card(&mut self) -> bool {
        let (card, refilled) = self.next_card();
        self.hand.push(card.clone());
        refilled
    }

    pub fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn get_obtained_resources(&self) -> Vec<Resource> {
        self.tokens
            .iter()
            .map(|token| token.resource.clone())
            .collect()
    }

    fn get_token_points(&self) -> u8 {
        self.tokens.iter().map(|token| token.points).sum()
    }

    pub fn has_all_resource_types(&self) -> bool {
        let my_resources = self.get_obtained_resources();
        get_all_resources()
            .map(|resource| my_resources.contains(&resource))
            .iter()
            .all(|&x| x)
    }

    pub fn get_score(&self) -> u8 {
        let points = self.get_token_points();
        // TODO ak ma double, tak za kazdu extra kartu -1 bod
        if self.has_all_resource_types() {
            points * 2
        } else {
            points
        }
    }
}
