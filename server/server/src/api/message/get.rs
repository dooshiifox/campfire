use crate::prelude::*;

/// The channel was not found or the user does not have permission to view it.
pub const NOT_FOUND: &str = "NotFound";

/// Returns all the messages in a channel.
pub async fn get(
    channel_id: web::Path<Snowflake>,
    session: Session,
    pool: web::Data<DbPool>,
) -> impl Responder {
    // Check the user has permission to view this channel
    match pool
        .channel()
        .has_read_permission(*channel_id, session.user_id)
        .await
    {
        Ok(()) => {}
        Err(channel::HasReadPermissionError::NotFound) => return err!(NOT_FOUND),
        Err(channel::HasReadPermissionError::DatabaseError(e)) => {
            error!("Database error: {}", e);
            return err!(INTERNAL_SERVER_ERROR => ISE);
        }
    };

    // Get the messages
    let messages = match pool.message().get(*channel_id, 50, 0).await {
        Ok(messages) => messages,
        Err(message::GetError::DatabaseError(e)) => {
            error!("Database error: {}", e);
            return err!(INTERNAL_SERVER_ERROR => ISE);
        }
    };

    ok!(messages)
}
