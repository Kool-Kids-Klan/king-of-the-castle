#[macro_use]
extern crate diesel;

pub mod models;
pub mod repo;
pub mod schema;

use repo::game_repo::PostgresGameRepo;
use repo::participation_repo::PostgresParticipationRepo;
use repo::user_repo::PostgresUserRepo;

use diesel::prelude::*;
use std::sync::Arc;

use anyhow::Result;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};

static mut VAL: Option<Arc<PgPool>> = None;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

async fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

/// # Safety
/// Is unsafe so only one DB connection is made
pub async unsafe fn establish_connection() {
    match VAL {
        Some(_) => (),
        None => {
            let database_url =
                option_env!("DATABASE_URL").unwrap_or_else(|| panic!("DATABASE_URL not defined!"));

            PgConnection::establish(database_url)
                .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

            VAL = Some(Arc::new(
                init_pool(database_url)
                    .await
                    .expect("Failed to create pool"),
            ));
        }
    }
}

pub async fn get_user_repo() -> Arc<PostgresUserRepo> {
    unsafe {
        establish_connection().await;
        let con_pool = match &VAL {
            Some(pool) => pool.clone(),
            None => panic!("Not connected!"),
        };
        Arc::new(PostgresUserRepo::new(con_pool))
    }
}

pub async fn get_game_repo() -> Arc<PostgresGameRepo> {
    unsafe {
        establish_connection().await;
        let con_pool = match &VAL {
            Some(pool) => pool.clone(),
            None => panic!("Not connected!"),
        };
        Arc::new(PostgresGameRepo::new(con_pool))
    }
}

pub async fn get_participation_repo() -> Arc<PostgresParticipationRepo> {
    unsafe {
        establish_connection().await;
        let con_pool = match &VAL {
            Some(pool) => pool.clone(),
            None => panic!("Not connected!"),
        };
        Arc::new(PostgresParticipationRepo::new(con_pool))
    }
}
