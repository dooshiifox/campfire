use crate::v1::prelude::*;

// Note that we can't use the username because multiple users can have the
// same username but different discriminators, and remembering your discrim
// when needing to login sounds like a pain.
#[derive(Deserialize, Debug)]
pub struct LoginParams {
    email: String,
    password: String,
}

#[derive(Serialize, Debug)]
pub struct LoginResponse {
    user_id: Snowflake,
    access_token: String,
}

/// The email or password is invalid
const INVALID_CREDENTIALS: &'static str = "InvalidCredentials";

pub async fn create(req: Json<LoginParams>, db: Data<DbPool>) -> impl Responder {
    let params = req.into_inner();

    let id = match db.user().login(&params.email, &params.password).await {
        Ok(id) => id,
        Err(user::LoginError::InvalidCredentials) => {
            return err!(UNAUTHORIZED => INVALID_CREDENTIALS)
        }
        Err(user::LoginError::DatabaseError(e)) => {
            error!("Database error: {}", e);
            return err!(INTERNAL_SERVER_ERROR => ISE);
        }
    };

    let jwt = match db.access_token().create(id).await {
        Ok(jwt) => jwt,
        Err(access_token::CreateError::JwtEncoding(e)) => {
            error!("JWT encoding error: {}", e);
            return err!(INTERNAL_SERVER_ERROR => ISE);
        }
        Err(access_token::CreateError::NotInserted) => {
            error!("Access token not inserted");
            return err!(INTERNAL_SERVER_ERROR => ISE);
        }
        Err(access_token::CreateError::DatabaseError(e)) => {
            error!("Database error: {}", e);
            return err!(INTERNAL_SERVER_ERROR => ISE);
        }
    };

    ok!(LoginResponse {
        user_id: id,
        access_token: jwt,
    })
}
