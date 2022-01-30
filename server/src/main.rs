extern crate core;

mod game;

use std::sync::Arc;

use crate::game::Game;

use kotc_database::{
    establish_connection,
    repo::{
        game_repo::{GameRepo, PostgresGameRepo},
        participation_repo::{ParticipationRepo, PostgresParticipationRepo},
        user_repo::{PostgresUserRepo, UserRepo},
    },
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let conn_pool = Arc::new(establish_connection().await);

    let user_repo = Arc::new(PostgresUserRepo::new(conn_pool.clone()));
    let game_repo = Arc::new(PostgresGameRepo::new(conn_pool.clone()));
    let participation_repo = Arc::new(PostgresParticipationRepo::new(conn_pool.clone()));

    let user_id = user_repo
        .create_user("Puckoland", "k@gmail.com", "aba", "asdfjkn")
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
        .update_user(user_id, Some("new name"), None, None, Some("password"))
        .await?;
    println!("{:?}", user);

    participation_repo
        .delete_participation(participation_id)
        .await?;
    game_repo.delete_game(game_id).await?;
    user_repo.delete_user(user_id).await?;

    let mut game = Game::new(vec![&user, &user2]);
    game.print_players();
    game.start_game().await;

    Ok(())
}
