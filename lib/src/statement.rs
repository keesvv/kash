use super::date::Date;
use super::value::MonthValues;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Statement {
    #[cfg(feature = "fixed")]
    Fixed(FixedExpense),
    #[cfg(feature = "income")]
    Income(Income),
    #[cfg(feature = "transaction")]
    Transaction(Transaction),
    #[cfg(feature = "account")]
    Account(Account),
}

#[cfg(feature = "fixed")]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FixedExpense {
    pub tag: String,
    pub description: String,
    pub expenses: MonthValues,
}

#[cfg(feature = "income")]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Income {
    pub description: String,
    pub income: MonthValues,
}

#[cfg(feature = "transaction")]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Transaction {
    pub tag: Option<String>,
    pub description: String,
    pub date: Date,
    pub mutation: f32,
}

#[cfg(feature = "account")]
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub account_type: AccountType,
    pub id: AccountId,
    pub name: String,
    pub bank: String,
}

#[cfg(feature = "account")]
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum AccountType {
    Payment,
    Savings,
}

#[cfg(feature = "account")]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AccountId(String);
