use kash::statement::{FixedExpense, Statement};
use kash_convert::output::Output;
use piechart::{Chart, Color, Data};
use std::iter;

pub struct PieOutput {
    statements: Vec<Statement>,
}

impl PieOutput {
    pub fn new(statements: &[Statement]) -> Self {
        Self {
            statements: statements.to_owned(),
        }
    }
}

impl Output for PieOutput {
    fn to_write<W>(&self, writer: &mut W) -> std::io::Result<()>
    where
        W: std::io::Write,
    {
        let mut expenses: Vec<FixedExpense> = Vec::new();

        for statement in &self.statements {
            match &statement {
                Statement::Fixed(f) => expenses.push(f.to_owned()),
                _ => (),
            }
        }

        let mut colors = iter::repeat([
            Color::Blue,
            Color::Red,
            Color::Green,
            Color::Purple,
            Color::Yellow,
        ])
        .flatten();

        let data = expenses
            .iter()
            // TODO: sum up expenses per tag
            .map(|e: &FixedExpense| (e.tag.to_owned(), e.expenses.year()))
            .map(|e| Data {
                label: e.0,
                value: e.1,
                color: Some(colors.next().unwrap().into()),
                ..Default::default()
            })
            .collect::<Vec<Data>>();

        Chart::new()
            .legend(true)
            .radius(8)
            .aspect_ratio(4)
            .draw_into(writer, &data)
    }
}
