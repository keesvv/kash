use colored::*;
use std::cmp::Ordering;
use std::fmt::{self, Display};
use tabular::{Row, Table};

#[derive(Clone)]
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

    pub fn with_total(&self, offset: usize, values: &[&[f32]]) -> Self {
        let mut cells: Vec<Cell> = vec![Cell::Text("total".into())];
        let mut table = self.clone();

        for _ in 1..offset {
            cells.push(Cell::Text("".into()));
        }

        for v in values {
            cells.push(Cell::Value(v.iter().sum()));
        }

        table.add_row(&cells);

        table
    }
}

impl Display for ValueTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.table.to_string().trim_end())
    }
}

#[derive(Debug, Clone)]
pub struct Col(pub String, pub Cell);

#[derive(Debug, Clone)]
pub enum Cell {
    Text(String),
    Value(f32),
}

impl Cell {
    fn get_mutation_style(value: f32) -> (char, Color) {
        match value.partial_cmp(&0.0).unwrap() {
            Ordering::Greater => ('+', Color::BrightGreen),
            Ordering::Less => ('-', Color::BrightRed),
            Ordering::Equal => ('=', Color::BrightBlack),
        }
    }

    pub fn content(&self) -> String {
        match self {
            Cell::Text(t) => t.to_owned(),
            Cell::Value(v) => {
                format_args!("{}{:.2}", Self::get_mutation_style(*v).0, v.abs()).to_string()
            }
        }
    }

    pub fn content_colored(&self) -> String {
        match self {
            Cell::Value(v) => self
                .content()
                .color(Self::get_mutation_style(*v).1)
                .to_string(),
            _ => self.content(),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.content_colored())
    }
}
