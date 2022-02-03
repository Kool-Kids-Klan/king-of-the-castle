use std::{panic};
use std::collections::HashMap;
// use std::collections::HashMap;
use itertools::{iproduct};
use rand::{seq::IteratorRandom, thread_rng};
use serde::{Deserialize, Serialize};

use card::Card;
use column::Column;
use player::Player;
use ws_messages::{MessageRecipient, ServerMessage, Error};
use kotc_database::models::User;
use kotc_commons::messages::message_types::ServerWsMessageType;
use crate::game::card::Character;

pub mod card;
pub mod column;
pub mod player;
mod utils;
pub mod ws_messages;

const NUMBER_OF_ROUNDS: u8 = 6;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Resource {
    Coins,
    Corn,
    Hat,
    Fiddle,
    Swords,
    Flask,
}

fn get_all_resources() -> [Resource; 6] {
    [
        Resource::Coins,
        Resource::Corn,
        Resource::Hat,
        Resource::Fiddle,
        Resource::Swords,
        Resource::Flask,
    ]
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Token {
    pub resource: Resource,
    pub points: u8,
}

#[derive(Clone)]
pub struct Game {
    id: i32,
    players: Vec<Player>,
    player_on_turn: usize,
    columns: Vec<Column>,
    token_deck: Vec<Token>,
    started: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
            id: 0,
            players: vec![],
            player_on_turn: 0,
            columns: vec![],
            token_deck: vec![],
            started: false,
        }
    }

    pub async fn add_player(&mut self, user_id: i32) {
        match utils::find_user_by_id(user_id).await {
            Ok(user) => self.players.push(Player::new(user)),
            Err(_) => panic!("User not found."),
        };
    }

    pub fn player_flip_ready(&mut self, user_id: i32) {
        if let Some(player) = self.get_player_by_id(user_id) {
            player.flip_ready();
        } else {
            // TODO send message "ERROR: Invalid player id."
        }
    }

    fn get_player_by_id(&mut self, user_id: i32) -> Option<&mut Player> {
        self.players.iter_mut().find(|p| p.user_id == user_id)
    }

    fn init_token_deck(&mut self) -> Vec<Token> {
        let mut rng = thread_rng();
        iproduct!(get_all_resources(), [1, 2, 3, 3, 4, 5])
            .choose_multiple(&mut rng, self.players.len() * NUMBER_OF_ROUNDS as usize)
            .into_iter()
            .map(|(resource, points)| Token { resource, points })
            .collect()
    }

    fn draw_next_tokens(&mut self) {
        (0..self.players.len()).into_iter().for_each(|_| {
            let token = match self.token_deck.pop() {
                Some(token) => token,
                None => panic!("Error: Bad number of tokens in token deck!")
            };
            self.columns.push(Column::new(token));
        });
        // TODO send message "Update columns"
    }

    pub async fn start_game(&mut self) {
        self.id = utils::create_new_game_in_db(&self.players).await;
        self.started = true;
        self.init_token_deck();
        self.draw_next_tokens();
    }

    pub fn make_action(&mut self, user_id: i32, column_index: usize, card_index: usize) {
        if let Some(_) = self.get_player_by_id(user_id) {
        } else {
            // TODO send message "ERROR: Invalid player id."
            return;
        }

        let mut played_card: Card = Card::dummy_card();
        match self.players.get(self.player_on_turn) {
            Some(player_on_turn) => {
                if user_id != player_on_turn.user_id {
                    // TODO send message "ERROR: It's not your turn."
                    return;
                }
                match player_on_turn.hand.get(card_index) {
                    Some(c) => {
                        played_card = c.clone();
                    }
                    None => {
                        // TODO send message "ERROR: invalid card index."
                        return;
                    }
                }
            }
            None => {} // This cannot happen, self.player_on_turn is always in range
        };

        match self.columns.get_mut(column_index) {
            Some(column) => {
                if column.blocked {
                    // TODO send message "ERROR: Column is blocked by Storm."
                    return;
                } else {
                    column.add_card(played_card);
                    // TODO send message "Update columns"
                    if let Some(character) = column.reveal_previous_card() {
                        // TODO send message "Update columns"
                        if character == Character::Killer {
                            column.pop_card();
                            // TODO send message "Update columns"
                        }
                    }

                    if let Some(p) = self.get_player_by_id(user_id) {
                        // always true
                        // TODO send message "Remove card from hand"
                        p.hand.remove(card_index);
                    }
                }
            }
            None => {
                // TODO send message "ERROR: Invalid column."
                return;
            }
        }

        // TODO maybe try to implement Rc+Refcell for self.players, so it can be borrowed multiple times and get_player_by_id() would only need to be called once
        if let Some(p) = self.get_player_by_id(user_id) {
            // always true
            // TODO send message "Add card to hand"
            p.draw_card();
        }
        self.player_on_turn = (self.player_on_turn + 1) % self.players.len();

        if self.round_finished() {
            self.eval_columns();
            if self.token_deck.is_empty() {
                // TODO finish game
            } else {
                self.columns = vec![];
                self.draw_next_tokens();
            }
        }
    }

    fn round_finished(&self) -> bool {
        self.columns.iter().all(|column| column.is_completed())
    }

    fn eval_columns(&mut self) {
        // evaluates columns and adds tokens to the players
        self.columns.iter_mut().for_each(|column| {
            let winner = column.eval();
            let win_player = match self
                .players
                .iter_mut()
                .filter(|player| player.username == winner)
                .next()
            {
                Some(player) => player,
                None => panic!("Error while evaluating winner!"),
            };
            win_player.add_token(column.token.clone());
            // TODO send message "add token"
        });
    }

    fn get_results(&self) -> (String, HashMap<String, u8>) {
        let mut results: HashMap<String, u8> = self.players.iter()
            .map(|player| (player.clone().username, player.get_score()))
            .collect();
        let mut winner = "Unknown".to_string();
        if let Some((username, _)) = results
            .iter()
            .max_by_key(|(_, &score)| score) {
            winner = username.clone();
        }
        (winner, results)
    }

    fn finish_game(&mut self, winner_id: i32) {
        utils::update_game_result_in_db(self.id, winner_id);
        let (winner, results) = self.get_results();
        // TODO send message "Game finished" + results
    }
}
