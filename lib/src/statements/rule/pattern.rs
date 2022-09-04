use regex::Regex;
use serde::{de, Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct Pattern(pub Regex);

impl<'de> Deserialize<'de> for Pattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let regex_str = String::deserialize(deserializer)?;
        Ok(Self(Regex::new(&regex_str).map_err(de::Error::custom)?))
    }
}

impl Serialize for Pattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.0.as_str())
    }
}
