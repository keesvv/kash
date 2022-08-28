#[cfg(feature = "account")]
pub mod account;
#[cfg(feature = "budget")]
pub mod budget;
#[cfg(feature = "fixed")]
pub mod fixed;
#[cfg(feature = "income")]
pub mod income;
#[cfg(feature = "rule")]
pub mod rule;
#[cfg(feature = "savings")]
pub mod savings;
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
    #[cfg(feature = "budget")]
    Budget(budget::Budget),
    #[cfg(feature = "rule")]
    Rule(rule::Rule),
    #[cfg(feature = "savings")]
    Savings(savings::Savings),
    #[cfg(feature = "savings")]
    Goal(savings::Goal),
}

impl Statement {
    pub fn with_context<C: Context>(&self, context: &C) -> Self {
        context.apply(self.to_owned())
    }
}

pub trait Context {
    fn apply(&self, statement: Statement) -> Statement;
}
