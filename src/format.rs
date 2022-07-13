use crate::statement::{FixedCosts, FixedStatement, IncomeStatement, Statement};
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NoSuchType,
}

pub struct Deserializer<'a> {
    input: &'a str,
}

impl<'a> Deserializer<'a> {
    pub fn from_str(input: &'a str) -> Self {
        Deserializer { input }
    }

    // TODO: refactor this ugly bodged mess, use Serde for this ASAP
    pub fn deserialize(&self) -> Result<Statement> {
        let mut cols = self.input.split('|').map(|s| s.trim());

        match cols.next().unwrap().chars().nth(0).unwrap_or('#') {
            'f' => Ok(Statement::Fixed(FixedStatement {
                tag: cols.next().unwrap_or_default().to_string(),
                description: cols.next().unwrap_or_default().to_string(),
                costs: {
                    let mut cols = cols.map(|c| c.parse().unwrap());
                    FixedCosts::new((1..=12).map(|_| cols.next()).collect())
                },
            })),
            'i' => Ok(Statement::Income(IncomeStatement {
                description: cols.next().unwrap_or_default().to_string(),
                costs: {
                    let mut cols = cols.map(|c| c.parse().unwrap());
                    FixedCosts::new((1..=12).map(|_| cols.next()).collect())
                },
            })),
            '#' => Ok(Statement::None),
            _ => Err(Error::NoSuchType),
        }
    }
}
