//! Serde support for chrono::Duration as iso8601 strings

use serde::{Deserialize as _, Deserializer, Serialize as _, Serializer, de::Error as _};

use chrono::Duration as ChronoDuration;
use iso8601::Duration as IsoDuration;

pub fn serialize<S>(duration: &Option<ChronoDuration>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    duration.map(|d| d.to_string()).serialize(serializer)
}

pub fn deserialize<'de, D, Dur>(deserializer: D) -> Result<Option<Dur>, D::Error>
where
    D: Deserializer<'de>,
    Dur: super::DurationExt,
{
    let d = Option::<IsoDuration>::deserialize(deserializer)?;

    let Some(d) = d else {
        return Ok(None);
    };

    Ok(Some(match d {
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
    }))
}
