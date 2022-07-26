use super::{Input, InputError};
use kash::statements::Statement;
use serde::Deserialize;
use std::io::Read;

pub struct KtfInput;

impl KtfInput {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug, Deserialize)]
pub struct KtfInputData {
    pub col1: f32,
    pub col2: f32,
    pub col3: f32,
}

impl Input for KtfInput {
    fn from_read<R>(&self, mut reader: R) -> Result<Vec<Statement>, InputError>
    where
        R: Read,
    {
        let statements = Vec::new();

        // FIXME: actually implement from_read instead of this
        // memory-hogging garbage.
        let input_data = ktf::from_str::<KtfInputData>(
            {
                let mut input = String::new();
                reader
                    .read_to_string(&mut input)
                    .map_err(|_| InputError::Read)?;
                input
            }
            .as_str(),
        )
        .map_err(|e| InputError::Invalid(e.to_string()))?;
        println!("{:#?}", input_data);

        Ok(statements)
    }
}
