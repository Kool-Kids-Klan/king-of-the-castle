use std::sync::Arc;

use super::{
    super::{
        models::{Game, NewGame},
        schema::games::dsl as table,
        PgPool,
    },
    user_repo::{PostgresUserRepo, UserRepo},
};

use anyhow::Result;
use async_trait::async_trait;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[async_trait]
pub trait GameRepo {
    async fn create_game(&self, started_at: NaiveDateTime, ended_at: NaiveDateTime, winner_id: i32) -> Result<i32>;
    async fn get_game(&self, game_id: i32) -> Result<Game>;
    async fn delete_game(&self, game_id: i32) -> Result<()>;
    async fn update_game_winner(&self, game_id: i32, new_winner_id: i32) -> Result<Game>;
}

pub struct PostgresGameRepo {
    pg_pool: Arc<PgPool>,
}

impl PostgresGameRepo {
    pub fn new(pg_pool: Arc<PgPool>) -> Self {
        Self { pg_pool }
    }
}

#[async_trait]
impl GameRepo for PostgresGameRepo {
    async fn create_game(&self,
                         started_at: NaiveDateTime,
                         ended_at: NaiveDateTime,
                         winner_id: i32) -> Result<i32> {
        let new_game = NewGame {
            started_at,
            ended_at,
            winner_id
        };

        let rec: Game = diesel::insert_into(table::games)
            .values(&new_game)
            .get_result(&self.pg_pool.get()?)
            .expect("Error saving new game");

        Ok(rec.id)
    }

    async fn get_game(&self, game_id: i32) -> Result<Game> {
        Ok(table::games
            .filter(table::id.eq(game_id))
            .first(&self.pg_pool.get()?)
            .expect("Error loading game"))
    }

    async fn delete_game(&self, game_id: i32) -> Result<()> {
        diesel::delete(table::games.filter(table::id.eq(game_id)))
            .execute(&self.pg_pool.get()?)
            .expect("Error deleting game");

        Ok(())
    }

    async fn update_game_winner(&self, game_id: i32, new_winner_id: i32) -> Result<Game> {
        let game: Game = diesel::update(table::games.filter(table::id.eq(game_id)))
            .set(table::winner_id.eq(new_winner_id))
            .get_result(&self.pg_pool.get()?)
            .expect("Error updating game");

        let user_repo = Arc::new(PostgresUserRepo::new(self.pg_pool.clone()));
        user_repo.add_won_game(new_winner_id).await?;

        Ok(game)
    }
}
