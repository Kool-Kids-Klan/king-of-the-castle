use std::panic;
use std::{thread, time};
// use std::collections::HashMap;
use itertools::{cloned, iproduct, Itertools};
use rand::{seq::IteratorRandom, thread_rng};

use crate::game::column::Column;
use crate::game::player::Player;
use kotc_database::models::User;

pub mod card;
pub mod column;
pub mod player;

const NUMBER_OF_ROUNDS: u8 = 6;

#[derive(Clone, Debug)]
pub enum Resource {
    Coins,
    Corn,
    Hat,
    Fiddle,
    Swords,
    Flask,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub resource: Resource,
    pub points: u8
}

fn get_resource_types() -> [Resource; 6] {
    [Resource::Coins, Resource::Corn, Resource::Hat, Resource::Fiddle, Resource::Swords, Resource::Flask]
}

// TODO move this to kotc_actix and import it from there
struct Action {
    pub card: u8,
    pub column: u8
}

#[derive(Clone)]
pub struct Game<'a> {
    players: Vec<Player<'a>>,
    columns: Vec<Column>,
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
        iproduct!(get_resource_types(), [1, 2, 3, 3, 4, 5])
            .choose_multiple(&mut rng, number_of_players * NUMBER_OF_ROUNDS as usize)
            .into_iter()
            .map(|(resource, points)| Token {resource, points})
            .collect()
    }

    pub async fn start_game(&mut self) -> Option<()> {
        while !self.token_deck.is_empty() {
            self.round().await?;
            break;
        }
        Some(())
    }

    async fn round(&mut self) -> Option<()> {
        self.columns = vec![];
        (0..self.players.len()).into_iter().for_each(|_| {
            let token = match self.token_deck.pop() {
                Some(token) => token,
                None => panic!("Bad number of tokens in token deck!")
            };
            self.columns.push(Column::new(token));
        });

        let players = self.players.clone();
        let mut player_it = players.iter().cycle();
        while self.columns.iter().any(|column| !column.is_completed()) {
            // self.make_action(player_it.next()?).await;
            break;
        }
        self.eval_columns();
        Some(())
    }

    async fn make_action(&mut self, player: &mut Player<'_>) {
        println!("{}, make action!", player.user.username);
        let action = 0;// let action = wait_for_action().await?;
        println!("completed");

        player.draw_card();
    }

    fn eval_columns(&mut self) {
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

    fn get_player_by_username(&self) {
        // alebo rovno assign_points_to_player(username)?
    }

    pub fn print_players(&self) {
        println!("{:?}", self.players)
    }
}

async fn wait() {
    let ten_millis = time::Duration::from_millis(5000);
    thread::sleep(ten_millis);
}
