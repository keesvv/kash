use super::Savings;
use crate::{
    contexts::Context,
    statements::{self, Statement},
};
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
            now,
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
                let savings_progress: f32 = self
                    .savings
                    .iter()
                    .filter(|savings: &&Savings| savings.goal == Some(goal.id.to_owned()))
                    .map(|savings: &Savings| savings.get_total_amount(self.now))
                    .sum();

                Statement::Goal(statements::goal::Goal {
                    progress: goal.progress + savings_progress,
                    ..goal
                })
            }
            other => other,
        }
    }
}
