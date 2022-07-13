use colored::*;
use kash::statement::{FixedStatement, IncomeStatement};
use tabular::{Row, Table};

pub fn format_heading(heading: &str) -> String {
    String::from(heading).bold().to_string()
}

pub fn format_totals_row(table: &Table, values: Vec<f32>) -> Row {
    let mut cells: Vec<String> = vec!["total".to_string()];

    for _ in 2..table.column_count() {
        cells.push("".to_string());
    }

    cells.push(values.iter().sum::<f32>().to_string());

    Row::from_cells(cells)
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

    table.add_row(format_totals_row(
        &table,
        statements.iter().map(|s| s.costs.month_avg()).collect(),
    ));

    table
}

pub fn format_income(statements: Vec<IncomeStatement>) -> Table {
    let mut table = Table::new("{:<} {:>} {:>}")
        .with_heading(format_heading("Income"))
        .with_row(Row::from_cells(["description", "avg/mo", "year"]));

    for statement in &statements {
        table.add_row(Row::from_cells([
            statement.description.clone(),
            statement.costs.month_avg().to_string(),
            statement.costs.year().to_string(),
        ]));
    }

    table.add_row(format_totals_row(
        &table,
        statements.iter().map(|s| s.costs.year()).collect(),
    ));

    table
}
