mod id;
pub use id::*;

use serde::{Deserialize, Serialize};

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
