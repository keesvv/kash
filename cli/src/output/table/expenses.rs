use super::{
    value::{Cell, Col, ValueTable},
    ToTable,
};
use crate::output::OutputOptions;
use kash::statements::fixed::FixedExpense;

pub struct ExpensesTable {
    pub expenses: Vec<FixedExpense>,
}

impl ToTable<ValueTable> for ExpensesTable {
    fn to_table(&self, opts: OutputOptions) -> ValueTable {
        let mut table = ValueTable::new(
            "Fixed expenses",
            &[
                Col::Text("tag".into()),
                Col::Text("description".into()),
                Col::Value("avg/mo".into()),
                Col::Value("year".into()),
            ],
            opts,
        );

        for expense in &self.expenses {
            table.add_row(&[
                Cell::Text(expense.tag.to_owned()),
                Cell::Text(expense.description.to_owned()),
                Cell::Value(expense.expenses.month_avg() * -1.0),
                Cell::Value(expense.expenses.year() * -1.0),
            ]);
        }

        table.with_total(
            2,
            &[
                &self
                    .expenses
                    .iter()
                    .map(|expense| expense.expenses.month_avg() * -1.0)
                    .collect::<Vec<f32>>(),
                &self
                    .expenses
                    .iter()
                    .map(|expense| expense.expenses.year() * -1.0)
                    .collect::<Vec<f32>>(),
            ],
        )
    }
}
