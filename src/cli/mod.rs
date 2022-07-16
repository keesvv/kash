pub mod table;

use kash::statement::{FixedStatement, IncomeStatement};
use table::ValueTable;
use table::{Cell, Col};

pub fn format_fixed(statements: &[FixedStatement]) -> ValueTable {
    let mut table = ValueTable::new(
        "Fixed costs",
        &[
            Col("tag".into(), Cell::Text(Default::default())),
            Col("description".into(), Cell::Text(Default::default())),
            Col("avg/mo".into(), Cell::Value(Default::default())),
        ],
    );

    for statement in statements {
        table.add_row(&[
            Cell::Text(statement.tag.to_owned()),
            Cell::Text(statement.description.to_owned()),
            Cell::Value(statement.costs.month_avg()),
        ]);
    }

    table
}

pub fn format_income(statements: &[IncomeStatement]) -> ValueTable {
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
            Cell::Value(statement.costs.month_avg()),
            Cell::Value(statement.costs.year()),
        ]);
    }

    table
}
