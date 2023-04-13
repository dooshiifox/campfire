use crate::prelude::*;

#[derive(Deserialize, Debug)]
pub struct LoginParams {
    email: String,
    password: String,
}

/// The email or password is invalid
const INVALID_CREDENTIALS: &'static str = "InvalidCredentials";
/// Internal server error
const INTERNAL_SERVER_ERROR: &'static str = "ISE";

#[post("/login")]
pub async fn login(req: Json<LoginParams>, db: Data<DbPool>) -> impl Responder {
    let params = req.into_inner();

    match db.user().login(&params.email, &params.password).await {
        Ok(token) => ok!(token),
        Err(user::LoginError::InvalidCredentials) => {
            err!(StatusCode::UNAUTHORIZED => INVALID_CREDENTIALS)
        }
        Err(user::LoginError::DatabaseError(e)) => {
            error!("Database error: {}", e);
            err!(StatusCode::INTERNAL_SERVER_ERROR => INTERNAL_SERVER_ERROR)
        }
    }
}
