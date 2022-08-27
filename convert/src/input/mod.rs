#[cfg(feature = "input-camt053")]
pub mod camt053;
#[cfg(feature = "input-json")]
pub mod json;
#[cfg(feature = "input-ktf")]
pub mod ktf;
#[cfg(feature = "input-toml")]
pub mod toml;

use kash::statements::Statement;
use std::error;
use std::fmt::Display;
use std::io::Read;

#[derive(Debug)]
pub enum InputError {
    Read,
    Invalid(String),
}

impl error::Error for InputError {}
impl Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read => write!(f, "input read error"),
            Self::Invalid(err) => write!(f, "invalid input: {err}"),
        }
    }
}

pub trait Input {
    fn from_read<R>(&self, reader: R) -> Result<Vec<Statement>, InputError>
    where
        R: Read;
}
