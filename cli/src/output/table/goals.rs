use super::{
    value::{Cell, Col, ValueTable},
    ToTable,
};
use crate::output::OutputOptions;
use kash::statements::goal::Goal;

pub struct GoalsTable {
    pub goals: Vec<Goal>,
}

impl ToTable<ValueTable> for GoalsTable {
    fn to_table(&self, opts: OutputOptions) -> ValueTable {
        let mut table = ValueTable::new(
            "Goals",
            &[
                Col::Text("id".into()),
                Col::Text("description".into()),
                Col::Value("progress".into()),
                Col::Value("goal".into()),
            ],
            opts,
        );

        for goal in &self.goals {
            table.add_row(&[
                Cell::Text(goal.id.to_owned()),
                Cell::Text(goal.description.to_owned()),
                Cell::Value(goal.progress),
                Cell::Value(goal.get_goal()),
            ]);
        }

        table
    }
}
