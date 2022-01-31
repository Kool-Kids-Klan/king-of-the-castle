extern crate core;

mod game;

use std::sync::Arc;

use kotc_actix::start_actix_server;
use crate::game::{Game, column::Column, Token};

use game::card::{Card, Character};
use kotc_database::{
    establish_connection,
    repo::{
        game_repo::{GameRepo, PostgresGameRepo},
        participation_repo::{ParticipationRepo, PostgresParticipationRepo},
        user_repo::{PostgresUserRepo, UserRepo},
    },
};

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let conn_pool = Arc::new(establish_connection().await);

    let user_repo = Arc::new(PostgresUserRepo::new(conn_pool.clone()));
    let game_repo = Arc::new(PostgresGameRepo::new(conn_pool.clone()));
    let participation_repo = Arc::new(PostgresParticipationRepo::new(conn_pool.clone()));

    let user_id = user_repo
        .create_user("Puckoland", "k@gmail.com", "asdfjkn")
        .await?;
    let user = user_repo.get_user(user_id).await?;
    println!("{:?}", user);
    let user2_id = user_repo
        .create_user("Dante", "d@gmail.com", "bab", "dgfvbzsd<a")
        .await?;
    let user2 = user_repo.get_user(user2_id).await?;
    println!("{:?}", user2);

    let game_id = game_repo.create_game().await?;
    let game = game_repo.get_game(game_id).await?;
    println!("{:?}", game);

    let participation_id = participation_repo
        .create_participation(game_id, user_id)
        .await?;
    let participation = participation_repo
        .get_participation(participation_id)
        .await?;
    println!("{:?}", participation);

    let won_game = game_repo.update_game_winner(game_id, user_id).await?;
    println!("{:?}", won_game);
    let user = user_repo
        .update_user(user_id, Some("new name"), None, Some("password"))
        .await?;
    println!("{:?}", user);

    participation_repo
        .delete_participation(participation_id)
        .await?;
    game_repo.delete_game(game_id).await?;
    user_repo.delete_user(user_id).await?;

    start_actix_server().await?;

    let mut game = Game::new(vec![&user, &user2]);
    game.start_game().await;

    test_column_eval();

    Ok(())
}

fn test_column_eval() {
    let cards = vec![
        Card::new(String::from("ccc"), Character::Dvojnik, 0.0),
        Card::new(String::from("bbb"), Character::Dvojnik, 0.0),
        Card::new(String::from("aaa"), Character::Kral, 20.0),
        Card::new(String::from("aaa"), Character::Kralovna, 16.0),
        Card::new(String::from("aaa"), Character::Julie, 14.0),
        Card::new(String::from("aaa"), Character::Alchymista, 8.0),
        Card::new(String::from("aaa"), Character::Sermir, 8.0),
        Card::new(String::from("bbb"), Character::Statkar, 8.0),
        Card::new(String::from("bbb"), Character::Kupec, 8.0),
        Card::new(String::from("aaa"), Character::Kardinal, 8.0),
        Card::new(String::from("bbb"), Character::Trubadur, 8.0),
        Card::new(String::from("aaa"), Character::Objevitel, 13.0),
        Card::new(String::from("aaa"), Character::Mordyr, 9.5),
        Card::new(String::from("aaa"), Character::Boure, 9.0),
        // Card::new(String::from("ccc"), Character::Prevlek, 0.0),
        // Card::new(String::from("aaa"), Character::Zradca, 10.0),
        // Card::new(String::from("aaa"), Character::Musketyri, 11.0),
        // Card::new(String::from("ccc"), Character::Mag, 7.0),
        // Card::new(String::from("aaa"), Character::Carodejnice, 1.0),
        // Card::new(String::from("bbb"), Character::Princ, 14.0),
        Card::new(String::from("bbb"), Character::Panos, 2.0),
        Card::new(String::from("aaa"), Character::Poustevnik, 12.0),
        Card::new(String::from("ccc"), Character::Palecek, 2.0),
        Card::new(String::from("aaa"), Character::Drak, 11.0),
        Card::new(String::from("aaa"), Character::Romeo, 5.0),
        // Card::new(String::from("aaa"), Character::Zebrak, 4.0),
    ];
    let mut column = Column::new(Token {resource: game::Resource::Coins, points: 3});
    cards.into_iter().for_each(|card| column.add_card(card));
    println!("AND THE WINNER IS: {}", column.eval());
}
