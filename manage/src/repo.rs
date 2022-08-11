use kash::statements::Statement;
use kash_convert::input::InputError;
use std::fmt::Debug;
use std::io;
use std::result;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Input(InputError),
    Message(String),
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
