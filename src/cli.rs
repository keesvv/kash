use kash::statement::FixedStatement;
use tabular::{Row, Table};

pub fn format_fixed(statement: &FixedStatement) -> Table {
    Table::new("{:<} {:<} {:>}")
        .with_heading("Fixed costs")
        .with_row(Row::from_cells(["tag", "description", "avg/mo"]))
        .with_row(Row::from_cells([
            statement.tag,
            statement.description,
            statement.costs.month_avg().to_string().as_str(),
        ]))
}
