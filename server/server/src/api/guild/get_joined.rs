use crate::prelude::*;

pub async fn get_joined(session: Session, db: Data<DbPool>) -> impl Responder {
    // Get all the guilds the user is in.
    debug!("Getting all guilds the user is in.");
    let guilds = match db.guild().get_joined(session.user_id).await {
        Ok(guilds) => guilds,
        Err(guild::GetJoinedError::DatabaseError(e)) => {
            error!("Database error: {}", e);
            return err!(INTERNAL_SERVER_ERROR => ISE);
        }
    };

    ok!(guilds)
}
