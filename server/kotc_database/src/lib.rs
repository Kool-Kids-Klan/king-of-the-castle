#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod repo;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use anyhow::Result;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

async fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub async fn establish_connection() -> PgPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));

    init_pool(&database_url)
        .await
        .expect("Failed to create pool")
}
