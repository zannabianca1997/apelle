//! Serde support for chrono::Duration as iso8601 strings

use std::{ops::Add, u32};

use serde::{Deserialize as _, Deserializer, Serialize as _, Serializer, de::Error as _};

use chrono::Duration as ChronoDuration;
use iso8601::Duration as IsoDuration;
use std::time::Duration as StdDuration;

pub fn serialize<S>(duration: &ChronoDuration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    duration.to_string().serialize(serializer)
}

pub fn deserialize<'de, D, Dur>(deserializer: D) -> Result<Dur, D::Error>
where
    D: Deserializer<'de>,
    Dur: DurationExt,
{
    let d = IsoDuration::deserialize(deserializer)?;

    Ok(match d {
        IsoDuration::YMDHMS {
            year,
            month,
            day,
            hour,
            minute,
            second,
            millisecond,
        } => {
            if year != 0 || month != 0 {
                return Err(D::Error::custom(
                    "unsupported deserialization from iso8601 that contains years or months",
                ));
            }
            Dur::days(day)
                + Dur::hours(hour)
                + Dur::minutes(minute)
                + Dur::seconds(second)
                + Dur::milliseconds(millisecond)
        }
        IsoDuration::Weeks(w) => Dur::weeks(w),
    })
}

pub trait DurationExt: Sized + Add<Output = Self> {
    fn days(n: u32) -> Self;
    fn hours(n: u32) -> Self;
    fn minutes(n: u32) -> Self;
    fn seconds(n: u32) -> Self;
    fn milliseconds(n: u32) -> Self;

    fn weeks(n: u32) -> Self;
}

impl DurationExt for ChronoDuration {
    fn days(n: u32) -> Self {
        Self::days(n as _)
    }

    fn hours(n: u32) -> Self {
        Self::hours(n as _)
    }

    fn minutes(n: u32) -> Self {
        Self::minutes(n as _)
    }

    fn seconds(n: u32) -> Self {
        Self::seconds(n as _)
    }

    fn milliseconds(n: u32) -> Self {
        Self::milliseconds(n as _)
    }

    fn weeks(n: u32) -> Self {
        Self::weeks(n as _)
    }
}

impl DurationExt for StdDuration {
    fn days(n: u32) -> Self {
        Self::from_secs(n as u64 * 24 * 3600)
    }

    fn hours(n: u32) -> Self {
        Self::from_secs(n as u64 * 3600)
    }

    fn minutes(n: u32) -> Self {
        Self::from_secs(n as u64 * 60)
    }

    fn seconds(n: u32) -> Self {
        Self::from_secs(n as _)
    }

    fn milliseconds(n: u32) -> Self {
        Self::from_millis(n as _)
    }

    fn weeks(n: u32) -> Self {
        Self::days(7 * n)
    }
}
