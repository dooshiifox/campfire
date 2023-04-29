use crate::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Snowflake,
    pub username: String,
    pub discrim: i16,
    /// The user's profile image id.
    /// To get a users profile image, query `cdn:/users/:user_id/pfp/:profile_img_id.png`
    pub profile_img_id: Option<Snowflake>,
    pub accent_color: Option<String>,
    pub pronouns: Option<String>,
    pub bio: Option<String>,
}

/// An interface for interacting with the `users` table of the database.
pub struct UserTable<'a> {
    pub(crate) conn: &'a sqlx::Pool<sqlx::Postgres>,
}

impl<'a> UserTable<'a> {
    pub async fn get(&self, id: Snowflake) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query!(
            "SELECT id, username, discrim, profile_img_id, accent_color, pronouns, bio FROM users WHERE id = $1",
            id.into_number()
        )
        .fetch_optional(self.conn)
        .await?;

        match user {
            Some(user) => Ok(Some(User {
                id: user.id.into(),
                username: user.username,
                discrim: user.discrim,
                profile_img_id: user.profile_img_id.map(Into::<Snowflake>::into),
                accent_color: user.accent_color,
                pronouns: user.pronouns,
                bio: user.bio,
            })),
            None => Ok(None),
        }
    }

    /// Registers a user's account.
    pub async fn register<'pw, P: Into<password::Password<'pw>>>(
        &self,
        id: Snowflake,
        username: &str,
        password: P,
        email: &str,
    ) -> Result<User, NewUserError> {
        // Check the email doesnt already exist
        let email_exists =
            sqlx::query_scalar!("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)", email)
                .fetch_one(self.conn)
                .await?;
        match email_exists {
            Some(true) | None => return Err(NewUserError::EmailTaken),
            _ => {}
        }

        // Get every user with the same username and their discriminator
        // so we can generate a new one not already used
        let existing_discrims =
            sqlx::query!("SELECT discrim FROM users WHERE username = $1", username)
                .fetch_all(self.conn)
                .await?;
        // Create a hashset with all possible discrims (0-9999) then
        // for each discrim in the database, remove it from the hashset
        let mut discrims: HashSet<i16> = (0..10000).collect();
        for discrim in existing_discrims {
            discrims.remove(&discrim.discrim);
        }
        if discrims.is_empty() {
            return Err(NewUserError::AllDiscriminatorsUsed);
        }
        // Pick a random one from whats available
        let discrim = discrims
            .iter()
            .choose(&mut rand::thread_rng())
            .unwrap_or(&0);

        let phc = password.into().generate();

        let success = sqlx::query!(
            "INSERT INTO users (id, username, discrim, phc, email) VALUES ($1, $2, $3, $4, $5)",
            id.into_number(),
            username,
            discrim,
            phc,
            email
        )
        .execute(self.conn)
        .await?;

        if success.rows_affected() == 1 {
            self.get(id).await?.ok_or(NewUserError::UserNotFound)
        } else {
            Err(NewUserError::NotInserted)
        }
    }

    /// Logs a user in with their email and password and returns their ID.
    pub async fn login<'pw, P: Into<password::Password<'pw>>>(
        &self,
        email: &str,
        password: P,
    ) -> Result<User, LoginError> {
        let user = {
            struct LoginDetails {
                id: i64,
                phc: String,
            }

            sqlx::query_as!(
                LoginDetails,
                "SELECT id, phc FROM users WHERE email = $1",
                email
            )
            .fetch_optional(self.conn)
            .await?
            .ok_or(LoginError::InvalidCredentials)?
        };

        // TODO: Update password if changed (due to encryption method changing)

        if password.into().verify(&user.phc) {
            self.get(user.id.into())
                .await?
                .ok_or(LoginError::UserNotFound)
        } else {
            Err(LoginError::InvalidCredentials)
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum NewUserError {
    #[error("The email address is already taken")]
    EmailTaken,
    #[error("All the username + discriminator combos are already taken")]
    AllDiscriminatorsUsed,
    #[error("The entry was not inserted into the database")]
    NotInserted,
    #[error("The user was not found after creating them in the database")]
    UserNotFound,
    #[error("An error occurred while querying the database")]
    DatabaseError(#[from] sqlx::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum LoginError {
    #[error("The credentials were not correct")]
    InvalidCredentials,
    #[error("The user was not found even after asserting they exist")]
    UserNotFound,
    #[error("An error occurred while querying the database")]
    DatabaseError(#[from] sqlx::Error),
}
