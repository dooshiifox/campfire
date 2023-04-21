use std::collections::HashMap;

use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Guild {
    pub id: Snowflake,
    pub owner: user::User,
    pub name: String,
    pub channels: Vec<channel::Channel>,
}

/// An interface for interacting with the `guilds` table of the database.
pub struct GuildTable<'a> {
    pub(crate) conn: &'a sqlx::Pool<sqlx::Postgres>,
}

impl<'a> GuildTable<'a> {
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

    /// Get all the guilds a user has joined.
    pub async fn get_joined(&self, user: Snowflake) -> Result<Vec<Guild>, GetJoinedError> {
        debug!("Getting guilds");

        // Get all the guilds the user is in, ordered by `order`
        let guilds_with_channels = sqlx::query!(
            r#"
            SELECT
                g.id,
                g.name,
                g.owner_id,
                u.username as owner_username,
                u.discrim as owner_discrim,
                u.profile_img_id as owner_profile_img_id,
                u.accent_color as owner_accent_color,
                u.pronouns as owner_pronouns,
                u.bio as owner_bio,
                c.id as channel_id,
                c.name as channel_name
            FROM
                guilds g
                INNER JOIN users u ON g.owner_id = u.id
                INNER JOIN channels c ON g.id = c.guild_id
                INNER JOIN guild_members gm ON g.id = gm.guild_id
            WHERE
                g.id IN (
                    SELECT
                        guild_id
                    FROM
                        guild_members
                    WHERE
                        user_id = $1
                )
            ORDER BY
                gm.order,
                c.order
            "#,
            user.into_number()
        )
        .fetch_all(self.conn)
        .await?;

        debug!("Got guilds with channels");

        // SQL returns each channel as a separate row, so we need to group them
        let mut guild_mapping = HashMap::new();
        let mut guild_order: Vec<i64> = vec![];
        for guild in guilds_with_channels {
            let guild_id = guild.id;
            let inserted_guild = guild_mapping.entry(guild_id).or_insert_with(|| {
                // Insert from within this callback so that we only insert the
                // guild once.
                guild_order.push(guild_id);

                Guild {
                    id: guild_id.into(),
                    owner: user::User {
                        id: guild.owner_id.into(),
                        username: guild.owner_username,
                        discrim: guild.owner_discrim,
                        profile_img_id: guild.owner_profile_img_id.map(|id| id.into()),
                        accent_color: guild.owner_accent_color,
                        pronouns: guild.owner_pronouns,
                        bio: guild.owner_bio,
                    },
                    name: guild.name,
                    channels: vec![],
                }
            });

            inserted_guild.channels.push(channel::Channel {
                id: guild.channel_id.into(),
                name: guild.channel_name,
            });
        }

        debug!("Mapped guilds with channels");

        let mut guilds = vec![];
        for id in guild_order {
            guilds.push(guild_mapping.remove(&id).unwrap());
        }

        debug!("Pushed guilds with channels");

        Ok(guilds)
    }

    /// Make a user join a guild.
    pub async fn join(
        &self,
        id: Snowflake,
        guild: Snowflake,
        user: Snowflake,
    ) -> Result<(), JoinError> {
        let success = sqlx::query!(
            "INSERT INTO guild_members (id, guild_id, user_id) VALUES ($1, $2, $3)",
            id.into_number(),
            guild.into_number(),
            user.into_number(),
        )
        .execute(self.conn)
        .await?;

        if success.rows_affected() != 1 {
            return Err(JoinError::NotInserted);
        }

        // By default place at the top
        sqlx::query!(
            r#"UPDATE guild_members SET "order" = "order" + 1 WHERE id != $1 AND user_id = $2"#,
            id.into_number(),
            user.into_number(),
        )
        .execute(self.conn)
        .await?;

        Ok(())
    }

    /// Gets the permissions of a user in the guild
    pub async fn get_permissions(
        &self,
        guild: Snowflake,
        user: Snowflake,
    ) -> Result<bool, GetPermissionsError> {
        let guild = sqlx::query!(
            r#"
            SELECT id, owner_id
            FROM guilds
            WHERE guilds.id = $1
            "#,
            guild.into_number()
        )
        .fetch_optional(self.conn)
        .await?;

        let Some(guild) = guild else {
            return Err(GetPermissionsError::NotInGuild);
        };

        Ok(guild.owner_id == user.into_number())
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
pub enum JoinError {
    #[error("The entry was not inserted into the database")]
    NotInserted,
    #[error("An error occurred while querying the database")]
    DatabaseError(#[from] sqlx::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum GetJoinedError {
    #[error("An error occurred while querying the database")]
    DatabaseError(#[from] sqlx::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum GetPermissionsError {
    #[error("The user is not in the guild")]
    NotInGuild,
    #[error("An error occurred while querying the database")]
    DatabaseError(#[from] sqlx::Error),
}
