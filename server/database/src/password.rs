#![allow(clippy::module_name_repetitions)]
use argon2::PasswordVerifier;

/// Returns the global pepper secret
fn secret() -> Vec<u8> {
    use base64::Engine;

    // Read the secret from the env and parse from b64 to [u8]
    let secret = std::env::var("PEPPER").expect("PEPPER not set");
    base64::engine::general_purpose::STANDARD
        .decode(secret)
        .expect("PEPPER is not valid base64")
}

/// A struct for password generation and validation.
///
/// A lot of the code in this struct comes from
/// [this article](https://www.lpalmieri.com/posts/password-authentication-in-rust/#3-3-6-argon2)
pub struct Password<'pw> {
    password: &'pw str,
}
impl<'pw> Password<'pw> {
    pub fn new(password: &'pw str) -> Self {
        Self { password }
    }

    /// Validates that a password has a valid format.
    ///
    /// # Errors
    ///
    /// Returns a [`PasswordError`] if the password is not valid.
    pub fn validate(&self, _username: &str, _email: &str) -> Result<(), PasswordError> {
        if self.password.len() < 8 {
            return Err(PasswordError::TooShort);
        }
        if self.password.len() > 64 {
            return Err(PasswordError::TooLong);
        }

        // TODO: More password validation!
        // Find a good library that makes sure passwords are secure.
        Ok(())
    }

    /// Generates a new hash for the given password.
    pub fn generate(&self) -> String {
        use argon2::PasswordHasher;

        let secret = secret();
        let hasher = argon2::Argon2::new_with_secret(
            &secret,
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            argon2::Params::new(15000, 2, 1, None).expect("Invalid args to argon2::Params"),
        )
        .expect("Why is your pepper so long????");

        let salt = password_hash::SaltString::generate(&mut rand::thread_rng());
        hasher
            .hash_password(self.password.as_bytes(), &salt)
            .expect("Failed to hash password")
            .to_string()
    }

    /// Verifies this password matches a PHC string.
    pub fn verify(&self, phc: &str) -> bool {
        let expected_hash = argon2::PasswordHash::new(phc).expect("Failed to create password hash");
        argon2::Argon2::new_with_secret(
            &secret(),
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            argon2::Params::default(),
        )
        .expect("dam that secret is long")
        .verify_password(self.password.as_bytes(), &expected_hash)
        .is_ok()
    }
}

impl<'pw> From<&'pw str> for Password<'pw> {
    fn from(password: &'pw str) -> Self {
        Self::new(password)
    }
}
impl<'pw> From<&'pw String> for Password<'pw> {
    fn from(password: &'pw String) -> Self {
        Self::new(password)
    }
}

/// An error that can occur when validating a password.
pub enum PasswordError {
    /// The password being validated was too short.
    TooShort,
    /// The password being validated was too long.
    TooLong,
    /// The password being validated was too common.
    TooCommon,
    /// The password being validated was too similar to the username.
    TooSimilarToUsername,
    /// The password being validated was too similar to the email.
    TooSimilarToEmail,
    /// The password being validated was too weak.
    TooWeak,
}
