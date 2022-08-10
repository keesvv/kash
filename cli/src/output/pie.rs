use colored::*;
use kash::statement::{FixedExpense, Statement};
use kash_convert::output::Output;
use piechart::{Chart, Color, Data};
use std::collections::HashMap;
use std::io;
use std::iter::{self, Flatten, Repeat};

pub struct PieOutput {
    statements: Vec<Statement>,
}

impl PieOutput {
    pub fn new(statements: &[Statement]) -> Self {
        Self {
            statements: statements.to_owned(),
        }
    }

    pub fn get_sorted_data<D>(&self, data: D) -> Vec<(String, f32)>
    where
        D: Iterator<Item = (String, f32)>,
    {
        let mut entries: Vec<(String, f32)> = data
            .fold(HashMap::new(), |mut acc, entry| {
                acc.insert(entry.0, entry.1);
                acc
            })
            .into_iter()
            .collect();

        entries.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        entries
    }

    pub fn get_color_iter(&self) -> Flatten<Repeat<[Color; 5]>> {
        iter::repeat([
            Color::Blue,
            Color::Red,
            Color::Green,
            Color::Purple,
            Color::Yellow,
        ])
        .flatten()
    }

    pub fn write_chart<W>(
        &self,
        writer: &mut W,
        caption: &str,
        data: &[(String, f32)],
    ) -> io::Result<()>
    where
        W: io::Write,
    {
        let mut colors = self.get_color_iter();
        let data = data
            .iter()
            .map(|(label, value)| Data {
                label: label.to_owned(),
                value: value.to_owned(),
                color: Some(colors.next().unwrap().into()),
                ..Default::default()
            })
            .collect::<Vec<Data>>();

        write!(writer, "{}\n\n", caption.bold())?;
        Chart::new()
            .legend(true)
            .radius(8)
            .aspect_ratio(4)
            .draw_into(writer, &data)
    }
}

impl Output for PieOutput {
    fn to_write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        let mut expenses: Vec<FixedExpense> = Vec::new();

        for statement in &self.statements {
            match &statement {
                Statement::Fixed(f) => expenses.push(f.to_owned()),
                _ => (),
            }
        }

        self.write_chart(
            writer,
            "Fixed expenses",
            &self.get_sorted_data(
                expenses
                    .into_iter()
                    .map(|e| (e.tag.to_owned(), e.expenses.year())),
            ),
        )
    }
}
