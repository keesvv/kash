use super::{Input, InputError};
use kash::statements::Statement;
use serde::Deserialize;
use std::io::{BufRead, BufReader, Read};

pub struct KtfInput;

impl KtfInput {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Deserialize)]
pub struct KtfInputData {
    pub test: f32,
}

impl Input for KtfInput {
    fn from_read<R>(&self, reader: R) -> Result<Vec<Statement>, InputError>
    where
        R: Read,
    {
        let statements = Vec::new();
        let buf = BufReader::new(reader);

        for ln in buf.lines() {
            if let Err(_) = ln {
                return Err(InputError::Read);
            }

            let input_data = ktf::serde::from_str::<KtfInputData>(&ln.unwrap()).unwrap();
            println!("{:#?}", input_data);

            // match statement {
            //     Ok(s) => statements.push(s),
            //     Err(e) => return Err(InputError::Invalid(e.to_string())),
            // }
        }

        Ok(statements)
    }
}
