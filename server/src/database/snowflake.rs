use crate::prelude::*;
use std::cell::Cell;

pub const DB_EPOCH: i64 = 1672531200000;

/// A snowflake ID generator.
///
/// Each snowflake is a 63-bit integer, split into
/// - 42 bits for the (utc) timestamp with the epoch at 2023-01-01T00:00:00Z.
///     This will last us ~139 years, at which point its no longer our problem
/// - 10 bits for the worker ID
/// - 11 bits for the increment.
pub struct SnowflakeGenerator {
    last_timestamp: Cell<u64>,
    machine_id: u16,
    increment: Cell<u16>,
    /// The amount of milliseconds overflowed the increment is.
    ///
    /// To explain, if the increment overflows, we add 1 to the timestamp for
    /// the next snowflake and reset the increment to 0. However, the next
    /// millisecond also needs to start at the incremented value for the
    /// previous millisecond. This is where this value comes in: It tracks
    /// how many times this has happened.
    increment_overflow: Cell<u16>,
}
impl SnowflakeGenerator {
    pub fn new(machine_id: u16) -> Self {
        if machine_id > 0b1111111111 {
            panic!("machine_id must be less than 1024");
        }

        Self {
            last_timestamp: Cell::new(0),
            machine_id,
            increment: Cell::new(0),
            increment_overflow: Cell::new(0),
        }
    }

    /// Generates a new snowflake ID.
    pub fn generate(&self) -> Snowflake {
        let mut timestamp = (chrono::Utc::now().timestamp_millis() - DB_EPOCH) as u64;
        let mut increment = self.increment.get();
        let mut increment_overflow = self.increment_overflow.get();

        // Milliseconds have increased
        // Either reset the increment or decrement the overflow.
        if timestamp != self.last_timestamp.get() {
            self.last_timestamp.set(timestamp);
            increment_overflow = increment_overflow.saturating_sub(1);
            if increment_overflow == 0 {
                increment = 0;
            }
        }

        if increment_overflow != 0 {
            // We've overflowed the increment, so we need to increment the
            // timestamp by 1.
            timestamp += increment_overflow as u64;
        }

        increment += 1;
        if increment == 0b11111111111 {
            warn!(
                "Snowflake generator increment overflowed: {} {} {}",
                timestamp, self.machine_id, increment_overflow
            );
            increment_overflow += 1;
            increment = 0;
        }

        self.increment.set(increment);
        self.increment_overflow.set(increment_overflow);

        Snowflake {
            timestamp,
            machine_id: self.machine_id,
            increment,
        }
    }
}

/// A snowflake ID.
///
/// This is a 63-bit integer, split into
/// - 42 bits for the (utc) timestamp with the epoch at 2023-01-01T00:00:00Z
/// - 10 bits for the worker ID
/// - 11 bits for the increment.
pub struct Snowflake {
    timestamp: u64,
    machine_id: u16,
    increment: u16,
}
impl Snowflake {
    pub fn into_number(&self) -> i64 {
        ((self.timestamp as i64) << 21) | ((self.machine_id as i64) << 11) | (self.increment as i64)
    }

    pub fn timestamp(&self) -> std::time::SystemTime {
        std::time::UNIX_EPOCH + std::time::Duration::from_millis(self.timestamp + DB_EPOCH as u64)
    }
}
