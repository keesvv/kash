use kash::statements::Statement;
use std::result;

pub enum Error {}

pub type Result<T> = result::Result<T, Error>;

pub trait RepoLike {
    fn get_all() -> Result<Vec<Statement>>;
    fn insert(statements: &[Statement]) -> Result<()>;
}
