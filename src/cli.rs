use kash::statement::FixedStatement;
use tabular::{Row, Table};

pub fn format_fixed(statements: Vec<FixedStatement>) -> Table {
    let mut table = Table::new("{:<} {:<} {:>}")
        .with_heading("Fixed costs")
        .with_row(Row::from_cells(["tag", "description", "avg/mo"]));

    for statement in statements {
        table.add_row(Row::from_cells([
            statement.tag,
            statement.description,
            statement.costs.month_avg().to_string(),
        ]));
    }

    table
}
