use super::date::Date;
use super::value::MonthValues;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Statement {
    #[cfg(feature = "fixed")]
    Fixed(FixedStatement),
    #[cfg(feature = "income")]
    Income(IncomeStatement),
    #[cfg(feature = "transaction")]
    Transaction(Transaction),
}

#[cfg(feature = "fixed")]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FixedStatement {
    pub tag: String,
    pub description: String,
    pub expenses: MonthValues,
}

#[cfg(feature = "income")]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct IncomeStatement {
    pub description: String,
    pub income: MonthValues,
}

#[cfg(feature = "transaction")]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Transaction {
    pub tag: String,
    pub description: String,
    pub date: Date,
}
