use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Channel {
    pub id: Snowflake,
    pub name: String,
}

/// An interface for interacting with the `channels` table of the database.
pub struct ChannelTable<'a> {
    pub(crate) conn: &'a sqlx::Pool<sqlx::Postgres>,
}

impl<'a> ChannelTable<'a> {
    /// Create a new guild in the database.
    pub async fn create(
        &self,
        id: Snowflake,
        guild: Snowflake,
        name: &str,
        next: Option<Snowflake>,
    ) -> Result<(), CreateError> {
        // If `next` exists, query to find out whether it exists in the database
        // and that channel belongs to the same guild this one does.
        // Otherwise, we'll just insert this channel at the end of the list.
        let order = match next {
            Some(next) => {
                let order = sqlx::query!(
                    r#"SELECT "order" FROM channels WHERE id = $1 AND guild_id = $2"#,
                    next.into_number(),
                    guild.into_number()
                )
                .fetch_optional(self.conn)
                .await?;

                match order {
                    Some(order) => order.order,
                    None => return Err(CreateError::NextChannelDoesNotExist),
                }
            }
            None => {
                let order = sqlx::query!(
                    r#"SELECT MAX("order") FROM channels WHERE guild_id = $1"#,
                    guild.into_number()
                )
                .fetch_one(self.conn)
                .await?;

                order.max.unwrap_or(0) + 1
            }
        };

        let success = sqlx::query!(
            r#"INSERT INTO channels (id, guild_id, name, "order") VALUES ($1, $2, $3, $4)"#,
            id.into_number(),
            guild.into_number(),
            name,
            order
        )
        .execute(self.conn)
        .await?;

        if success.rows_affected() != 1 {
            return Err(CreateError::NotInserted);
        } else if let Some(_) = next {
            // Assuming we didn't insert this channel at the end of the list,
            // we need to update the `order` columns of all the channels
            // that come after this one.
            sqlx::query!(
                r#"UPDATE channels SET "order" = "order" + 1 WHERE guild_id = $1 AND "order" >= $2 AND id != $3"#,
                guild.into_number(),
                order,
                id.into_number()
            )
            .execute(self.conn)
            .await?;
        }

        Ok(())
    }

    /// Checks if a user has permission to read the contents of this channel.
    pub async fn has_read_permission(
        &self,
        channel_id: Snowflake,
        user_id: Snowflake,
    ) -> Result<(), HasReadPermissionError> {
        let channel = sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM channels WHERE id = $1 AND guild_id IN (SELECT guild_id FROM guild_members WHERE user_id = $2))",
            channel_id.into_number(),
            user_id.into_number()
        )
        .fetch_one(self.conn)
        .await?;

        match channel.exists {
            Some(false) | None => Err(HasReadPermissionError::NotFound),
            _ => Ok(()),
        }
    }

    pub async fn has_write_permission(
        &self,
        channel_id: Snowflake,
        user_id: Snowflake,
    ) -> Result<(), HasWritePermissionError> {
        let channel = sqlx::query!(
            "SELECT EXISTS(SELECT 1 FROM channels WHERE id = $1 AND guild_id IN (SELECT guild_id FROM guild_members WHERE user_id = $2))",
            channel_id.into_number(),
            user_id.into_number()
        )
        .fetch_one(self.conn)
        .await?;

        match channel.exists {
            Some(false) | None => Err(HasWritePermissionError::NotFound),
            _ => Ok(()),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CreateError {
    #[error("The `next` channel does not exist or does not belong to this guild")]
    NextChannelDoesNotExist,
    #[error("The entry was not inserted into the database")]
    NotInserted,
    #[error("An error occurred while querying the database")]
    DatabaseError(#[from] sqlx::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum HasReadPermissionError {
    #[error("The channel does not exist or the user does not have permission to view it")]
    NotFound,
    #[error("An error occurred while querying the database")]
    DatabaseError(#[from] sqlx::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum HasWritePermissionError {
    #[error("The channel does not exist or the user does not have permission to view it")]
    NotFound,
    #[error("An error occurred while querying the database")]
    DatabaseError(#[from] sqlx::Error),
}
