use chrono::Duration as ChronoDuration;
use std::{ops::Add, time::Duration as StdDuration};

pub mod duration;
pub mod opt_duration;

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
