#[cfg(feature = "input-ktf")]
pub mod ktf;

#[cfg(feature = "input-json")]
pub mod json;

use kash::statement::Statement;
use std::io::Read;

#[derive(Debug)]
pub enum InputError {
    Read,
    Invalid(String),
}

pub trait Input {
    fn from_read<R>(&self, reader: R) -> Result<Vec<Statement>, InputError>
    where
        R: Read;
}
