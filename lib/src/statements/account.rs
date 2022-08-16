use serde::{de, Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub account_type: AccountType,
    pub id: AccountId,
    pub name: String,
    pub bank: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum AccountType {
    Payment,
    Savings,
}

#[derive(Clone, Debug)]
pub enum AccountId {
    Iban(String),
    Cash(String),
}

impl<'de> Deserialize<'de> for AccountId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let account_str = String::deserialize(deserializer)?;
        let mut s = account_str.as_str().split(':');

        let (acc_type, acc_id): (&str, &str) = (
            s.next().ok_or(de::Error::custom("missing account type"))?,
            s.next().ok_or(de::Error::custom("missing account ID"))?,
        );

        match acc_type {
            "iban" => Ok(Self::Iban(acc_id.to_owned())),
            "cash" => Ok(Self::Cash(acc_id.to_owned())),
            unknown => Err(de::Error::custom(format!(
                "unknown account type '{unknown}'",
            ))),
        }
    }
}

impl Serialize for AccountId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl ToString for AccountId {
    fn to_string(&self) -> String {
        match self {
            Self::Iban(iban) => format!("iban:{}", iban),
            Self::Cash(id) => format!("cash:{}", id),
        }
    }
}
