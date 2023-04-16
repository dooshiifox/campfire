use crate::prelude::*;

#[derive(Deserialize, Debug)]
pub struct CreateParams {
    /// Name of the channel, between 2 and 60 characters
    name: String,
    /// The channel to place this channel before
    #[serde(default)]
    place_before: Option<Snowflake>,
}

#[derive(Serialize, Debug)]
pub struct CreateResponse {
    /// The ID of the created channel
    id: Snowflake,
}

/// The name was shorter than 2 character
pub const NAME_TOO_SHORT: &'static str = "NameTooShort";
/// The name was longer than 60 characters
pub const NAME_TOO_LONG: &'static str = "NameTooLong";
/// The name did not conform to the expected style
pub const NAME_INVALID: &'static str = "NameInvalid";
/// The guild was not found or the user is not in the guild
pub const GUILD_NOT_FOUND: &'static str = "GuildNotFound";
/// The user does not have permission to create a channel
pub const PERMISSION_DENIED: &'static str = "PermissionDenied";
/// The `place_before` channel does not exist
pub const PLACE_BEFORE_NOT_FOUND: &'static str = "PlaceBeforeNotFound";

pub async fn create(
    guild_id: web::Path<Snowflake>,
    req: Json<CreateParams>,
    session: Session,
    channel_sfgen: Data<Mutex<ChannelSnowflakeGen>>,
    db: Data<DbPool>,
) -> impl Responder {
    if req.name.len() < 2 {
        return err!(NAME_TOO_SHORT 2);
    }
    if req.name.len() > 32 {
        return err!(NAME_TOO_LONG 32);
    }

    let matches_regex = regex::Regex::new(r"^[a-z][a-z0-9-]+[a-z0-9]$")
        .unwrap()
        .is_match(&req.name);
    if !matches_regex {
        return err!(NAME_INVALID "^[a-z][a-z0-9-]+[a-z0-9]$");
    }

    // Check the user has permission to create a channel
    let perms = match db.guild().get_permissions(*guild_id, session.user_id).await {
        Ok(perms) => perms,
        Err(guild::GetPermissionsError::NotInGuild) => {
            return err!(GUILD_NOT_FOUND);
        }
        Err(guild::GetPermissionsError::DatabaseError(e)) => {
            error!("Database error: {}", e);
            return err!(INTERNAL_SERVER_ERROR => ISE);
        }
    };
    if !perms {
        return err!(UNAUTHORIZED => PERMISSION_DENIED);
    }

    // Add a new channel to the guild
    let channel_id = { channel_sfgen.lock().await.generate() };
    match db
        .channel()
        .create(channel_id, *guild_id, &req.name, req.place_before)
        .await
    {
        Ok(()) => {}
        Err(channel::CreateError::NextChannelDoesNotExist) => {
            return err!(PLACE_BEFORE_NOT_FOUND);
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

    ok!(CreateResponse { id: channel_id })
}
