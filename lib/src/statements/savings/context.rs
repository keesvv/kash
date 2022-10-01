use super::Savings;
use crate::{contexts::Context, statements::{Statement, self}};
use chrono::NaiveDate;

#[derive(Clone)]
pub struct SavingsContext {
    savings: Vec<Savings>,
    now: NaiveDate,
}

impl SavingsContext {
    pub fn new(now: NaiveDate) -> Self {
        Self {
            savings: Vec::new(),
            now
        }
    }
}

impl Context for SavingsContext {
    fn add(&mut self, statements: &[Statement]) {
        self.savings.extend(
            statements
                .iter()
                .filter_map(|s| match s {
                    Statement::Savings(savings) => Some(savings.to_owned()),
                    _ => None,
                })
                .collect::<Vec<Savings>>(),
        )
    }

    fn apply(&self, statement: Statement) -> Statement {
        match statement {
            #[cfg(feature = "goal")]
            Statement::Goal(goal) => {
                let progress = self
                    .savings
                    .iter()
                    .filter(|savings: &&Savings| savings.goal == goal.id)
                    .map(|savings: &Savings| savings.get_total_amount(self.now))
                    .sum();

                Statement::Goal(statements::goal::Goal { progress, ..goal })
            }
            other => other,
        }
    }
}
