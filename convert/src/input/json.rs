use super::{Input, InputError};
use kash::statements::Statement;

pub struct JsonInput;

impl JsonInput {
    pub fn new() -> Self {
        Self
    }
}

impl Input for JsonInput {
    fn from_read<R>(&self, reader: R) -> Result<Vec<Statement>, InputError>
    where
        R: std::io::Read,
    {
        serde_json::from_reader(reader).map_err(|e| {
            if e.is_io() {
                InputError::Read
            } else {
                InputError::Invalid(e.to_string())
            }
        })
    }
}
