#![allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::cast_lossless,
    clippy::module_name_repetitions
)]
use crate::prelude::*;
use std::hash::Hash;

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
    /// Creates a new snowflake generator.
    ///
    /// # Panics
    ///
    /// Panics if `machine_id >= 1024`.
    pub fn new(machine_id: u16) -> Self {
        assert!(machine_id < 1024, "machine_id must be less than 1024");

        Self {
            last_timestamp: 0,
            machine_id,
            increment: 0,
            increment_overflow: 0,
        }
    }

    /// Generates a new snowflake ID.
    pub fn generate(&mut self) -> Snowflake {
        let mut timestamp = time::now() as u64;

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
            timestamp += u64::from(self.increment_overflow);
        }

        self.increment += 1;
        if self.increment == 0b111_1111_1111 {
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
///
/// # Implementation Notes
///
/// Serializes to a string, but deserializes from a number or a string.
#[derive(Debug, Clone, Copy, Eq)]
pub struct Snowflake {
    pub timestamp: u64,
    pub machine_id: u16,
    pub increment: u16,
}
impl Snowflake {
    /// Creates a new [`Snowflake`] from a number.
    pub fn from_number(number: u64) -> Self {
        Self {
            timestamp: number >> 21,
            machine_id: ((number >> 11) & 0b11_1111_1111) as u16,
            increment: (number & 0b111_1111_1111) as u16,
        }
    }

    /// Returns the snowflake as an i64.
    pub fn into_number(&self) -> i64 {
        // These casts are safe because the values are guaranteed to be within
        // 0..2^63 until >100 years from now.
        ((self.timestamp as i64) << 21) | ((self.machine_id as i64) << 11) | (self.increment as i64)
    }

    /// Returns the timestamp of the snowflake.
    pub fn timestamp(&self) -> std::time::SystemTime {
        time::into_systime(self.timestamp)
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
impl Ord for Snowflake {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}
impl Hash for Snowflake {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.into_number().hash(state);
    }
}
impl Serialize for Snowflake {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}
impl<'de> Deserialize<'de> for Snowflake {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Try deserialize as a number, then attempt as a string, then return
        // an error if neither work.

        struct SnowflakeVisitor;

        impl<'de> serde::de::Visitor<'de> for SnowflakeVisitor {
            type Value = Snowflake;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(
                    "a snowflake (64-bit signed number or string containing 64-bit signed number)",
                )
            }

            // fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<Self::Value, E> {
            //     Ok(Snowflake::from_number(value))
            // }

            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                value.parse().map(Snowflake::from_number).map_err(|_| {
                    serde::de::Error::custom(format!(
                        "could not parse `{value}` as 64-bit signed number"
                    ))
                })
            }
        }

        // This would be all good if it weren't for actix-web's Path deserializer
        // which doesn't support `any`.
        // deserializer.deserialize_any(SnowflakeVisitor)
        deserializer.deserialize_str(SnowflakeVisitor)
    }
}
impl From<Snowflake> for i64 {
    fn from(snowflake: Snowflake) -> Self {
        snowflake.into_number()
    }
}
impl From<i64> for Snowflake {
    fn from(number: i64) -> Self {
        Self::from_number(number as u64)
    }
}
impl From<Snowflake> for u64 {
    fn from(snowflake: Snowflake) -> Self {
        snowflake.into_number() as u64
    }
}
impl From<u64> for Snowflake {
    fn from(number: u64) -> Self {
        Self::from_number(number)
    }
}
