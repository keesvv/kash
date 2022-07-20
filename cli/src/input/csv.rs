use super::ktf::Statement;
use super::{Input, InputError};
use csv::ReaderBuilder;
use serde::Deserialize;
use std::io::Read;

pub struct CsvInput {
    delim: u8,
}

#[derive(Deserialize, Debug)]
pub struct CsvInputRecord {
    name: String,
}

impl CsvInput {
    pub fn new(delim: u8) -> Self {
        CsvInput { delim }
    }
}

impl Input for CsvInput {
    fn from_read<R>(&self, reader: R) -> Result<Vec<Statement>, InputError>
    where
        R: Read,
    {
        let statements = Vec::new();
        let mut csv_r = ReaderBuilder::new()
            .delimiter(self.delim)
            .from_reader(reader);

        for statement in csv_r.deserialize::<CsvInputRecord>() {
            if let Err(_) = statement {
                return Err(InputError::Read);
            }

            println!("{:#?}", statement);
        }

        Ok(statements)
    }
}
