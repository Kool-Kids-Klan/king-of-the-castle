use rand::seq::SliceRandom;
use rand::thread_rng;
use crate::game::{get_all_resources, Resource};

use super::card::{Card, Character};
use super::{User, Token};

#[derive(Clone, Debug)]
pub struct Player {
    pub user: User,
    pub hand: Vec<Card>,
    deck: Vec<Card>,
    tokens: Vec<Token>,
    ready: bool
}

impl Player {
    pub fn new(user: User) -> Player {
        let mut player = Player {
            user,
            hand: vec![],
            deck: vec![],
            tokens: vec![],
            ready: false
        };
        player.refill_deck();
        player.draw_card();
        // TODO send message "Add card to hand"
        player.draw_card();
        // TODO send message "Add card to hand"
        player.draw_card();
        // TODO send message "Add card to hand"
        player
    }

    pub fn flip_ready(&mut self) {
        self.ready = !self.ready;
    }

    fn refill_deck(&mut self) {
        let except: Vec<Character> = self.hand.clone()
            .into_iter()
            .map(|card| card.character)
            .collect();
        let initial_deck = vec![
            Card::new(self.user.username.clone(), Character::King, 20.0),
            Card::new(self.user.username.clone(), Character::Queen, 16.0),
            Card::new(self.user.username.clone(), Character::Julia, 14.0),
            Card::new(self.user.username.clone(), Character::Alchemist, 8.0),
            Card::new(self.user.username.clone(), Character::Swordsman, 8.0),
            Card::new(self.user.username.clone(), Character::Landlord, 8.0),
            Card::new(self.user.username.clone(), Character::Merchant, 8.0),
            Card::new(self.user.username.clone(), Character::Cardinal, 8.0),
            Card::new(self.user.username.clone(), Character::Troubadour, 8.0),
            // Card::new(self.user.username.clone(), Character::Explorer, 13.0),
            Card::new(self.user.username.clone(), Character::Killer, 9.5),
            Card::new(self.user.username.clone(), Character::Storm, 9.0),
            // Card::new(self.user.username.clone(), Character::Prevlek, 0.0),
            // Card::new(self.user.username.clone(), Character::Zradca, 10.0),
            Card::new(self.user.username.clone(), Character::Musketeers, 11.0),
            Card::new(self.user.username.clone(), Character::Mage, 7.0),
            Card::new(self.user.username.clone(), Character::Witch, 1.0),
            Card::new(self.user.username.clone(), Character::Prince, 14.0),
            Card::new(self.user.username.clone(), Character::Squire, 2.0),
            Card::new(self.user.username.clone(), Character::Hermit, 12.0),
            Card::new(self.user.username.clone(), Character::Thumb, 2.0),
            Card::new(self.user.username.clone(), Character::Doppelganger, 0.0),
            Card::new(self.user.username.clone(), Character::Dragon, 11.0),
            Card::new(self.user.username.clone(), Character::Romeo, 5.0),
            Card::new(self.user.username.clone(), Character::Beggar, 4.0),
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

    pub fn draw_card(&mut self) -> Card {
        let card = self.next_card();
        self.hand.push(card.clone());
        card
    }

    pub fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn get_obtained_resources(&self) -> Vec<Resource> {
        self.tokens.iter().map(|token| token.clone().resource).collect()
    }

    fn get_token_points(&self) -> u8 {
        self.tokens.iter().map(|token| token.points).sum()
    }

    pub fn get_score(&self) -> u8 {
        let points = self.get_token_points();
        let my_resources = self.get_obtained_resources();
        let has_all = get_all_resources()
            .map(|resource| my_resources.contains(&resource))
            .iter().any(|&x| x);
        // TODO ak ma double, tak za kazdu extra kartu -1 bod
        if has_all {points*2} else {points}
    }
}
