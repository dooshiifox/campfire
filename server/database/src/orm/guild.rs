use crate::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum CreateError {
    #[error("The entry was not inserted into the database")]
    NotInserted,
    #[error("An error occurred while querying the database")]
    DatabaseError(#[from] sqlx::Error),
}

/// An interface for interacting with the `guilds` table of the database.
pub struct Guild<'a> {
    pub(crate) conn: &'a sqlx::Pool<sqlx::Postgres>,
}

impl<'a> Guild<'a> {
    /// Create a new guild in the database.
    pub async fn create(
        &self,
        id: Snowflake,
        owner: Snowflake,
        name: &str,
    ) -> Result<(), CreateError> {
        let success = sqlx::query!(
            "INSERT INTO guilds (id, owner_id, name) VALUES ($1, $2, $3)",
            id.into_number(),
            owner.into_number(),
            name,
        )
        .execute(self.conn)
        .await?;

        if success.rows_affected() == 1 {
            Ok(())
        } else {
            Err(CreateError::NotInserted)
        }
    }

    pub async fn join(
        &self,
        id: Snowflake,
        guild: Snowflake,
        user: Snowflake,
    ) -> Result<(), CreateError> {
        let success = sqlx::query!(
            "INSERT INTO guild_members (id, guild_id, user_id) VALUES ($1, $2, $3)",
            id.into_number(),
            guild.into_number(),
            user.into_number(),
        )
        .execute(self.conn)
        .await?;

        if success.rows_affected() != 1 {
            return Err(CreateError::NotInserted);
        }

        // By default place at the bottom, so set `next` of the only guild to
        // have `next` as `None` to be this one
        let success = sqlx::query!(
            "UPDATE guild_members SET next = $1 WHERE next IS NULL AND user_id = $2",
            id.into_number(),
            user.into_number(),
        )
        .execute(self.conn)
        .await?;

        if success.rows_affected() == 1 {
            Ok(())
        } else {
            Err(CreateError::NotInserted)
        }
    }
}
