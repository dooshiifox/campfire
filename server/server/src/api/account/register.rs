use crate::prelude::*;

#[derive(Deserialize, Debug)]
pub struct RegisterParams {
    /// The chosen username of the new user
    username: String,
    /// The chosen password of the new user
    password: String,
    /// The email address of the new user
    email: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    access_token: String,
    user: user::User,
}

/// The username is invalid
const INVALID_USERNAME: &'static str = "InvalidUsername";
/// The username and all discriminators are already taken
const USERNAME_TAKEN: &'static str = "UsernameTaken";
/// The email address is invalid
const INVALID_EMAIL: &'static str = "InvalidEmail";
/// The email address is already taken
const EMAIL_TAKEN: &'static str = "EmailTaken";
/// The password is too weak
const PASSWORD_TOO_WEAK: &'static str = "PasswordTooWeak";
/// The password is too long
const PASSWORD_TOO_LONG: &'static str = "PasswordTooLong";
/// The password is too short
const PASSWORD_TOO_SHORT: &'static str = "PasswordTooShort";
/// The password is too common
const PASSWORD_TOO_COMMON: &'static str = "PasswordTooCommon";
/// The password is too similar to the username
const PASSWORD_LIKE_USERNAME: &'static str = "PasswordLikeUsername";
/// The password is too similar to the email address
const PASSWORD_LIKE_EMAIL: &'static str = "PasswordLikeEmail";

pub async fn register(
    req: Json<RegisterParams>,
    user_sfgen: Data<Mutex<UserSnowflakeGen>>,
    db: Data<DbPool>,
) -> impl Responder {
    if !validation::validate_username(&req.username) {
        return err!(INVALID_USERNAME);
    }

    if !validation::validate_email(&req.email) {
        return err!(INVALID_EMAIL);
    }

    // Validate the password
    let password = password::Password::new(&req.password);
    if let Err(e) = password.validate(&req.username, &req.email) {
        return match e {
            password::PasswordError::TooShort => err!(PASSWORD_TOO_SHORT),
            password::PasswordError::TooLong => err!(PASSWORD_TOO_LONG),
            password::PasswordError::TooCommon => err!(PASSWORD_TOO_COMMON),
            password::PasswordError::TooSimilarToUsername => err!(PASSWORD_LIKE_USERNAME),
            password::PasswordError::TooSimilarToEmail => err!(PASSWORD_LIKE_EMAIL),
            password::PasswordError::TooWeak => err!(PASSWORD_TOO_WEAK),
        };
    };

    let user_id = { user_sfgen.lock().await.generate() };
    let user = match db
        .user()
        .register(user_id, &req.username, password, &req.email)
        .await
    {
        Ok(user) => user,
        Err(user::NewUserError::AllDiscriminatorsUsed) => return err!(USERNAME_TAKEN),
        Err(user::NewUserError::EmailTaken) => return err!(EMAIL_TAKEN),
        Err(user::NewUserError::NotInserted) => {
            warn!(
                "Register error: User not inserted into database: {user_id} / {} / {}",
                req.username, req.email
            );
            return err!(INTERNAL_SERVER_ERROR => ISE);
        }
        Err(user::NewUserError::UserNotFound) => {
            error!(
                "Register error: User not found after insertion: {user_id} / {} / {}",
                req.username, req.email
            );
            return err!(INTERNAL_SERVER_ERROR => ISE);
        }
        Err(user::NewUserError::DatabaseError(e)) => {
            error!(
                "Register error: Database error: {} / {user_id} / {} / {}",
                e, req.username, req.email
            );
            return err!(INTERNAL_SERVER_ERROR => ISE);
        }
    };

    let jwt = match db.access_token().create(user.id).await {
        Ok(jwt) => jwt,
        Err(access_token::CreateError::JwtEncoding(_)) => {
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

    ok!(RegisterResponse {
        access_token: jwt,
        user,
    })
}
