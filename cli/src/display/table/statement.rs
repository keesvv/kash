use super::value::{Cell, Col, ValueTable};
use kash::statement::{FixedStatement, IncomeStatement, Statement};
use std::fmt::{self, Display};

pub struct StatementsTable {
    statements: Vec<Statement>,
}

impl StatementsTable {
    pub fn new(statements: &[Statement]) -> Self {
        Self {
            statements: statements.to_owned(),
        }
    }

    pub fn format_fixed(&self, statements: &[FixedStatement]) -> ValueTable {
        let mut table = ValueTable::new(
            "Fixed expenses",
            &[
                Col("tag".into(), Cell::Text(Default::default())),
                Col("description".into(), Cell::Text(Default::default())),
                Col("avg/mo".into(), Cell::Value(Default::default())),
                Col("year".into(), Cell::Value(Default::default())),
            ],
        );

        for statement in statements {
            table.add_row(&[
                Cell::Text(statement.tag.to_owned()),
                Cell::Text(statement.description.to_owned()),
                Cell::Value(statement.expenses.month_avg() * -1.0),
                Cell::Value(statement.expenses.year() * -1.0),
            ]);
        }

        table.with_total(
            2,
            &[
                &statements
                    .iter()
                    .map(|statement| statement.expenses.month_avg() * -1.0)
                    .collect::<Vec<f32>>(),
                &statements
                    .iter()
                    .map(|statement| statement.expenses.year() * -1.0)
                    .collect::<Vec<f32>>(),
            ],
        )
    }

    pub fn format_income(&self, statements: &[IncomeStatement]) -> ValueTable {
        let mut table = ValueTable::new(
            "Income",
            &[
                Col("description".into(), Cell::Text(Default::default())),
                Col("avg/mo".into(), Cell::Value(Default::default())),
                Col("year".into(), Cell::Value(Default::default())),
            ],
        );

        for statement in statements {
            table.add_row(&[
                Cell::Text(statement.description.to_owned()),
                Cell::Value(statement.income.month_avg()),
                Cell::Value(statement.income.year()),
            ]);
        }

        table.with_total(
            1,
            &[
                &statements
                    .iter()
                    .map(|statement| statement.income.month_avg())
                    .collect::<Vec<f32>>(),
                &statements
                    .iter()
                    .map(|statement| statement.income.year())
                    .collect::<Vec<f32>>(),
            ],
        )
    }
}

impl Display for StatementsTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: refactor
        let fixed = self
            .statements
            .iter()
            .filter_map(|statement| match statement {
                Statement::Fixed(s) => Some(s.to_owned()),
                _ => None,
            })
            .collect::<Vec<FixedStatement>>();

        // TODO: refactor
        let income = self
            .statements
            .iter()
            .filter_map(|statement| match statement {
                Statement::Income(s) => Some(s.to_owned()),
                _ => None,
            })
            .collect::<Vec<IncomeStatement>>();

        write!(
            f,
            "{}\n\n{}",
            self.format_fixed(&fixed),
            self.format_income(&income)
        )
    }
}
