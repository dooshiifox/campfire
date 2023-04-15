use crate::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum CreateError {
    #[error("The entry was not inserted into the database")]
    NotInserted,
    #[error("An error occurred while querying the database")]
    DatabaseError(#[from] sqlx::Error),
}

/// An interface for interacting with the `messages` table of the database.
pub struct Message<'a> {
    pub(crate) conn: &'a sqlx::Pool<sqlx::Postgres>,
}

impl<'a> Message<'a> {
    /// Create a new message in the database.
    pub async fn create(
        &self,
        id: Snowflake,
        channel: Snowflake,
        author: Snowflake,
        message: &str,
    ) -> Result<(), CreateError> {
        let success = sqlx::query!(
            "INSERT INTO messages (id, channel_id, author_id, content, updated_at) VALUES ($1, $2, $3, $4, $5)",
            id.into_number(),
            channel.into_number(),
            author.into_number(),
            message,
            time::now()
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
