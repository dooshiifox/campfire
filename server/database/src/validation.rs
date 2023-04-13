/// Validates a username is valid.
pub fn validate_username(username: &str) -> bool {
    if username.len() < 3 || username.len() > 32 {
        return false;
    }

    if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return false;
    }

    true
}

/// Validates an email conforms to a standard / *could* exist.
///
/// Does not check if it actually exists or send out a verification email.
pub fn validate_email(email: &str) -> bool {
    if email.len() < 3 || email.len() > 320 {
        return false;
    }

    if !email.contains('@') {
        return false;
    }

    true
}
