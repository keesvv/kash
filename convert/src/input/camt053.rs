use super::{Input, InputError};
use kash::statement::Statement;

pub struct Camt053Input;

impl Camt053Input {
    pub fn new() -> Self {
        Self
    }
}

impl Input for Camt053Input {
    fn from_read<R>(&self, _: R) -> Result<Vec<Statement>, InputError>
    where
        R: std::io::Read,
    {
        todo!()
    }
}
