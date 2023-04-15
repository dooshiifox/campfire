use crate::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum CreateError {
    #[error("The `next` channel does not exist or does not belong to this guild")]
    NextChannelDoesNotExist,
    #[error("The entry was not inserted into the database")]
    NotInserted,
    #[error("An error occurred while querying the database")]
    DatabaseError(#[from] sqlx::Error),
}

/// An interface for interacting with the `channels` table of the database.
pub struct Channel<'a> {
    pub(crate) conn: &'a sqlx::Pool<sqlx::Postgres>,
}

impl<'a> Channel<'a> {
    /// Create a new guild in the database.
    pub async fn create(
        &self,
        id: Snowflake,
        guild: Snowflake,
        name: &str,
        next: Option<Snowflake>,
    ) -> Result<(), CreateError> {
        // If `next` exists, query to find out whether it exists in the database
        // and that channel belongs to the same guild this one does
        if let Some(next) = next {
            let exists = sqlx::query!(
                "SELECT EXISTS(SELECT 1 FROM channels WHERE id = $1 AND guild_id = $2)",
                next.into_number(),
                guild.into_number()
            )
            .fetch_one(self.conn)
            .await?;

            match exists.exists {
                Some(false) | None => return Err(CreateError::NextChannelDoesNotExist),
                _ => (),
            }
        }

        let success = sqlx::query!(
            "INSERT INTO channels (id, guild_id, name, next) VALUES ($1, $2, $3, $4)",
            id.into_number(),
            guild.into_number(),
            name,
            next.map(|n| n.into_number())
        )
        .execute(self.conn)
        .await?;

        if success.rows_affected() != 1 {
            return Err(CreateError::NotInserted);
        } else if let Some(next) = next {
            // At this point, we now have two channels saying `next` is `id`.
            // Update the one that isn't this to point to this one, if it exists
            // (it may not if `next` was the first channel in the order)
            sqlx::query!(
                "UPDATE channels SET next = $1 WHERE next = $2 AND id != $3",
                id.into_number(),
                next.into_number(),
                id.into_number()
            )
            .execute(self.conn)
            .await?;
        }

        Ok(())
    }
}
