use super::{Input, InputError};
use kash::statements::Statement;
use serde::{Deserialize, Serialize};

pub struct TomlInput;

impl TomlInput {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Deserialize, Serialize)]
pub struct TomlData {
    #[serde(rename = "statement", default)]
    statements: Vec<Statement>,
}

impl Input for TomlInput {
    fn from_read<R>(&self, mut reader: R) -> Result<Vec<Statement>, InputError>
    where
        R: std::io::Read,
    {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).map_err(|_| InputError::Read)?;

        let data: TomlData =
            toml::from_slice(&buf).map_err(|e| InputError::Invalid(e.to_string()))?;

        Ok(data.statements)
    }
}
