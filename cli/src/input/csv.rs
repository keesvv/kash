use std::io::Read;

use super::ktf::Statement;
use super::{Input, InputError};

pub struct CsvInput;

impl CsvInput {
    pub fn new() -> Self {
        CsvInput {}
    }
}

impl Input for CsvInput {
    fn from_read<R>(&self, _reader: R) -> Result<Vec<Statement>, InputError>
    where
        R: Read,
    {
        Ok(vec![Statement::None])
    }
}
