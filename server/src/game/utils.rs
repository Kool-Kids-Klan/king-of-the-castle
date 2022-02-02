use kotc_database::{get_game_repo, get_user_repo};
use chrono::{NaiveDateTime, Utc};
use anyhow::Result;

use kotc_database::models::User;
use kotc_database::repo::game_repo::GameRepo;
use kotc_database::repo::user_repo::UserRepo;

pub async fn find_user_by_id(id: i32) -> Result<User> {
    get_user_repo().await.get_user(id).await
}

pub async fn add_game_to_db(started_at: NaiveDateTime, winner_id: i32) {
    let ended_at = Utc::now().naive_utc();
    get_game_repo().await.create_game(started_at, ended_at, winner_id).await;
}