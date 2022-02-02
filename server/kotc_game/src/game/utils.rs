use anyhow::Result;
use chrono::{NaiveDateTime, Utc};
use kotc_database::{get_game_repo, get_user_repo};

use kotc_database::models::User;
use kotc_database::repo::game_repo::GameRepo;
use kotc_database::repo::user_repo::UserRepo;

pub async fn find_user_by_id(id: i32) -> Result<User> {
    get_user_repo().await.get_user(id).await
}

pub async fn create_new_game_in_db() -> i32 {
    match get_game_repo().await.create_game().await {
        Ok(game_id) => game_id,
        Err(_) => panic!("Game creation failed."),
    }
}

pub async fn update_game_in_db(game_id: i32, winner_id: i32) {
    get_game_repo()
        .await
        .update_game_winner(game_id, winner_id)
        .await;
}
