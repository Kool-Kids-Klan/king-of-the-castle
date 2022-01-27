use std::sync::Arc;

use super::{
    super::{
        models::{NewParticipation, Participation},
        schema::participations::dsl::*,
        PgPool,
    },
    user_repo::{PostgresUserRepo, UserRepo},
};

use anyhow::Result;
use async_trait::async_trait;
use diesel::prelude::*;

#[async_trait]
pub trait ParticipationRepo {
    async fn create_participation(&self, game_id: i32, user_id: i32) -> Result<i32>;
    async fn get_participation(&self, participation_id: i32) -> Result<Participation>;
    async fn delete_participation(&self, participation_id: i32) -> Result<()>;
}

pub struct PostgresParticipationRepo {
    pg_pool: Arc<PgPool>,
}

impl PostgresParticipationRepo {
    pub fn new(pg_pool: Arc<PgPool>) -> Self {
        Self { pg_pool: pg_pool }
    }
}

#[async_trait]
impl ParticipationRepo for PostgresParticipationRepo {
    async fn create_participation(&self, new_game_id: i32, new_user_id: i32) -> Result<i32> {
        let new_participation = NewParticipation {
            game_id: new_game_id,
            user_id: new_user_id,
        };

        let rec: Participation = diesel::insert_into(participations)
            .values(&new_participation)
            .get_result(&self.pg_pool.get()?)
            .expect("Error saving new participation");

        let user_repo = Arc::new(PostgresUserRepo::new(self.pg_pool.clone()));
        user_repo.add_played_game(new_user_id).await?;

        Ok(rec.id)
    }

    async fn get_participation(&self, participation_id: i32) -> Result<Participation> {
        Ok(participations
            .filter(id.eq(participation_id))
            .first(&self.pg_pool.get()?)
            .expect("Error loading participation"))
    }

    async fn delete_participation(&self, participation_id: i32) -> Result<()> {
        diesel::delete(participations.filter(id.eq(participation_id)))
            .execute(&self.pg_pool.get()?)
            .expect("Error deleting participation");

        Ok(())
    }
}
