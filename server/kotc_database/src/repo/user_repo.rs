use std::sync::Arc;

use super::super::{
    models::{NewUser, User},
    schema::users::dsl as table,
    PgPool,
};

use anyhow::Result;
use async_trait::async_trait;
use diesel::prelude::*;

#[async_trait]
pub trait UserRepo {
    async fn create_user(&self, username: &str, email: &str, passhash: &str) -> Result<i32>;
    async fn list_users(&self) -> Result<Vec<User>>;
    async fn get_user(&self, user_id: i32) -> Result<User>;
    async fn delete_user(&self, user_id: i32) -> Result<()>;
    async fn update_user(
        &self,
        user_id: i32,
        new_username: Option<&str>,
        new_email: Option<&str>,
        new_passhash: Option<&str>,
    ) -> Result<User>;
    async fn add_played_game(&self, user_id: i32) -> Result<User>;
    async fn add_won_game(&self, user_id: i32) -> Result<User>;
}

pub struct PostgresUserRepo {
    pg_pool: Arc<PgPool>,
}

impl PostgresUserRepo {
    pub fn new(pg_pool: Arc<PgPool>) -> Self {
        Self { pg_pool }
    }
}

#[async_trait]
impl UserRepo for PostgresUserRepo {
    async fn create_user(
        &self,
        new_username: &str,
        new_email: &str,
        new_passhash: &str,
    ) -> Result<i32> {
        let new_user = NewUser {
            username: new_username,
            email: new_email,
            passhash: new_passhash,
        };

        let rec: User = diesel::insert_into(table::users)
            .values(&new_user)
            .get_result(&self.pg_pool.get()?)
            .expect("Error saving new user");

        Ok(rec.id)
    }

    async fn list_users(&self) -> Result<Vec<User>> {
        Ok(table::users.load::<User>(&self.pg_pool.get()?)?)
    }

    async fn get_user(&self, user_id: i32) -> Result<User> {
        Ok(table::users
            .filter(table::id.eq(user_id))
            .first(&self.pg_pool.get()?)?)
    }

    async fn delete_user(&self, user_id: i32) -> Result<()> {
        diesel::delete(table::users.filter(table::id.eq(user_id))).execute(&self.pg_pool.get()?)?;

        Ok(())
    }

    async fn update_user(
        &self,
        user_id: i32,
        new_username: Option<&str>,
        new_email: Option<&str>,
        new_passhash: Option<&str>,
    ) -> Result<User> {
        let user = self.get_user(user_id).await?;
        let new_username = match new_username {
            Some(u) => u,
            None => &user.username,
        };
        let new_email = match new_email {
            Some(e) => e,
            None => &user.email,
        };
        let new_passhash = match new_passhash {
            Some(p) => p,
            None => &user.passhash,
        };

        let user = diesel::update(table::users.filter(table::id.eq(user_id)))
            .set((
                table::username.eq(new_username),
                table::email.eq(new_email),
                table::passhash.eq(new_passhash),
            ))
            .get_result(&self.pg_pool.get()?)?;

        Ok(user)
    }

    async fn add_played_game(&self, user_id: i32) -> Result<User> {
        let user: User = diesel::update(table::users.filter(table::id.eq(user_id)))
            .set(table::games_played.eq(table::games_played + 1))
            .get_result(&self.pg_pool.get()?)?;

        Ok(user)
    }

    async fn add_won_game(&self, user_id: i32) -> Result<User> {
        let user: User = diesel::update(table::users.filter(table::id.eq(user_id)))
            .set(table::games_won.eq(table::games_won + 1))
            .get_result(&self.pg_pool.get()?)?;

        Ok(user)
    }
}
