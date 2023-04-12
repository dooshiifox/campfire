pub mod password;
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
    pub async fn new_user(
        &self,
        id: Snowflake,
        username: &str,
        discrim: i16,
        phc: &str,
        email: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO users (id, username, discrim, phc, email) VALUES ($1, $2, $3, $4, $5)",
            id.into_number(),
            username,
            discrim,
            phc,
            email
        )
        .execute(&self.0)
        .await?;
        Ok(())
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
