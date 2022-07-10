use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {}

#[derive(Default, Debug)]
pub struct Statement<'a> {
    tag: &'a str,
}

pub struct Deserializer<'a> {
    input: &'a str,
}

impl<'a> Deserializer<'a> {
    pub fn from_str(input: &'a str) -> Self {
        Deserializer { input }
    }

    // TODO: refactor this ugly bodged mess, use Serde for this ASAP
    pub fn deserialize(&self) -> Result<Statement> {
        let mut statement: Statement = Default::default();

        for (i, col) in self.input.split('|').map(|s| s.trim()).enumerate() {
            match i {
                1 => statement.tag = col,
                _ => (),
            }
        }

        Ok(statement)
    }
}
