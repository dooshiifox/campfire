use std::hash::Hash;

use crate::prelude::*;

pub const DB_EPOCH: i64 = 1672531200000;

/// A snowflake ID generator.
///
/// Each snowflake is a 63-bit integer, split into
/// - 42 bits for the (utc) timestamp with the epoch at 2023-01-01T00:00:00Z.
///     This will last us ~139 years, at which point its no longer our problem
/// - 10 bits for the worker ID
/// - 11 bits for the increment.
pub struct SnowflakeGenerator {
    last_timestamp: u64,
    machine_id: u16,
    increment: u16,
    /// The amount of milliseconds overflowed the increment is.
    ///
    /// To explain, if the increment overflows, we add 1 to the timestamp for
    /// the next snowflake and reset the increment to 0. However, the next
    /// millisecond also needs to start at the incremented value for the
    /// previous millisecond. This is where this value comes in: It tracks
    /// how many times this has happened.
    increment_overflow: u16,
}
impl SnowflakeGenerator {
    pub fn new(machine_id: u16) -> Self {
        if machine_id > 0b1111111111 {
            panic!("machine_id must be less than 1024");
        }

        Self {
            last_timestamp: 0,
            machine_id,
            increment: 0,
            increment_overflow: 0,
        }
    }

    /// Generates a new snowflake ID.
    pub fn generate(&mut self) -> Snowflake {
        let mut timestamp = (chrono::Utc::now().timestamp_millis() - DB_EPOCH) as u64;

        // Milliseconds have increased
        // Either reset the increment or decrement the overflow.
        if timestamp != self.last_timestamp {
            self.last_timestamp = timestamp;
            self.increment_overflow = self.increment_overflow.saturating_sub(1);
            if self.increment_overflow == 0 {
                self.increment = 0;
            }
        }

        if self.increment_overflow != 0 {
            // We've overflowed the increment, so we need to increment the
            // timestamp by 1.
            timestamp += self.increment_overflow as u64;
        }

        self.increment += 1;
        if self.increment == 0b11111111111 {
            warn!(
                "Snowflake generator increment overflowed: {} {} {}",
                timestamp, self.machine_id, self.increment_overflow
            );
            self.increment_overflow += 1;
            self.increment = 0;
        }

        Snowflake {
            timestamp,
            machine_id: self.machine_id,
            increment: self.increment,
        }
    }
}

/// A snowflake ID.
///
/// This is a 63-bit integer, split into
/// - 42 bits for the (utc) timestamp with the epoch at 2023-01-01T00:00:00Z
/// - 10 bits for the worker ID
/// - 11 bits for the increment.
#[derive(Debug, Clone, Copy, Eq, Ord)]
pub struct Snowflake {
    timestamp: u64,
    machine_id: u16,
    increment: u16,
}
impl Snowflake {
    pub fn from_number(number: i64) -> Self {
        Self {
            timestamp: (number >> 21) as u64,
            machine_id: ((number >> 11) & 0b1111111111) as u16,
            increment: (number & 0b11111111111) as u16,
        }
    }

    pub fn into_number(&self) -> i64 {
        ((self.timestamp as i64) << 21) | ((self.machine_id as i64) << 11) | (self.increment as i64)
    }

    pub fn timestamp(&self) -> std::time::SystemTime {
        std::time::UNIX_EPOCH + std::time::Duration::from_millis(self.timestamp + DB_EPOCH as u64)
    }
}
impl std::fmt::Display for Snowflake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.into_number())
    }
}
impl PartialEq for Snowflake {
    fn eq(&self, other: &Self) -> bool {
        self.into_number() == other.into_number()
    }
}
impl PartialOrd for Snowflake {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.timestamp.partial_cmp(&other.timestamp)
    }
}
impl Hash for Snowflake {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.into_number().hash(state);
    }
}
impl Serialize for Snowflake {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i64(self.into_number())
    }
}
