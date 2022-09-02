use crate::{
    statements::savings::{Savings, SavingsModel},
    value::MonthValues,
};

impl MonthValues {
    pub fn get_discretionary(&self, expenses: Self, savings: &[Savings]) -> Self {
        *self
            - expenses
            - Self::new(
                [savings
                    .iter()
                    .filter(|s: &&Savings| {
                        if let SavingsModel::Recurring { .. } = s.model {
                            true
                        } else {
                            false
                        }
                    })
                    .map(|s: &Savings| s.amount)
                    .sum(); 12],
            )
    }
}
