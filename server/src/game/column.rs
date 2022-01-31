use super::card::{Card, Character};
use crate::game::Token;

use anyhow::Result;
use itertools::Itertools;
use super::Resource;

type GameResult = Vec<(String, f32)>;

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
            Resource::Fiddle => Character::Trubadur,
            Resource::Flask => Character::Alchymista,
            Resource::Hat => Character::Kardinal,
            Resource::Swords => Character::Sermir,
        };
        self.cards.iter_mut().for_each(|card| if card.character == bonus_character {card.strength = 12.0});

        let charactes: Vec<Character> = self.cards.iter().map(|card| card.character).collect();
        let muscatiers = charactes.contains(&Character::Musketyri);
        let hags = charactes.iter().filter(|&&character| character == Character::Carodejnice).count() == 1;
        let mages = charactes.iter().filter(|&&character| character == Character::Mag).count() == 1;

        if muscatiers {
            return self.get_winner(&mut self.get_results(), true);
        }

        if mages {
            self.cards.retain(|card| card.strength < 10.0);
        }
        if hags {
            self.cards.retain(|card| 
                card.strength > 9.0 || card.character == Character::Carodejnice || card.character == Character::Dvojnik /* TODO or Dvojnik no?*/
            );
        }

        let num_of_cards = self.cards.len() as f32;

        self.cards.iter_mut().for_each(|card| match card.character {
            Character::Poustevnik => card.strength = f32::max(0.0, card.strength - num_of_cards + 1.0),
            Character::Palecek => card.strength += (num_of_cards - 1.0) * 3.0,
            _ => {},
        });

        match self.eval_by_owner(false) {
            Some(winner) => return winner,
            None => (),
        }
        self.set_doubler_points();
        self.get_winner(&mut self.get_results(), false)
    }

    pub fn eval_by_owner(&mut self, muscatiers: bool) -> Option<String> {
        let cards_by_owners = self.cards
            .iter_mut()
            .map(|card| (card.owner.clone(), card))
            .into_group_map();
        for (owner, cards) in cards_by_owners {
            let characters_by_owner: Vec<Character> = cards.iter().map(|card| card.character).collect();
            if !muscatiers && characters_by_owner.contains(&Character::Princ) && characters_by_owner.contains(&Character::Panos) {
                return Some(owner);
            }
            if characters_by_owner.contains(&Character::Julie) {
                cards.into_iter().filter(|card| card.character == Character::Romeo).for_each(|card| card.strength = 15.0);
            }
            // TODO drak
        }
        None
    }

    fn set_doubler_points(&mut self) {
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

    fn get_results(&self) -> GameResult {
        let cards_by_owners = self.cards
            .iter()
            .map(|card| (card.owner.clone(), card))
            .into_group_map();
        let res: GameResult = cards_by_owners.iter().map(|(owner, cards)| {
            let strength = cards.iter().map(|card| card.strength).sum();
            println!("{}: {:?}", owner.clone(), cards);
            (owner.clone(), strength)
        }).collect();
        println!("RESULTS:\n{:?}", res);
        res
    }

    fn get_winner(&self, points: &mut GameResult, muscatiers: bool) -> String {
        let charactes: Vec<Character> = self.cards.iter().map(|card| card.character).collect();
        let beggar = charactes.iter().filter(|&&character| character == Character::Zebrak).count() >= 1;

        let compare: for<'r, 's> fn(&'r (String, f32), &'s (String, f32)) -> _ = if !muscatiers && beggar {
            |(_, r1), (_, r2)| r1.partial_cmp(r2).unwrap_or(std::cmp::Ordering::Equal)
        } else {
            |(_, r1), (_, r2)| r2.partial_cmp(r1).unwrap_or(std::cmp::Ordering::Equal)
        };

        // TODO kto je vyssie pri remize???
        points.sort_by(compare);
        println!("WINNER:\n{:?}", points[0]);
        points[0].0.clone()
    }
}
