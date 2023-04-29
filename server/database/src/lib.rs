#![warn(clippy::all, clippy::pedantic, clippy::style)]
#![allow(
    clippy::must_use_candidate,
    clippy::module_name_repetitions,
    // sqlx can panic and I don't want to add this on to every method
    // that makes a query.
    clippy::missing_panics_doc,
    clippy::missing_errors_doc
)]

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
    /// Creates a [`user::UserTable`] interface
    pub fn user(&self) -> user::UserTable {
        user::UserTable { conn: &self.0 }
    }

    /// Creates a [`access_token::AccessTokenTable`] interface
    pub fn access_token(&self) -> access_token::AccessTokenTable {
        access_token::AccessTokenTable { conn: &self.0 }
    }

    /// Creates a [`guild::GuildTable`] interface
    pub fn guild(&self) -> guild::GuildTable {
        guild::GuildTable { conn: &self.0 }
    }

    // /// Creates a [`role::RoleTable`] interface
    // pub fn role(&self) -> role::Role {
    //     role::Role { conn: &self.0 }
    // }

    /// Creates a [`channel::ChannelTable`] interface
    pub fn channel(&self) -> channel::ChannelTable {
        channel::ChannelTable { conn: &self.0 }
    }

    /// Creates a [`message::MessageTable`] interface
    pub fn message(&self) -> message::MessageTable {
        message::MessageTable { conn: &self.0 }
    }
}

/// Create a new database pool
///
/// # Panics
///
/// Panics if the database cannot be connected to or if the migrations
/// fail to run for some reason.
pub async fn new_pool() -> Pool<Postgres> {
    let env_path = std::path::Path::new(".env");
    if env_path.exists() {
        dotenvy::from_path(env_path).unwrap();
    }
    info!(
        "Connecting to database: {}",
        dotenvy::var("DATABASE_URL").unwrap()
    );

    let pool = PgPoolOptions::new()
        .max_connections(12)
        .connect(&dotenvy::var("DATABASE_URL").unwrap())
        .await
        .expect("Unable to connect to database. Is it online?");

    sqlx::migrate!("../migrations")
        .run(&pool)
        .await
        .expect("Cannot run migrations.");

    pool
}
