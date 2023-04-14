//! Functions related to session authentication.
//!
//! In order to authenticate a user, the user sends a JWT that contains the
//! access token. This access token is a randomly generated i64, stored in the
//! database. Using this module you can create and retrieve the user's ID from
//! this JWT.
//!
//! We do not encrypt the user's ID directly because that is public information,
//! leading to a higher risk of a user's ID being leaked. Additionally, this
//! method allows us to revoke a specific access token, i.e. when a user logs
//! out or changes their password.

use crate::prelude::*;
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header};

/// An interface for interacting with the `access_tokens` table of the database.
pub struct AccessToken<'a> {
    pub(crate) conn: &'a sqlx::Pool<sqlx::Postgres>,
}

/// Returns the secret used to sign JWTs. This is read from the `JWT_SECRET`
/// environment variable.
fn secret() -> Vec<u8> {
    use base64::Engine;

    // Read the secret from the env and parse from b64 to [u8]
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET not set");
    base64::engine::general_purpose::STANDARD
        .decode(secret)
        .expect("JWT_SECRET is not valid base64")
}

/// An access token from the database.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Auth {
    tkn: i64,
}

impl Auth {
    fn new(tkn: i64) -> Self {
        Self { tkn }
    }

    /// Construct a new JWT from the access token.
    fn encode(&self) -> Result<String, jsonwebtoken::errors::Error> {
        encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(&secret()),
        )
    }

    /// Decode a JWT into an access token.
    fn decode(token: &str) -> Result<Self, jsonwebtoken::errors::Error> {
        let mut validation = jsonwebtoken::Validation::default();
        validation.validate_exp = false;
        validation.set_required_spec_claims::<&str>(&[]);
        jsonwebtoken::decode::<Self>(token, &DecodingKey::from_secret(&secret()), &validation)
            .map(|data| data.claims)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CreateError {
    #[error("The JSON web token could not be encoded")]
    JwtEncoding(#[from] jsonwebtoken::errors::Error),
    #[error("The entry was not inserted into the database")]
    NotInserted,
    #[error("An error occurred while querying the database")]
    DatabaseError(#[from] sqlx::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum CheckError {
    #[error("The JSON web token is invalid (expired or never existed)")]
    InvalidToken,
    #[error("The JSON web token could not be decoded")]
    JwtDecoding(#[from] jsonwebtoken::errors::Error),
    #[error("An error occurred while querying the database")]
    DatabaseError(#[from] sqlx::Error),
}

impl<'a> AccessToken<'a> {
    /// Inserts a new access token for the given user and returns the JWT.
    pub async fn create(&self, user_id: Snowflake) -> Result<String, CreateError> {
        let token = rand::thread_rng().gen_range(0..i64::MAX);
        let created_at = time::now();

        let success = sqlx::query!(
            "INSERT INTO access_tokens (token, user_id, created_at) VALUES ($1, $2, $3)",
            token,
            user_id.into_number(),
            created_at
        )
        .execute(self.conn)
        .await?;

        if success.rows_affected() != 1 {
            return Err(CreateError::NotInserted);
        }

        let auth = Auth::new(token);
        let token = auth.encode()?;
        Ok(token)
    }

    /// Checks a JWT and returns the user ID if it is valid.
    pub async fn check(&self, token: &str) -> Result<(Snowflake, i64), CheckError> {
        let auth = Auth::decode(token)?;
        let token = auth.tkn;

        let row = {
            struct UserData {
                user_id: i64,
            }

            sqlx::query_as!(
                UserData,
                "SELECT user_id FROM access_tokens WHERE token = $1",
                token
            )
            .fetch_optional(self.conn)
            .await?
        };

        row.map(|row| (row.user_id.into(), token))
            .ok_or(CheckError::InvalidToken)
    }
}
