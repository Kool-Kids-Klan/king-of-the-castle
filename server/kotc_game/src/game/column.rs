use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

use super::card::{Card, Character};
use super::Resource;
use crate::game::Token;

type ColumnResult = (String, f32);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Column {
    pub token: Token,
    pub is_blocked: bool, // Boure
    pub cards: Vec<Card>,
}

impl Column {
    pub fn new(token: Token) -> Column {
        Column {
            token,
            is_blocked: false,
            cards: vec![],
        }
    }

    pub fn is_completed(&self) -> bool {
        self.cards.len() as u8 >= self.token.points || self.is_blocked
    }

    pub fn reveal_previous_card(&mut self) -> Option<Card> {
        let number_of_cards = self.cards.len();
        if self.cards.len() >= 2 {
            let mut previous_card = self.cards.get_mut(number_of_cards - 2).unwrap();
            if previous_card.revealed {
                None
            } else {
                previous_card.revealed = true;
                Some(previous_card.clone())
            }
        } else {
            None
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn remove_revealed_explorer(&mut self) -> Card {
        let mut explorer = match self
            .cards
            .iter()
            .find(|card| card.character == Character::Explorer && card.revealed)
        {
            Some(explorer) => explorer.clone(),
            None => panic!("There is not a revealed explorer in the column."),
        };
        explorer.revealed = false;
        self.cards
            .retain(|card| card.character != Character::Explorer || card.owner != explorer.owner);
        explorer
    }

    pub fn eval(&mut self) -> String {
        let bonus_character = match self.token.resource {
            Resource::Coins => Character::Merchant,
            Resource::Corn => Character::Landlord,
            Resource::Hat => Character::Cardinal,
            Resource::Lute => Character::Troubadour,
            Resource::Swords => Character::Swordsman,
            Resource::Flask => Character::Alchemist,
        };
        self.cards.iter_mut().for_each(|card| {
            if card.character == bonus_character {
                card.strength = 12.0
            }
        });

        let characters: Vec<Character> = self.cards.iter().map(|card| card.character).collect();
        let musketeers = characters.contains(&Character::Musketeers);
        let witches = characters
            .iter()
            .filter(|&&character| character == Character::Witch)
            .count()
            == 1;
        let mages = characters
            .iter()
            .filter(|&&character| character == Character::Mage)
            .count()
            == 1;

        if musketeers {
            return self.get_winner(&self.get_results(), true);
        }

        if mages {
            self.cards.retain(|card| card.strength < 10.0);
        }

        if witches {
            self.cards.retain(|card| {
                card.strength > 9.0
                    || card.character == Character::Witch
                    || card.character == Character::Doppelganger
            });
        }

        let num_of_cards = self.cards.len() as f32;
        self.cards.iter_mut().for_each(|card| match card.character {
            Character::Hermit => card.strength = f32::max(0.0, card.strength - num_of_cards + 1.0),
            Character::Thumb => card.strength += (num_of_cards - 1.0) * 3.0,
            _ => {}
        });

        let all_with_combo = self.get_players_with_prince_squire_combo();
        if let Some(player) = self.get_player_with_highest_prince_squire_combo(all_with_combo) {
            return player;
        }

        self.set_mirrorer_points();
        self.get_winner(&self.get_results(), false)
    }

    fn get_players_with_prince_squire_combo(&mut self) -> Vec<String> {
        // apply Romeo+Julia buff on the way
        // TODO drak debuff

        let mut players = vec![];
        let cards_by_owners = self
            .cards
            .iter_mut()
            .map(|card| (card.owner.clone(), card))
            .into_group_map();
        for (owner, cards) in cards_by_owners {
            let characters: Vec<Character> = cards.iter().map(|card| card.character).collect();

            if characters.contains(&Character::Prince) && characters.contains(&Character::Squire) {
                players.push(owner);
            }

            if characters.contains(&Character::Julia) {
                cards
                    .into_iter()
                    .filter(|card| card.character == Character::Romeo)
                    .for_each(|card| card.strength = 15.0);
            }
        }
        players
    }

    fn get_player_with_highest_prince_squire_combo(
        &self,
        all_with_combo: Vec<String>,
    ) -> Option<String> {
        let winner = self.cards.iter().find(|card| {
            all_with_combo.contains(&card.owner)
                && [Character::Prince, Character::Squire].contains(&card.character)
        });
        winner.map(|card| card.clone().owner)
    }

    fn get_player_highest_card_index(&self, player: String) -> usize {
        let result = self.cards.iter().find_position(|card| card.owner == player);
        match result {
            Some((index, _)) => index,
            None => panic!("Player doesn't have any card in this column."),
        }
    }

    fn set_mirrorer_points(&mut self) {
        let mut iterator = self.cards.iter_mut().rev().peekable();
        while let Some(card) = iterator.next() {
            if let Some(next) = iterator.peek_mut() {
                if next.character == Character::Doppelganger {
                    next.strength = card.strength;
                }
            }
        }
    }

    fn get_results(&self) -> Vec<ColumnResult> {
        let cards_by_owners = self
            .cards
            .iter()
            .map(|card| (card.owner.clone(), card))
            .into_group_map();
        let result: Vec<ColumnResult> = cards_by_owners
            .iter()
            .map(|(owner, cards)| {
                let strength = cards.iter().map(|card| card.strength).sum();
                (owner.clone(), strength)
            })
            .collect();
        result
    }

    fn comparator(
        (highest_a, score_a): (&usize, &f32),
        (highest_b, score_b): (&usize, &f32),
        beggar_activated: bool,
    ) -> Option<Ordering> {
        if score_a != score_b {
            if beggar_activated {
                score_a.partial_cmp(score_b)
            } else {
                score_b.partial_cmp(score_a)
            }
        } else if beggar_activated {
            highest_b.partial_cmp(highest_a)
        } else {
            highest_a.partial_cmp(highest_b)
        }
    }

    fn get_winner(&self, results: &[ColumnResult], musketeers: bool) -> String {
        let characters: Vec<Character> = self.cards.iter().map(|card| card.character).collect();
        let beggar = characters
            .iter()
            .filter(|&&character| character == Character::Beggar)
            .count()
            >= 1;

        let compare: for<'r, 's> fn(&'r (usize, &f32), &'s (usize, &f32)) -> _ = if !musketeers
            && beggar
        {
            |(h1, s1), (h2, s2)| {
                Column::comparator((h1, s1), (h2, s2), true).unwrap_or(std::cmp::Ordering::Equal)
            }
        } else {
            |(h1, s1), (h2, s2)| {
                Column::comparator((h1, s1), (h2, s2), false).unwrap_or(std::cmp::Ordering::Equal)
            }
        };

        results
            .iter()
            .map(|(player, score)| {
                (
                    self.get_player_highest_card_index(player.to_string()),
                    score,
                )
            })
            .collect::<Vec<(usize, &f32)>>()
            .sort_by(compare);

        match results.get(0) {
            Some(result) => result.0.clone(),
            None => panic!("Empty column."),
        }
    }

    pub fn get_concealed_cards(&self) -> Vec<Card> {
        self.cards
            .clone()
            .into_iter()
            .map(|card| {
                if card.revealed {
                    card
                } else {
                    Card::dummy_card(card.owner, card.color)
                }
            })
            .collect()
    }
}
