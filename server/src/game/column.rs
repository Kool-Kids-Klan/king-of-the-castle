use anyhow::Result;
use itertools::{chain, Itertools};
use std::cmp::Ordering;

use super::card::{Card, Character};
use crate::game::Token;
use crate::game::player::Player;
use super::Resource;

type ColumnResults = Vec<(String, f32)>;

#[derive(Clone, Debug)]
pub struct Column {
    pub token: Token,
    blocked: bool,  // Boure
    cards: Vec<Card>,
}

impl Column {
    pub fn new(token: Token) -> Column {
        Column {
            token,
            blocked: false,
            cards: vec![],
        }
    }

    pub fn is_completed(&self) -> bool {
        self.cards.len() as u8 >= self.token.points || self.blocked
    }

    fn reveal_last_card(&mut self) {
        if !self.cards.is_empty() {
            let last_index = self.cards.len() - 1;
            self.cards[last_index].revealed = true;
        }
    }

    pub fn add_card(&mut self, card: Card)  {
        self.reveal_last_card();
        self.cards.push(card);
    }

    pub fn eval(&mut self) -> String {
        let bonus_character = match self.token.resource {
            Resource::Coins => Character::Kupec,
            Resource::Corn => Character::Statkar,
            Resource::Hat => Character::Kardinal,
            Resource::Fiddle => Character::Trubadur,
            Resource::Swords => Character::Sermir,
            Resource::Flask => Character::Alchymista,
        };
        self.cards.iter_mut().for_each(|card| if card.character == bonus_character {card.strength = 12.0});

        let characters: Vec<Character> = self.cards.iter().map(|card| card.character).collect();
        let musketeers = characters.contains(&Character::Musketyri);
        let witches = characters.iter().filter(|&&character| character == Character::Carodejnice).count() == 1;
        let mages = characters.iter().filter(|&&character| character == Character::Mag).count() == 1;

        if musketeers {
            return self.get_winner(&mut self.get_results(), true);
        }

        if mages {
            self.cards.retain(|card| card.strength < 10.0);
        }

        if witches {
            self.cards.retain(|card| 
                card.strength > 9.0 || card.character == Character::Carodejnice || card.character == Character::Dvojnik
            );
        }

        let num_of_cards = self.cards.len() as f32;
        self.cards.iter_mut().for_each(|card| match card.character {
            Character::Poustevnik => card.strength = f32::max(0.0, card.strength - num_of_cards + 1.0),
            Character::Palecek => card.strength += (num_of_cards - 1.0) * 3.0,
            _ => {},
        });

        let all_with_combo = self.get_players_with_prince_squire_combo();
        match self.get_player_with_highest_prince_squire_combo(all_with_combo) {
            Some(player) => return player,
            None => ()
        }

        self.set_mirrorer_points();
        self.get_winner(&mut self.get_results(), false)
    }

    pub fn get_players_with_prince_squire_combo(&mut self) -> Vec<String> {
        // apply Romeo+Julia buff on the way
        // TODO drak debuff

        let mut players = vec![];
        let cards_by_owners = self.cards
            .iter_mut()
            .map(|card| (card.owner.clone(), card))
            .into_group_map();
        for (owner, cards) in cards_by_owners {
            let characters: Vec<Character> = cards.iter().map(|card| card.character).collect();

            if characters.contains(&Character::Princ) && characters.contains(&Character::Panos) {
                players.push(owner);
            }

            if characters.contains(&Character::Julie) {
                cards.into_iter()
                    .filter(|card| card.character == Character::Romeo)
                    .for_each(|card| card.strength = 15.0);
            }
        }
        players
    }

    fn get_player_with_highest_prince_squire_combo(&self, all_with_combo: Vec<String>) -> Option<String> {
        let winner = self.cards.iter().find(| card| {
            all_with_combo.contains(&card.owner) && [Character::Princ, Character::Panos].contains(&card.character)
        });
        match winner {
            Some(card) => Some(card.clone().owner),
            None => None
        }
    }

    fn set_mirrorer_points(&mut self) {
        let mut iterator = self.cards.iter_mut().rev().peekable();
        while let Some(card) = iterator.next() {
            match iterator.peek_mut() {
                Some(next) => {
                    if next.character == Character::Dvojnik {
                        next.strength = card.strength;
                    }
                },
                None => (),
            }
        }
    }

    fn get_results(&self) -> ColumnResults {
        let cards_by_owners = self.cards
            .iter()
            .map(|card| (card.owner.clone(), card))
            .into_group_map();
        let result: ColumnResults = cards_by_owners.iter().map(|(owner, cards)| {
            let strength = cards.iter().map(|card| card.strength).sum();
            println!("{}: {:?}", owner.clone(), cards);
            (owner.clone(), strength)
        }).collect();
        println!("RESULTS:\n{:?}", result);
        result
    }


    fn get_winner(&self, results: &mut ColumnResults, musketeers: bool) -> String {
        let characters: Vec<Character> = self.cards.iter().map(|card| card.character).collect();
        let beggar = characters.iter().filter(|&&character| character == Character::Zebrak).count() >= 1;

        let compare: for<'r, 's> fn(&'r (String, f32), &'s (String, f32)) -> _ = if !musketeers && beggar {
            |(_, r1), (_, r2)| r1.partial_cmp(r2).unwrap_or(std::cmp::Ordering::Equal)
        } else {
            |(_, r1), (_, r2)| r2.partial_cmp(r1).unwrap_or(std::cmp::Ordering::Equal)
        };

        results.sort_by(compare);
        println!("WINNER:\n{:?}", results[0]);
        results[0].0.clone()
    }
}
