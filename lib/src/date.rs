use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Default)]
pub struct Date(pub DateTime<Utc>);

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let timestamp = i64::deserialize(deserializer)?;

        Ok(Self(DateTime::from_utc(
            NaiveDateTime::from_timestamp(timestamp, 0),
            Utc,
        )))
    }
}

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(self.0.timestamp())
    }
}
