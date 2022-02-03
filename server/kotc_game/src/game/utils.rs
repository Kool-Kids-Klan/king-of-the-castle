use anyhow::Result;
use kotc_database::{get_game_repo, get_participation_repo, get_user_repo};

use crate::game::player::Player;
use kotc_database::models::User;
use kotc_database::repo::game_repo::GameRepo;
use kotc_database::repo::participation_repo::ParticipationRepo;
use kotc_database::repo::user_repo::UserRepo;

pub async fn find_user_by_id(id: i32) -> Result<User> {
    get_user_repo().await.get_user(id).await
}

// call when starting the game
pub async fn create_new_game_in_db(players: &Vec<Player>) -> i32 {
    let game_id = match get_game_repo().await.create_game().await {
        Ok(game_id) => game_id,
        Err(_) => panic!("Game creation failed."),
    };
    for player in players {
        get_participation_repo()
            .await
            .create_participation(game_id, player.user_id)
            .await;
    }
    game_id
}

// call when finishing the game
pub async fn update_game_result_in_db(game_id: i32, winner_id: i32) {
    get_game_repo()
        .await
        .update_game_winner(game_id, winner_id)
        .await;
}
