use crate::repo::{RepoLike, Result};
use kash::statements::Statement;

pub struct MemRepo {
    statements: Vec<Statement>,
}

impl MemRepo {
    pub fn new() -> Self {
        Self {
            statements: Vec::new(),
        }
    }
}

impl RepoLike for MemRepo {
    fn get_all(&self) -> Result<Vec<Statement>> {
        Ok(self.statements.to_owned())
    }

    fn insert(&mut self, statements: &Statement) -> Result<()> {
        Ok(self.statements.push(statements.to_owned()))
    }
}
