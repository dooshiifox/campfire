pub mod orm;
pub mod password;
pub mod prelude;
pub mod snowflake;
pub mod validation;

use crate::prelude::*;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct DbPool(pub sqlx::Pool<sqlx::Postgres>);
impl std::ops::Deref for DbPool {
    type Target = sqlx::Pool<sqlx::Postgres>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DbPool {
    pub fn user(&self) -> user::User {
        user::User { conn: &self.0 }
    }
}

pub async fn new_pool() -> Pool<Postgres> {
    dotenvy::from_path(std::path::Path::new(".env")).unwrap();

    PgPoolOptions::new()
        .max_connections(12)
        .connect(&dotenvy::var("DATABASE_URL").unwrap())
        .await
        .expect("Unable to connect to database. Is it online?")
}
