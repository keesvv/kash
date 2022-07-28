use super::{Input, InputError};
use kash::statement::Statement;

pub struct JsonInput;

impl JsonInput {
    pub fn new() -> Self {
        Self
    }
}

impl Input for JsonInput {
    fn from_read<R>(&self, _reader: R) -> Result<Vec<Statement>, InputError>
    where
        R: std::io::Read,
    {
        todo!()
    }
}
