use itertools::iproduct;
use rand::{seq::IteratorRandom, thread_rng};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::panic;
use std::rc::Rc;

use card::{Card, Character};
use column::Column;
use kotc_commons::messages::message_types::ServerWsMessageType;
use kotc_database::models::User;
use player::Player;
use ws_messages::{
    ActionLog, Error, FinishGame, MessageRecipient, ServerMessage, StartGame, UpdateColumns,
    UpdateHand, UpdatePlayers, UpdateTokens,
};

use crate::game::player::Color;

pub mod card;
pub mod column;
pub mod player;
mod utils;
pub mod ws_messages;

const NUMBER_OF_ROUNDS: u8 = 1;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Resource {
    Coins,
    Corn,
    Hat,
    Lute,
    Swords,
    Flask,
}

fn get_all_resources() -> [Resource; 6] {
    [
        Resource::Coins,
        Resource::Corn,
        Resource::Hat,
        Resource::Lute,
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
    players: Rc<RefCell<Vec<Player>>>,
    players_count: usize,
    player_on_turn: usize,
    columns: Rc<RefCell<Vec<Column>>>,
    token_deck: Vec<Token>,
    round: u8,
    available_colors: Vec<Color>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            id: 0,
            players: Rc::new(RefCell::new(vec![])),
            players_count: 0,
            player_on_turn: 0,
            columns: Rc::new(RefCell::new(vec![])),
            token_deck: vec![],
            round: 1,
            available_colors: vec![
                Color::Black,
                Color::White,
                Color::Red,
                Color::Yellow,
                Color::Green,
                Color::Blue,
            ],
        }
    }

    pub async fn connect_player(&mut self, user_id: i32) -> Vec<ServerMessage> {
        if let Some(_) = Rc::clone(&self.players)
            .borrow()
            .iter()
            .find(|player| player.user_id == user_id)
        {
            return vec![self.message_error(format!(
                "Error: User with ID {} is already in the lobby.",
                user_id
            ))];
        };
        match utils::find_user_by_id(user_id).await {
            Ok(user) => {
                let mut messages = vec![];

                let new_player = Player::new(user, self.available_colors.pop().unwrap()); // TODO unwrap
                Rc::clone(&self.players)
                    .borrow_mut()
                    .push(new_player.clone());
                self.players_count += 1;

                messages.push(self.message_update_players());
                messages.push(self.log(format!(
                    "Player {} has joined the lobby.",
                    new_player.username
                )));
                messages.push(self.message_update_hand(new_player.hand));
                messages
            }
            Err(_) => {
                vec![self.message_error(format!("Error: User with ID {} not found.", user_id))]
            }
        }
    }

    pub fn disconnect_player(&mut self, user_id: i32) -> Vec<ServerMessage> {
        let player = Rc::clone(&self.players)
            .borrow()
            .iter()
            .find(|player| player.user_id == user_id)
            .cloned();
        match player {
            Some(player) => {
                let mut messages = vec![];

                Rc::clone(&self.players)
                    .borrow_mut()
                    .retain(|player| player.user_id != user_id);
                self.players_count -= 1;

                messages.push(self.log(format!("Player {} left the lobby.", player.username)));
                messages.push(self.message_update_players());
                messages
            }
            None => {
                vec![self.message_error(format!("Error: User with ID {} not found.", user_id))]
            }
        }
    }

    fn get_ready_count(&self) -> usize {
        Rc::clone(&self.players)
            .borrow()
            .iter()
            .filter(|player| player.ready)
            .count()
    }

    pub async fn player_flip_ready(&mut self, user_id: i32) -> Vec<ServerMessage> {
        let mut messages = vec![];
        match Rc::clone(&self.players)
            .borrow_mut()
            .iter_mut()
            .find(|p| p.user_id == user_id)
        {
            Some(player) => {
                player.flip_ready();
            }
            None => {
                messages.push(
                    self.message_error(format!("Error: User with ID {} not found.", user_id)),
                );
                return messages;
            }
        };

        messages.push(self.message_update_players());

        if self.get_ready_count() >= 2 {
            self.start_game().await;
            messages.push(self.message_start_game());
            messages.push(self.log(format!(
                "Game has started.\nRound {}/{} has started.\n{} is on turn.",
                self.round,
                NUMBER_OF_ROUNDS,
                self.get_player_on_turn_username()
            )));
            messages.push(self.message_update_columns());
        };
        messages
    }

    fn get_player_on_turn_username(&self) -> String {
        Rc::clone(&self.players)
            .borrow()
            .get(self.player_on_turn) // always in range
            .unwrap()
            .username
            .clone()
    }

    fn init_token_deck(&mut self) {
        let mut rng = thread_rng();
        self.token_deck = iproduct!(get_all_resources(), [1, 2, 3, 3, 4, 5])
            .choose_multiple(&mut rng, self.players_count * NUMBER_OF_ROUNDS as usize)
            .into_iter()
            .map(|(resource, points)| Token { resource, points })
            .collect()
    }

    fn draw_next_tokens(&mut self) {
        (0..self.players_count).into_iter().for_each(|_| {
            let token = match self.token_deck.pop() {
                Some(token) => token,
                None => panic!("Error: Bad number of tokens in token deck!"),
            };
            Rc::clone(&self.columns)
                .borrow_mut()
                .push(Column::new(token));
        });
    }

    async fn start_game(&mut self) {
        let players: Vec<Player> = Rc::clone(&self.players).borrow().to_vec();
        self.id = utils::create_new_game_in_db(players).await;
        self.init_token_deck();
        self.draw_next_tokens();
    }

    pub async fn make_action(
        &mut self,
        user_id: i32,
        column_index: usize,
        card_index: usize,
    ) -> Vec<ServerMessage> {
        if self.round > 6 {
            return vec![self.message_error("Game already ended.".to_string())];
        }

        if let Some(_) = Rc::clone(&self.players)
            .borrow_mut()
            .iter_mut()
            .find(|p| p.user_id == user_id)
        {
        } else {
            return vec![self.message_error(format!("Error: User with ID {} not found.", user_id))];
        }

        let mut played_card: Card = Card::dummy_card("".to_string(), Color::Black);
        match Rc::clone(&self.players).borrow().get(self.player_on_turn) {
            Some(player_on_turn) => {
                if user_id != player_on_turn.user_id {
                    return vec![self.message_error("Error: It's not your turn.".to_string())];
                }
                match player_on_turn.hand.get(card_index) {
                    Some(c) => {
                        played_card = c.clone();
                    }
                    None => {
                        return vec![self
                            .message_error(format!("Error: Invalid card index: {}", card_index))];
                    }
                }
            }
            None => {} // This cannot happen, self.player_on_turn is always in range
        }

        let mut messages = vec![];
        let blocked;
        match Rc::clone(&self.columns).borrow().get(column_index) {
            Some(column) => {
                blocked = column.is_blocked;
            }
            None => {
                return vec![
                    self.message_error(format!("Error: Invalid column index: {}.", column_index))
                ];
            }
        }
        if blocked {
            return vec![self.message_error("Error: Column is blocked by Storm.".to_string())];
        } else {
            if let Some(p) = Rc::clone(&self.players)
                .borrow_mut()
                .iter_mut()
                .find(|p| p.user_id == user_id)
            {
                // always true
                p.hand.remove(card_index);
                messages.push(self.message_update_hand(p.hand.clone()));

                self.push_card_to_column(column_index, played_card, &mut messages);

                if p.draw_card() {
                    messages
                        .push(self.log(format!("Player {} has refilled his deck.", p.username)));
                }
                messages.push(self.message_update_hand(p.hand.clone()));
            }
        }

        self.player_on_turn = (self.player_on_turn + 1) % self.players_count;
        self.message_update_players();

        if self.round_finished() {
            messages.push(self.log(format!("All columns closed - round ended.")));
            self.eval_columns(&mut messages);

            self.round += 1;
            if self.token_deck.is_empty() {
                let (winner_id, winner_username, results) = self.get_results();

                messages.push(
                    self.log(
                        results
                            .iter()
                            .fold("Results:\n".to_string(), |s, (username, score)| {
                                s + &format!("{}: {} points\n", username, score)
                            })
                            + &format!("Winner: {}\n", winner_username),
                    ),
                );

                messages.push(self.message_finish_game(winner_username, results));
                utils::update_game_result_in_db(self.id, winner_id).await;
                return messages;
            }
            self.columns = Rc::new(RefCell::new(vec![]));
            self.draw_next_tokens();
            messages.push(self.log(format!(
                "Round {}/{} has started.",
                self.round, NUMBER_OF_ROUNDS
            )));
            messages.push(self.message_update_columns());
        }
        messages.push(self.log(format!(
            "{} is on turn.",
            self.get_player_on_turn_username()
        )));
        messages
    }

    fn push_card_to_column(
        &self,
        column_index: usize,
        played_card: Card,
        messages: &mut Vec<ServerMessage>,
    ) {
        Rc::clone(&self.columns)
            .borrow_mut()
            .get_mut(column_index)
            .unwrap()
            .add_card(played_card.clone());
        messages.push(self.message_update_columns());
        messages.push(self.log(format!(
            "{}'s card has been added into column {}.",
            played_card.owner,
            column_index + 1
        )));

        let revealed = Rc::clone(&self.columns)
            .borrow_mut()
            .get_mut(column_index)
            .unwrap()
            .reveal_previous_card();
        if let Some(revealed_card) = revealed {
            messages.push(self.message_update_columns());
            messages.push(self.log(format!(
                "{}'s {:?} has been revealed.",
                revealed_card.owner, revealed_card.character
            )));

            match revealed_card.character {
                Character::Killer => {
                    Rc::clone(&self.columns)
                        .borrow_mut()
                        .get_mut(column_index)
                        .unwrap()
                        .cards
                        .pop();
                    messages.push(self.message_update_columns());
                    messages.push(self.log(format!(
                        "{}'s played card has been removed by Killer!",
                        played_card.owner
                    )));
                }
                Character::Storm => {
                    Rc::clone(&self.columns)
                        .borrow_mut()
                        .get_mut(column_index)
                        .unwrap()
                        .is_blocked = true;
                    messages.push(self.log(format!(
                        "From now on, column {} is blocked by Storm.",
                        column_index + 1
                    )));
                }
                Character::Explorer => {
                    let explorer_card = Rc::clone(&self.columns)
                        .borrow_mut()
                        .get_mut(column_index)
                        .unwrap()
                        .remove_revealed_explorer();
                    let next_column_index = (column_index + 1) % self.players_count;
                    messages.push(self.log(format!(
                        "{}'s Explorer moves to the right (from column {} to column {}).",
                        revealed_card.owner,
                        column_index + 1,
                        next_column_index + 1
                    )));
                    self.push_card_to_column(next_column_index, explorer_card, messages);
                }
                _ => (),
            }
        }
    }

    fn round_finished(&self) -> bool {
        Rc::clone(&self.columns)
            .borrow()
            .iter()
            .all(|column| column.is_completed())
    }

    fn eval_columns(&mut self, messages: &mut Vec<ServerMessage>) {
        // evaluates columns and adds tokens to the players
        Rc::clone(&self.columns)
            .borrow_mut()
            .iter_mut()
            .for_each(|column| {
                let winner_username = column.eval();
                messages.push(self.log(format!(
                    "{:?} for {} points won by {}",
                    column.token.resource, column.token.points, winner_username
                )));
                if let Some(winner) = Rc::clone(&self.players)
                    .borrow_mut()
                    .iter_mut()
                    .find(|player| player.username == winner_username)
                {
                    let had_all = winner.has_all_resource_types();
                    winner.add_token(column.token.clone());
                    if !had_all && winner.has_all_resource_types() {
                        messages.push(self.log(format!(
                            "{} obtained all resource types, therefore his points \
                             will be doubled. Who can stop this guy?",
                            winner.username
                        )));
                    }
                }
            });
        messages.push(self.message_update_tokens());
    }

    fn get_results(&self) -> (i32, String, HashMap<String, u8>) {
        let results: HashMap<String, u8> = Rc::clone(&self.players)
            .borrow()
            .iter()
            .map(|player| (player.clone().username, player.get_score()))
            .collect();
        let mut winner_id = 0;
        let mut winner_username = "Unknown".to_string();
        if let Some((username, _)) = results.iter().max_by_key(|(_, &score)| score) {
            winner_username = username.clone();
        }
        if let Some(p) = Rc::clone(&self.players)
            .borrow()
            .clone()
            .iter_mut()
            .find(|p| p.username == winner_username)
        {
            winner_id = p.user_id;
        }
        (winner_id, winner_username, results)
    }

    /// UTIL MESSAGE FUNCTIONS ///

    fn message_update_players(&self) -> ServerMessage {
        let mut players: Vec<Player> = Rc::clone(&self.players).borrow().to_vec().clone();
        players.iter_mut().for_each(|player| {
            player.hand = vec![];
            player.deck = vec![];
        });

        ServerMessage {
            message_type: ServerWsMessageType::UpdatePlayers,
            recipient: MessageRecipient::AllUsers,
            content: serde_json::to_string(&UpdatePlayers { players }).unwrap(),
        }
    }

    fn message_update_hand(&self, hand: Vec<Card>) -> ServerMessage {
        ServerMessage {
            message_type: ServerWsMessageType::UpdateHand,
            recipient: MessageRecipient::SingleUser,
            content: serde_json::to_string(&UpdateHand { hand }).unwrap(),
        }
    }

    fn message_update_columns(&self) -> ServerMessage {
        let mut columns = Rc::clone(&self.columns).borrow().to_vec();
        columns.iter_mut().for_each(|column| {
            column.cards = column.get_concealed_cards();
        });
        ServerMessage {
            message_type: ServerWsMessageType::UpdateColumns,
            recipient: MessageRecipient::AllUsers,
            content: serde_json::to_string(&UpdateColumns { columns }).unwrap(),
        }
    }

    fn message_update_tokens(&self) -> ServerMessage {
        let tokens: HashMap<String, (Color, Vec<Token>)> = Rc::clone(&self.players)
            .borrow()
            .iter()
            .map(|player| (player.username.clone(), (player.color.clone() , player.tokens.clone())))
            .collect();
        ServerMessage {
            message_type: ServerWsMessageType::UpdateTokens,
            recipient: MessageRecipient::AllUsers,
            content: serde_json::to_string(&UpdateTokens { tokens }).unwrap(),
        }
    }

    fn message_finish_game(&self, winner: String, results: HashMap<String, u8>) -> ServerMessage {
        ServerMessage {
            message_type: ServerWsMessageType::FinishGame,
            recipient: MessageRecipient::AllUsers,
            content: serde_json::to_string(&FinishGame { winner, results }).unwrap(),
        }
    }

    fn log(&self, detail: String) -> ServerMessage {
        ServerMessage {
            message_type: ServerWsMessageType::ActionLog,
            recipient: MessageRecipient::AllUsers,
            content: serde_json::to_string(&ActionLog { detail }).unwrap(),
        }
    }

    fn message_error(&self, detail: String) -> ServerMessage {
        ServerMessage {
            message_type: ServerWsMessageType::Error,
            recipient: MessageRecipient::SingleUser,
            content: serde_json::to_string(&Error { detail }).unwrap(),
        }
    }

    fn message_start_game(&self) -> ServerMessage {
        ServerMessage {
            message_type: ServerWsMessageType::StartGame,
            recipient: MessageRecipient::AllUsers,
            content: serde_json::to_string(&StartGame {}).unwrap(),
        }
    }
}
