use crate::{
    contexts::Context,
    statements::{
        savings::{Goal, Savings},
        Statement,
    },
};

#[derive(Clone)]
pub struct SavingsContext {
    savings: Vec<Savings>,
}

impl SavingsContext {
    pub fn new() -> Self {
        Self {
            savings: Vec::new(),
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
            Statement::Goal(goal) => {
                let progress = self
                    .savings
                    .iter()
                    .filter(|savings: &&Savings| savings.goal_id == Some(goal.id.to_owned()))
                    .map(|savings: &Savings| savings.get_total_amount())
                    .sum();

                Statement::Goal(Goal { progress, ..goal })
            }
            other => other,
        }
    }
}
