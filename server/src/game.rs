// use std::collections::HashMap;
use itertools::iproduct;
use rand::{seq::IteratorRandom, thread_rng};

use crate::game::column::Column;
use crate::game::player::Player;
use kotc_database::models::User;

pub mod card;
pub mod column;
pub mod player;

const NUMBER_OF_ROUNDS: u8 = 6;

#[derive(Clone, Debug)]
pub enum Character {
    Kral,
    Kralovna,
    Julie,
    Alchymista,
    Sermir,
    Statkar,
    Kupec,
    Kardinal,
    Trubadur,
    Objevitel,
    Mordyr,
    Boure,
    Prevlek,
    Zradca,
    Musketyri,
    Mag,
    Carodejnice,
    Princ,
    Panos,
    Poustevnik,
    Palecek,
    Dvojnik,
    Drak,
    Romeo,
    Zebrak,
}

#[derive(Clone, Debug)]
pub enum Token {
    Coins(u8),
    Corn(u8),
    Hat(u8),
    Fiddle(u8),
    Swords(u8),
    Flask(u8),
}

fn get_token_types() -> [fn(u8) -> Token; 6] {
    [Token::Coins, Token::Corn, Token::Hat, Token::Fiddle, Token::Swords, Token::Flask]
}

#[derive(Clone)]
pub struct Game<'a> {
    players: Vec<Player<'a>>,
    columns: Vec< Column>,
    token_deck: Vec<Token>,
}

impl Game<'_> {
    pub fn new(users: Vec<&User>) -> Game {
        let players = users.iter().map(|&user| Player::new(user)).collect();
        Game {
            players,
            columns: vec![],
            token_deck: Game::init_token_deck(users.len()),
        }
    }

    fn init_token_deck(number_of_players: usize) -> Vec<Token> {
        let mut rng = thread_rng();
        iproduct!([1, 2, 3, 3, 4, 5], get_token_types())
            .choose_multiple(&mut rng, number_of_players * NUMBER_OF_ROUNDS as usize)
            .iter()
            .map(|(points, resource)| resource(*points))
            .collect()
    }

    pub fn start_game(&mut self) {
        while !self.token_deck.is_empty() {
            self.round()
        }
    }

    fn round(&mut self) {
        // TODO create fresh columns
        // TODO make action for each player
        // TODO repeat until all columns are completed
        // TODO calculate winners and assign points
    }

    pub fn get_results(&self) { // -> HashMap<String, u8> {

    }

    pub fn print_players(&self) {
        println!("{:?}", self.players)
    }
}
