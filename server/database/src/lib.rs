pub mod orm;
pub mod password;
pub mod prelude;
pub mod snowflake;
pub mod time;
pub mod validation;

use crate::prelude::*;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

/// A wrapper around the database pool that allows for shorthand methods
/// and an easier-to-read `web::Data` type.
pub struct DbPool(pub sqlx::Pool<sqlx::Postgres>);
impl std::ops::Deref for DbPool {
    type Target = sqlx::Pool<sqlx::Postgres>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DbPool {
    /// Shorthand method for creating a [`user::User`] object
    pub fn user(&self) -> user::User {
        user::User { conn: &self.0 }
    }

    /// Shorthand method for creating a [`access_token::AccessToken`] object
    pub fn access_token(&self) -> access_token::AccessToken {
        access_token::AccessToken { conn: &self.0 }
    }
}

/// Create a new database pool
pub async fn new_pool() -> Pool<Postgres> {
    dotenvy::from_path(std::path::Path::new(".env")).unwrap();

    PgPoolOptions::new()
        .max_connections(12)
        .connect(&dotenvy::var("DATABASE_URL").unwrap())
        .await
        .expect("Unable to connect to database. Is it online?")
}
