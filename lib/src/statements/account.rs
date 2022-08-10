use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub account_type: AccountType,
    pub id: AccountId,
    pub name: String,
    pub bank: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum AccountType {
    Payment,
    Savings,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AccountId(String);

impl ToString for AccountId {
    fn to_string(&self) -> String {
        self.0.to_owned()
    }
}
