#[cfg(feature = "rule")]
pub mod rule;

use super::account::AccountId;
use crate::date::Date;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub tag: Option<String>,
    pub description: String,
    pub date: Date,
    pub amount: f32,
    pub account_id: AccountId,
}
