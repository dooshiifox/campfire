use crate::v1::prelude::*;

#[derive(Deserialize, Debug)]
pub struct RegisterParams {
    /// The chosen username of the new user
    username: String,
    /// The chosen password of the new user
    password: String,
    /// The email address of the new user
    email: String,
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
const WEAK_PASSWORD: &'static str = "WeakPassword";
/// The password is too long
const LONG_PASSWORD: &'static str = "LongPassword";
/// The password is too short
const SHORT_PASSWORD: &'static str = "ShortPassword";
/// The password is too common
const COMMON_PASSWORD: &'static str = "CommonPassword";
/// The password is too similar to the username
const SIMILAR_USERNAME: &'static str = "SimilarUsername";
/// The password is too similar to the email address
const SIMILAR_EMAIL: &'static str = "SimilarEmail";

#[post("/register")]
pub async fn register(
    req: Json<RegisterParams>,
    user_sfgen: Data<Mutex<UserSnowflakeGen>>,
    db: Data<DbPool>,
) -> impl Responder {
    let params = req.into_inner();

    if !validation::validate_username(&params.username) {
        return err!(INVALID_USERNAME);
    }

    if !validation::validate_email(&params.email) {
        return err!(INVALID_EMAIL);
    }

    // Validate the password
    let password = password::Password::new(&params.password);
    if let Err(e) = password.validate(&params.username, &params.email) {
        return match e {
            password::PasswordError::TooShort => err!(SHORT_PASSWORD),
            password::PasswordError::TooLong => err!(LONG_PASSWORD),
            password::PasswordError::TooCommon => err!(COMMON_PASSWORD),
            password::PasswordError::TooSimilarToUsername => err!(SIMILAR_USERNAME),
            password::PasswordError::TooSimilarToEmail => err!(SIMILAR_EMAIL),
            password::PasswordError::TooWeak => err!(WEAK_PASSWORD),
        };
    };

    let user_id = { user_sfgen.lock().await.generate() };
    match db
        .user()
        .register(user_id, &params.username, password, &params.email)
        .await
    {
        Ok(()) => ok!(format!("Created u:{user_id}")),
        Err(user::NewUserError::AllDiscriminatorsUsed) => err!(USERNAME_TAKEN),
        Err(user::NewUserError::EmailTaken) => err!(EMAIL_TAKEN),
        Err(user::NewUserError::NotInserted) => {
            warn!(
                "Register error: User not inserted into database: {user_id} / {} / {}",
                params.username, params.email
            );
            err!(INTERNAL_SERVER_ERROR => ISE)
        }
        Err(user::NewUserError::DatabaseError(e)) => {
            error!(
                "Register error: Database error: {} / {user_id} / {} / {}",
                e, params.username, params.email
            );
            err!(INTERNAL_SERVER_ERROR => ISE)
        }
    }
}
