#[cfg(feature = "rule")]
pub mod rule;

use super::account::AccountId;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub tag: Option<String>,
    pub description: String,
    pub date: NaiveDateTime,
    pub amount: f32,
    pub account_id: AccountId,
}
