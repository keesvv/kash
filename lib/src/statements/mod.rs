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

pub trait Context {
    fn add(&mut self, statements: &[Statement]);
    fn apply(&self, statement: Statement) -> Statement;
    fn apply_all(&self, statements: &[Statement]) -> Vec<Statement> {
        statements
            .iter()
            .map(|statement| self.apply(statement.to_owned()))
            .collect()
    }
}

pub fn apply_contexts(store: &[Statement], contexts: &mut [Box<dyn Context>]) -> Vec<Statement> {
    contexts
        .iter_mut()
        .fold(store.to_owned(), |statements, context| {
            context.add(&statements);
            context.apply_all(&statements)
        })
}
