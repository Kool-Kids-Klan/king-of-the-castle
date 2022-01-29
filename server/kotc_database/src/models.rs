use super::schema::*;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub passhash: &'a str,
}

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub passhash: String,
    pub games_played: i32,
    pub games_won: i32,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "participations"]
pub struct NewParticipation {
    pub user_id: i32,
    pub game_id: i32,
}

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct Participation {
    pub id: i32,
    pub user_id: i32,
    pub game_id: i32,
}

#[derive(Insertable)]
#[table_name = "games"]
pub struct NewGame {
    pub winner_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct Game {
    pub id: i32,
    pub started_at: chrono::NaiveDateTime,
    pub winner_id: Option<i32>,
}
