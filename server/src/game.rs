use crate::game::column::Column;
use crate::game::player::Player;
use crate::game::token::Token;
use kotc_database::models::User;
use std::collections::HashMap;

pub mod card;
pub mod column;
pub mod player;
pub mod token;

const NUMBER_OF_ROUNDS: u8 = 6;

enum Character {
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

pub struct Game<'a> {
    players: Vec<&'a Player<'a>>,
    columns: Vec<&'a Column<'a>>,
    token_deck: Vec<Token>,
}

impl Game<'_> {
    pub fn new(users: Vec<&User>) -> Game {
        let players = users.iter().map(|&user| Player::new(user)).collect();
        let mut game = Game {
            players,
            columns: Vec::new(),
            token_deck: Vec::new(),
        };
        game.init_token_deck();
        game
    }

    fn init_token_deck(&mut self) {
        // TODO: calculate number of tokens based of number of players
        // TODO: fill with tokens
        // TODO: shuffle
    }

    pub fn start_game(&mut self) {
        while !self.token_deck.is_empty() {
            self.round()
        }
    }

    fn round(&mut self) {}

    pub fn get_results(&self) { // -> HashMap<String, u8> {
    }

    pub fn print_players(&self) {
        println!("{:?}", self.players)
    }
}
