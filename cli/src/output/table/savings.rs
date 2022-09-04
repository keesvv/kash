use super::{
    value::{Cell, Col, ValueTable},
    ToTable,
};
use crate::output::OutputOptions;
use kash::statements::savings::Goal;

pub struct SavingsTable {
    pub goals: Vec<Goal>,
}

impl ToTable<ValueTable> for SavingsTable {
    fn to_table(&self, opts: OutputOptions) -> ValueTable {
        let mut table = ValueTable::new(
            "Savings",
            &[
                Col::Text("id".into()),
                Col::Text("description".into()),
                Col::Value("progress".into()),
                Col::Value("total".into()),
            ],
            opts,
        );

        for goal in &self.goals {
            table.add_row(&[
                Cell::Text(goal.id.to_owned()),
                Cell::Text(goal.description.to_owned()),
                Cell::Value(goal.progress),
                Cell::Value(if goal.get_total() == 0.0 {
                    goal.progress
                } else {
                    goal.get_total()
                }),
            ]);
        }

        table
    }
}
