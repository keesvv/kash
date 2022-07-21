use super::{Input, InputError};
use kash::statement::Statement;
use ktf::Deserializer;
use std::io::{BufRead, BufReader};

pub struct KtfInput;

impl KtfInput {
    pub fn new() -> Self {
        Self
    }
}

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
