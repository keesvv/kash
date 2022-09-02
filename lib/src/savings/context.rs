use crate::statements::{
    savings::{Goal, Savings},
    Context, Statement,
};

#[derive(Clone)]
pub struct SavingsContext {
    savings: Vec<Savings>,
}

impl SavingsContext {
    pub fn from_statements(statements: &[Statement]) -> Self {
        Self {
            savings: statements
                .iter()
                .filter_map(|s| match s {
                    Statement::Savings(savings) => Some(savings.to_owned()),
                    _ => None,
                })
                .collect::<Vec<Savings>>(),
        }
    }
}

impl Context for SavingsContext {
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
