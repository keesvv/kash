use serde::{de::Error, Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Budget {
    pub tag: String,
    pub quota: Quota,
}

#[derive(Serialize, Clone, Copy, Debug)]
pub enum Quota {
    Percentage(f32),
    Absolute(f32),
}

impl Quota {
    pub fn get_abs(&self, income: f32) -> f32 {
        match self {
            Quota::Absolute(a) => *a,
            Quota::Percentage(p) => (p / 100.0) * income,
        }
    }
}

impl<'de> Deserialize<'de> for Quota {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let quota_str = String::deserialize(deserializer)?;

        match quota_str.chars().last() {
            Some('%') => Ok(Self::Percentage({
                let mut str = quota_str;
                str.pop();
                str.parse().map_err(Error::custom)?
            })),
            Some(_) => Ok(Self::Absolute(quota_str.parse().map_err(Error::custom)?)),
            _ => Err(Error::custom("invalid quota")),
        }
    }
}
