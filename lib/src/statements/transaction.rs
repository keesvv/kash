use super::account::AccountId;
use crate::date::Date;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Transaction {
    pub tag: Option<String>,
    pub description: String,
    pub date: Date,
    pub mutation: Decimal,
    pub account_id: AccountId,
}
