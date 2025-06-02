//! Serde support for chrono::Duration as iso8601 strings

use serde::{Deserialize as _, Deserializer, Serialize as _, Serializer, de::Error as _};

use chrono::Duration as ChronoDuration;
use iso8601::Duration as IsoDuration;

pub fn serialize<S>(duration: &ChronoDuration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    duration.to_string().serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<ChronoDuration, D::Error>
where
    D: Deserializer<'de>,
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
            ChronoDuration::days(day.into())
                + ChronoDuration::hours(hour.into())
                + ChronoDuration::minutes(minute.into())
                + ChronoDuration::seconds(second.into())
                + ChronoDuration::milliseconds(millisecond.into())
        }
        IsoDuration::Weeks(w) => ChronoDuration::weeks(w.into()),
    })
}
