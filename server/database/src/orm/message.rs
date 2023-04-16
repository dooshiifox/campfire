use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Snowflake,
    pub channel_id: Snowflake,
    pub author: user::User,
    pub content: String,
    pub sent_at: u64,
    pub updated_at: u64,
}

/// An interface for interacting with the `messages` table of the database.
pub struct MessageTable<'a> {
    pub(crate) conn: &'a sqlx::Pool<sqlx::Postgres>,
}

impl<'a> MessageTable<'a> {
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

    /// Get the messages in a channel, ordered by creation time, with a
    /// limit and offset.
    pub async fn get(
        &self,
        channel: Snowflake,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<Message>, GetError> {
        let messages = sqlx::query!(
            "SELECT m.id, m.channel_id, m.author_id, m.content, m.updated_at, u.username, u.discrim, u.profile_img_id, u.accent_color, u.pronouns, u.bio FROM messages m LEFT JOIN users u ON m.author_id = u.id WHERE m.channel_id = $1 ORDER BY m.id DESC LIMIT $2 OFFSET $3",
            channel.into_number(),
            limit as i64,
            offset as i64
        )
        .fetch_all(self.conn)
        .await?;

        Ok(messages
            .into_iter()
            .map(|message| Message {
                id: Snowflake::from_number(message.id),
                channel_id: Snowflake::from_number(message.channel_id),
                author: user::User {
                    id: Snowflake::from_number(message.author_id),
                    username: message.username,
                    discrim: message.discrim,
                    profile_img_id: message.profile_img_id,
                    accent_color: message.accent_color,
                    pronouns: message.pronouns,
                    bio: message.bio,
                },
                content: message.content,
                sent_at: Snowflake::from_number(message.id).timestamp,
                updated_at: message.updated_at as u64,
            })
            .collect())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CreateError {
    #[error("The entry was not inserted into the database")]
    NotInserted,
    #[error("An error occurred while querying the database")]
    DatabaseError(#[from] sqlx::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum GetError {
    #[error("An error occurred while querying the database")]
    DatabaseError(#[from] sqlx::Error),
}
