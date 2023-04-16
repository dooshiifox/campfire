use crate::prelude::*;

#[derive(Deserialize, Debug)]
pub struct CreateParams {
    /// Name of the guild, between 2 and 60 characters
    name: String,
}

#[derive(Serialize, Debug)]
pub struct CreateResponse {
    guild_id: Snowflake,
    channel_id: Snowflake,
}

/// The name was shorter than 2 character
pub const NAME_TOO_SHORT: &'static str = "NameTooShort";
/// The name was longer than 60 characters
pub const NAME_TOO_LONG: &'static str = "NameTooLong";

pub async fn create(
    req: Json<CreateParams>,
    session: Session,
    guild_sfgen: Data<Mutex<GuildSnowflakeGen>>,
    guild_member_sfgen: Data<Mutex<GuildMemberSnowflakeGen>>,
    channel_sfgen: Data<Mutex<ChannelSnowflakeGen>>,
    db: Data<DbPool>,
) -> impl Responder {
    if req.name.len() < 2 {
        return err!(NAME_TOO_SHORT 2);
    }
    if req.name.len() > 60 {
        return err!(NAME_TOO_LONG 60);
    }

    let guild_id = { guild_sfgen.lock().await.generate() };
    match db
        .guild()
        .create(guild_id, session.user_id, &req.name)
        .await
    {
        Ok(()) => {}
        Err(guild::CreateError::NotInserted) => {
            warn!("Guild not inserted into database");
            return err!(INTERNAL_SERVER_ERROR => ISE);
        }
        Err(guild::CreateError::DatabaseError(e)) => {
            error!("Database error: {}", e);
            return err!(INTERNAL_SERVER_ERROR => ISE);
        }
    };

    // Make the owner join the guild.
    let guild_member_id = { guild_member_sfgen.lock().await.generate() };
    match db
        .guild()
        .join(guild_member_id, guild_id, session.user_id)
        .await
    {
        Ok(()) => {}
        Err(guild::JoinError::NotInserted) => {
            warn!("Guild member not inserted into database");
            return err!(INTERNAL_SERVER_ERROR => ISE);
        }
        Err(guild::JoinError::DatabaseError(e)) => {
            error!("Database error adding member: {}", e);
            return err!(INTERNAL_SERVER_ERROR => ISE);
        }
    };

    // Add a new channel to the guild
    let channel_id = { channel_sfgen.lock().await.generate() };
    match db
        .channel()
        .create(channel_id, guild_id, "general", None)
        .await
    {
        Ok(()) => {}
        Err(channel::CreateError::NextChannelDoesNotExist) => {
            warn!("Next channel does not exist when creating guild");
            return err!(INTERNAL_SERVER_ERROR => ISE);
        }
        Err(channel::CreateError::NotInserted) => {
            warn!("Channel not inserted into database");
            return err!(INTERNAL_SERVER_ERROR => ISE);
        }
        Err(channel::CreateError::DatabaseError(e)) => {
            error!("Database error adding channel: {}", e);
            return err!(INTERNAL_SERVER_ERROR => ISE);
        }
    };

    ok!(CreateResponse {
        guild_id,
        channel_id
    })
}
