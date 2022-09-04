use kash::statements::Statement;
use kash_convert::input::InputError;
use std::error;
use std::fmt::{Debug, Display};
use std::io;
use std::result;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Input(InputError),
    Message(String),
}

impl error::Error for Error {}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(io) => write!(f, "io error: {io}"),
            Error::Input(input) => write!(f, "input error: {input}"),
            Error::Message(msg) => write!(f, "{msg}"),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;

pub trait RepoLike {
    fn get_all(&self) -> Result<Vec<Statement>>;
    fn insert(&mut self, statements: &Statement) -> Result<()>;
    fn insert_all(&mut self, statements: &[Statement]) -> Result<()> {
        for statement in statements {
            self.insert(statement)?;
        }
        Ok(())
    }
}
