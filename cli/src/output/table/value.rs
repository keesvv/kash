use super::OutputOptions;
use crate::output;
use colored::*;
use kash::statements::account::AccountId;
use std::cmp::Ordering;
use std::fmt::{self, Display};
use tabular::{Row, Table};

#[derive(Clone)]
pub struct ValueTable {
    table: Table,
    opts: OutputOptions,
}

impl ValueTable {
    pub fn new(heading: &str, cols: &[Col], opts: OutputOptions) -> Self {
        let table = Table::new(
            &cols
                .iter()
                .map(|col| match col.1 {
                    Cell::Value(_) | Cell::MaskedValue(_) => "{:>}",
                    _ => "{:<}",
                })
                .collect::<Vec<&str>>()
                .join(" "),
        )
        .with_heading(heading.bold().to_string())
        .with_row(Row::from_cells(
            cols.iter()
                .map(|col| match col.1 {
                    Cell::Value(_) => format!("{} ({})", col.0, opts.currency_symbol),
                    _ => col.0.to_owned(),
                })
                .collect::<Vec<String>>(),
        ));

        Self { table, opts }
    }

    pub fn mask_cell(&self, cell: &Cell) -> Cell {
        match cell {
            Cell::Value(v) => Cell::MaskedValue(*v),
            Cell::Quota(a, b) => Cell::MaskedQuota(*a, *b),
            Cell::AccountId(id) => Cell::MaskedAccountId(id.clone()),
            other => other.clone(),
        }
    }

    pub fn add_row(&mut self, row: &[Cell]) {
        let mut table_row = Row::new();

        for cell in row {
            let cell = if !self.opts.discrete {
                cell.to_owned()
            } else {
                self.mask_cell(cell)
            };

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
    MaskedText(String),
    Value(f32),
    MaskedValue(f32),
    Quota(f32, f32),
    MaskedQuota(f32, f32),
    AccountId(AccountId),
    MaskedAccountId(AccountId),
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
            Cell::MaskedText(t) => output::generate_mask(t.len()),
            Cell::Value(v) => {
                format!("{}{:.2}", Self::get_mutation_style(*v).0, v.abs())
            }
            Cell::MaskedValue(v) => {
                format!(
                    "{}{}",
                    Self::get_mutation_style(*v).0,
                    output::generate_mask(3)
                )
            }
            Cell::Quota(spent, quota) => {
                format!(
                    "{:.2}/{:.2} ({:.1}%)",
                    spent,
                    quota,
                    (spent / quota) * 100.0
                )
            }
            Cell::MaskedQuota(spent, quota) => {
                format!(
                    "{}/{} ({:.1}%)",
                    output::generate_mask(3),
                    output::generate_mask(3),
                    (spent / quota) * 100.0
                )
            }
            Cell::AccountId(id) => id.to_string(),
            Cell::MaskedAccountId(id) => {
                let id_parts = match id {
                    AccountId::Iban(iban) => ("iban", iban),
                    AccountId::Cash(id) => ("cash", id),
                };
                format!("{}:{}", id_parts.0, output::generate_mask(id_parts.1.len()))
            }
        }
    }

    pub fn content_colored(&self) -> String {
        match self {
            Cell::Value(v) | Cell::MaskedValue(v) => self
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
