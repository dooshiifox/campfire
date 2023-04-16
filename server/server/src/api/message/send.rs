use crate::prelude::*;

#[derive(Debug, Deserialize)]
pub struct SendParams {
    content: String,
}

#[derive(Serialize, Debug)]
pub struct SendResponse {
    message_id: Snowflake,
}

/// The message was too short (empty)
pub const MESSAGE_TOO_SHORT: &str = "MessageTooShort";
/// The message was too long (over 10000 characters)
pub const MESSAGE_TOO_LONG: &str = "MessageTooLong";

pub async fn send(
    channel_id: web::Path<Snowflake>,
    body: Json<SendParams>,
    session: Session,
    message_sfgen: Data<Mutex<MessageSnowflakeGen>>,
    db: Data<DbPool>,
) -> impl Responder {
    if body.content.len() == 0 {
        return err!(MESSAGE_TOO_SHORT);
    }
    if body.content.len() > 10000 {
        return err!(MESSAGE_TOO_LONG 10000);
    }

    let message_id = { message_sfgen.lock().await.generate() };
    match db
        .message()
        .create(
            message_id,
            (*channel_id).into(),
            session.user_id,
            &body.content,
        )
        .await
    {
        Ok(()) => {}
        Err(message::CreateError::NotInserted) => {
            warn!("Message not inserted into database");
            return err!(INTERNAL_SERVER_ERROR => ISE);
        }
        Err(message::CreateError::DatabaseError(e)) => {
            error!("Database error: {}", e);
            return err!(INTERNAL_SERVER_ERROR => ISE);
        }
    };

    ok!(SendResponse { message_id })
}
