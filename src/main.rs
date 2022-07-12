mod cli;

use kash::{
    format::{self, Result},
    statement::{FixedStatement, Statement},
};
use std::io;

fn main() -> Result<()> {
    let statements = io::stdin()
        .lines()
        .map(|ln| {
            format::Deserializer::from_str(ln.unwrap().as_str())
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

    print!("{}", cli::format_fixed(fixed));

    Ok(())
}
