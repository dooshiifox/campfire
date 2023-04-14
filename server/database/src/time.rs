/// The time epoch used for the database. Equivalent to 2023-01-01T00:00:00Z.
pub const DB_EPOCH: i64 = 1672531200000;

/// Returns the current (UTC) time in milliseconds since the database epoch.
pub fn now() -> i64 {
    chrono::Utc::now().timestamp_millis() - DB_EPOCH
}

/// Converts a timestamp in (UTC) milliseconds since the database epoch to a
/// [`std::time::SystemTime`].
pub fn into_systime(timestamp: u64) -> std::time::SystemTime {
    std::time::UNIX_EPOCH + std::time::Duration::from_millis(timestamp + DB_EPOCH as u64)
}
