#[cfg(feature = "account")]
pub mod account;
#[cfg(feature = "fixed")]
pub mod fixed;
#[cfg(feature = "income")]
pub mod income;
#[cfg(feature = "transaction")]
pub mod transaction;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Statement {
    #[cfg(feature = "fixed")]
    Fixed(fixed::FixedExpense),
    #[cfg(feature = "income")]
    Income(income::Income),
    #[cfg(feature = "transaction")]
    Transaction(transaction::Transaction),
    #[cfg(feature = "account")]
    Account(account::Account),
}
