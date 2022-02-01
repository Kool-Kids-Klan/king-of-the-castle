use std::panic;
use std::{thread, time};
// use std::collections::HashMap;
use itertools::{cloned, iproduct, Itertools};
use rand::{seq::IteratorRandom, thread_rng};

use column::Column;
use player::Player;
use kotc_database::models::User;
use card::Character;

pub mod card;
pub mod column;
pub mod player;

const NUMBER_OF_ROUNDS: u8 = 6;

#[derive(Clone, Debug, PartialEq)]
pub enum Resource {
    Coins,
    Corn,
    Hat,
    Fiddle,
    Swords,
    Flask,
}

fn get_all_resources() -> [Resource; 6] {
    [Resource::Coins, Resource::Corn, Resource::Hat, Resource::Fiddle, Resource::Swords, Resource::Flask]
}

#[derive(Clone, Debug)]
pub struct Token {
    pub resource: Resource,
    pub points: u8
}


// TODO move this to kotc_actix and import it from there
struct Action {
    pub card: u8,
    pub column: u8
}

#[derive(Clone)]
pub struct Game<'a> {
    players: Vec<Player<'a>>,
    player_on_turn: u8,
    columns: Vec<Column>,
    token_deck: Vec<Token>,
    started: bool
}

impl Game<'_> {
    pub fn new(users: Vec<&User>) -> Game {
        let players = users.iter().map(|&user| Player::new(user)).collect();
        Game {
            players,
            player_on_turn: 0,
            columns: vec![],
            token_deck: vec![],
            started: false
        }
    }

    pub fn add_player(&mut self, user: &User) {
        // FIXME
        // self.players.push(Player::new(user));
    }

    fn init_token_deck(&mut self) -> Vec<Token> {
        let mut rng = thread_rng();
        iproduct!(get_all_resources(), [1, 2, 3, 3, 4, 5])
            .choose_multiple(&mut rng, self.players.len() * NUMBER_OF_ROUNDS as usize)
            .into_iter()
            .map(|(resource, points)| Token {resource, points})
            .collect()
    }

    pub fn start_game(&mut self) {
        self.init_token_deck();

        // while !self.token_deck.is_empty() {
        //     self.round().await?;
        //     break;
        // }
    }

    // async fn round(&mut self) -> Option<()> {
    //     self.columns = vec![];
    //     (0..self.players.len()).into_iter().for_each(|_| {
    //         let token = match self.token_deck.pop() {
    //             Some(token) => token,
    //             None => panic!("Bad number of tokens in token deck!")
    //         };
    //         self.columns.push(Column::new(token));
    //     });
    //
    //     let players = self.players.clone();
    //     let mut player_it = players.iter().cycle();
    //     while self.columns.iter().any(|column| !column.is_completed()) {
    //         // self.make_action(player_it.next()?).await;
    //         break;
    //     }
    //     self.eval_columns();
    //     Some(())
    // }

    pub fn make_action(&mut self,
                   player_id: i32,
                   column_index: u8,
                   card_index: Character) {
        let mut player_ref: &Player;
        match self.get_player_by_id(player_id) {
            Some(p) => {
                player_ref = p;
            },
            None => {}
        };
        // if let Some(&player_obj) = self.players.get(player_id) {
        //     if player_id != self.player_on_turn {
        //         // TODO send message "ERROR: It's not your turn."
        //         return;
        //     }
        // } else {
        //     // TODO send message "ERROR: Invalid player id."
        //     return;
        // }
        // if let Some(&column) = self.columns.get(column_index) {
        //     if column.blocked {
        //         // TODO send message "ERROR: Column is blocked by Storm."
        //         return;
        //     }
        // } else {
        //     // TODO send message "ERROR: Invalid column."
        //     return;
        // }

        // FIXME draw card
        // player_ref.draw_card();
        // TODO send message "Add card to hand"
        self.player_on_turn = (self.player_on_turn + 1) % self.players.len() as u8;
    }

    fn eval_columns(&mut self) {
        // evaluates columns and adds tokens to the players
        self.columns.iter_mut().for_each(|column| {
            let winner = column.eval();
            println!("AND THE WINNER OF COLUMN IS: {}", winner);
            let win_player = match self.players.iter_mut().filter(|player| player.user.username == winner).next() {
                Some(player) => player,
                None => panic!("Error evaluating winner happened!"),
            };
            win_player.add_token(column.token.clone());
        });
    }

    fn get_player_by_id(&self, player_id: i32) -> Option<&Player> {
        self.players.iter().find(|p| p.user.id == player_id)
    }

}
