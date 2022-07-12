use colored::*;
use kash::statement::FixedStatement;
use tabular::{Row, Table};

pub fn format_heading(heading: &str) -> String {
    String::from(heading).bold().to_string()
}

pub fn format_fixed(statements: Vec<FixedStatement>) -> Table {
    let mut table = Table::new("{:<} {:<} {:>}")
        .with_heading(format_heading("Fixed costs"))
        .with_row(Row::from_cells(["tag", "description", "avg/mo"]));

    for statement in &statements {
        table.add_row(Row::from_cells([
            statement.tag.clone(),
            statement.description.clone(),
            statement.costs.month_avg().to_string(),
        ]));
    }

    table.add_row(Row::from_cells([
        "total",
        "",
        &statements
            .iter()
            .map(|s| s.costs.month_avg())
            .sum::<f32>()
            .to_string()
            .as_str(),
    ]));

    table
}
