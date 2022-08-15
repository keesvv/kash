use super::repo::{Error, RepoLike, Result};
use kash::statements::Statement;
use kash_convert::input::{camt053::Camt053Input, json::JsonInput, toml::TomlInput, Input};
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

#[derive(Clone)]
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
        match path.extension().unwrap_or_default().to_str() {
            Some("json") => Ok(JsonInput::new().from_read(input_file)),
            Some("toml") => Ok(TomlInput::new().from_read(input_file)),
            Some("camt") => Ok(Camt053Input::new().from_read(input_file)),
            Some(ext) => Err(Error::Message(format!("unknown format '{ext}'"))),
            None => Err(Error::Message("extension parse error".into())),
        }?
        .map_err(Error::Input)
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
