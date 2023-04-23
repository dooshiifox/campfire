use crate::prelude::*;

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
    access_token: String,
    user: user::User,
}

/// The email or password is invalid
const INVALID_CREDENTIALS: &'static str = "InvalidCredentials";

pub async fn login(req: Json<LoginParams>, db: Data<DbPool>) -> impl Responder {
    let user = match db.user().login(&req.email, &req.password).await {
        Ok(user) => user,
        Err(user::LoginError::InvalidCredentials) => {
            return err!(UNAUTHORIZED => INVALID_CREDENTIALS)
        }
        Err(user::LoginError::UserNotFound) => {
            warn!(
                "User {} not found even after asserting they exist",
                req.email
            );
            return err!(INTERNAL_SERVER_ERROR => ISE);
        }
        Err(user::LoginError::DatabaseError(e)) => {
            error!("Database error: {}", e);
            return err!(INTERNAL_SERVER_ERROR => ISE);
        }
    };

    let jwt = match db.access_token().create(user.id).await {
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
        access_token: jwt,
        user
    })
}
