use colored::*;
use kash::statements::{fixed::FixedExpense, Statement};
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
                acc.insert(
                    entry.0.clone(),
                    entry.1 + acc.get(&entry.0).map(ToOwned::to_owned).unwrap_or_default(),
                );
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

    pub fn get_empty_data(&self) -> Vec<Data> {
        vec![Data {
            label: "none".into(),
            ..Default::default()
        }]
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
            .draw_into(
                writer,
                &if !data.is_empty() {
                    data
                } else {
                    self.get_empty_data()
                },
            )
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
