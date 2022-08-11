use super::repo::{Error, RepoLike, Result};
use kash::statements::Statement;
use kash_convert::input::{json::JsonInput, Input};
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

pub struct FsRepo {
    path: PathBuf,
    statements: Vec<Statement>,
}

impl FsRepo {
    pub fn new(path: &Path) -> Self {
        Self {
            path: path.to_owned(),
            statements: Vec::new(),
        }
    }

    fn read_input(&self, path: &Path) -> Result<Vec<Statement>> {
        let input_file = File::open(path).map_err(Error::IO)?;
        JsonInput::new().from_read(input_file).map_err(Error::Input)
    }

    pub fn reload_store(&mut self) -> Result<()> {
        let mut statements = Vec::new();
        let st_dir = fs::read_dir(&self.path.join("statements")).map_err(Error::IO)?;

        for input in st_dir {
            let input_path = input.map_err(Error::IO)?.path();
            statements.extend(self.read_input(&input_path)?);
        }

        self.statements = statements;
        Ok(())
    }
}

impl RepoLike for FsRepo {
    fn get_all(&self) -> Result<Vec<Statement>> {
        Ok(self.statements.to_owned())
    }

    fn insert(&mut self, _statements: &Statement) -> Result<()> {
        todo!()
    }
}
