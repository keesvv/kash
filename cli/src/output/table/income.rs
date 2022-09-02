use super::{
    value::{Cell, Col, ValueTable},
    ToTable,
};
use crate::output::OutputOptions;
use kash::{statements::income::Income, value::MonthValues};

pub struct IncomeTable {
    pub income: Vec<Income>,
    pub gross_income: MonthValues,
    pub disc_income: MonthValues,
}

impl ToTable<ValueTable> for IncomeTable {
    fn to_table(&self, opts: OutputOptions) -> ValueTable {
        let mut table = ValueTable::new(
            "Income",
            &[
                Col::Text("description".into()),
                Col::Value("avg/mo".into()),
                Col::Value("year".into()),
            ],
            opts,
        );

        for statement in &self.income {
            table.add_row(&[
                Cell::Text(statement.description.to_owned()),
                Cell::Value(statement.income.month_avg()),
                Cell::Value(statement.income.year()),
            ]);
        }

        table.add_row(&[
            Cell::Text("total (gross)".into()),
            Cell::Value(self.gross_income.month_avg()),
            Cell::Value(self.gross_income.year()),
        ]);

        table.add_row(&[
            Cell::Text("total (discr.)".into()),
            Cell::Value(self.disc_income.month_avg()),
            Cell::Value(self.disc_income.year()),
        ]);

        table
    }
}
