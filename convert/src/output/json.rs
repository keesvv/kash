use super::Output;
use kash::statement::Statement;
use std::io;

pub struct JsonOutput {
    statements: Vec<Statement>,
}

impl JsonOutput {
    pub fn new(statements: &[Statement]) -> Self {
        Self {
            statements: statements.to_owned(),
        }
    }
}

impl Output for JsonOutput {
    fn to_write<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        match serde_json::to_writer(writer, &self.statements) {
            Ok(_) => Ok(()),
            Err(e) => match e.is_io() {
                true => Err(e.into()),
                false => panic!("{}", e),
            },
        }
    }
}
