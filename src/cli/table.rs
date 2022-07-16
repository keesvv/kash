use colored::*;
use std::fmt::{self, Display};
use tabular::{Row, Table};

pub struct ValueTable {
    table: Table,
}

impl ValueTable {
    pub fn new(heading: &str, cols: &[Col]) -> Self {
        let table = Table::new(
            &cols
                .iter()
                .map(|col| match col.1 {
                    Cell::Text(_) => "{:<}",
                    Cell::Value(_) => "{:>}",
                })
                .collect::<Vec<&str>>()
                .join(" "),
        )
        .with_heading(heading.bold().to_string())
        .with_row(Row::from_cells(
            cols.iter()
                .map(|col| col.0.to_owned())
                .collect::<Vec<String>>(),
        ));

        Self { table }
    }

    pub fn add_row(&mut self, row: &[Cell]) {
        let mut table_row = Row::new();

        for cell in row {
            table_row.add_custom_width_cell(cell.to_string(), cell.content().len());
        }

        self.table.add_row(table_row);
    }
}

impl Display for ValueTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.table.fmt(f)
    }
}

#[derive(Debug)]
pub struct Col(pub String, pub Cell);

#[derive(Debug)]
pub enum Cell {
    Text(String),
    Value(f32),
}

impl Cell {
    pub fn content(&self) -> String {
        match self {
            Cell::Text(t) => t.to_owned(),
            Cell::Value(v) => fmt::format(format_args!("{:.2}", v)),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: add color to cells
        f.write_str(&self.content())
    }
}

// pub fn format_totals_row(table: &Table, values: Vec<f32>) -> Row {
//     let mut cells: Vec<String> = vec!["total".to_string()];

//     for _ in 2..table.column_count() {
//         cells.push("".to_string());
//     }

//     cells.push(values.iter().sum::<f32>().to_string());

//     Row::from_cells(cells)
// }
