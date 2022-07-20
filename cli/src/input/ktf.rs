use super::{Input, InputError};
use kash::statement::{FixedStatement, IncomeStatement, Statement};
use kash::value::MonthValues;
use std::fmt::{self, Display};
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub enum ParseError {
    NoSuchType,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::NoSuchType => "no such type",
            }
        )
    }
}

pub struct Deserializer<'a> {
    input: &'a str,
}

impl<'a> Deserializer<'a> {
    pub fn from_str(input: &'a str) -> Self {
        Deserializer { input }
    }

    // TODO: refactor this ugly bodged mess, use Serde for this ASAP
    pub fn deserialize(&self) -> Result<Statement, ParseError> {
        let mut cols = self.input.split('|').map(|s| s.trim());

        match cols.next().unwrap().chars().nth(0).unwrap_or('#') {
            'f' => Ok(Statement::Fixed(FixedStatement {
                tag: cols.next().unwrap_or_default().to_string(),
                description: cols.next().unwrap_or_default().to_string(),
                expenses: self.deserialize_mv(cols.next().unwrap_or_default()),
            })),
            'i' => Ok(Statement::Income(IncomeStatement {
                description: cols.next().unwrap_or_default().to_string(),
                income: self.deserialize_mv(cols.next().unwrap_or_default()),
            })),
            '#' => Ok(Statement::None),
            _ => Err(ParseError::NoSuchType),
        }
    }

    // TODO: return Result
    fn deserialize_mv(&self, col: &str) -> MonthValues {
        MonthValues::from_iter(col.split_whitespace().map(|c| c.parse().unwrap()))
    }
}

pub struct KtfInput;

impl Input for KtfInput {
    fn from_read<R>(&self, reader: R) -> Result<Vec<Statement>, InputError>
    where
        R: std::io::Read,
    {
        let mut statements = Vec::new();
        let buf = BufReader::new(reader);

        for ln in buf.lines() {
            if let Err(_) = ln {
                return Err(InputError::Read);
            }

            let statement = Deserializer::from_str(&ln.unwrap()).deserialize();
            match statement {
                Ok(s) => statements.push(s),
                Err(e) => return Err(InputError::Invalid(e.to_string())),
            }
        }

        Ok(statements)
    }
}
