use chrono::{DateTime, FixedOffset};
use serde::{de, Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct Date(pub DateTime<FixedOffset>);

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw_date = String::deserialize(deserializer)?;
        let date = DateTime::parse_from_rfc3339(&raw_date).map_err(de::Error::custom)?;
        Ok(Self(date))
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
