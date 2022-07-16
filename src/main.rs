mod cli;

use kash::{
    format::{self, Result},
    statement::{FixedStatement, IncomeStatement, Statement},
};
use std::io;

fn main() -> Result<()> {
    let statements = io::stdin()
        .lines()
        .map(|ln| {
            format::Deserializer::from_str(&ln.unwrap())
                .deserialize()
                .unwrap()
        })
        .collect::<Vec<Statement>>();

    // TODO: refactor
    let fixed = statements
        .iter()
        .flat_map(|statement| match statement {
            Statement::Fixed(s) => Some(s.to_owned()),
            _ => None,
        })
        .collect::<Vec<FixedStatement>>();

    let income = statements
        .iter()
        .flat_map(|statement| match statement {
            Statement::Income(s) => Some(s.to_owned()),
            _ => None,
        })
        .collect::<Vec<IncomeStatement>>();

    print!(
        "{}\n{}",
        cli::format_fixed(&fixed),
        cli::format_income(&income)
    );

    Ok(())
}
