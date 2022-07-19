pub mod ktf;

use self::ktf::Statement;
use std::io::Read;

#[derive(Debug)]
pub enum InputError {
    Invalid(String),
}

pub trait Input {
    fn from_read<R>(&self, reader: R) -> Result<Vec<Statement>, InputError>
    where
        R: Read;
}
