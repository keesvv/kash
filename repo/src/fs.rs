use super::repo::{Error, RepoLike, Result};
use kash::{
    contexts, rules::context::RuleContext, savings::context::SavingsContext,
    statements::Statement,
};
use kash_convert::input::{
    camt053::Camt053Input, json::JsonInput, toml::TomlInput, Input,
};
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
            Some("xml") => Ok(Camt053Input::new().from_read(input_file)),
            Some(ext) => Err(Error::Message(format!("unknown format '{ext}'"))),
            None => Err(Error::Message("extension parse error".into())),
        }?
        .map_err(Error::Input)
    }

    fn get_input_path(&self, path: &Path) -> Result<String> {
        Ok(self
            .path
            .join(path)
            .to_str()
            .ok_or(Error::Message("path is not valid unicode".into()))?
            .into())
    }

    fn get_inputs(&self) -> Result<Vec<PathBuf>> {
        let include_str =
            fs::read_to_string(&self.path.join(".kash").join("include"))
                .map_err(Error::IO)?;

        Ok(include_str
            .lines()
            .map(PathBuf::from)
            .flat_map(|ln| {
                // TODO: replace unwrap with safe error handling
                glob::glob(&self.get_input_path(&ln).unwrap())
                    .unwrap()
                    .map(|buf| buf.unwrap())
            })
            .collect())
    }

    pub fn reload_store(&mut self) -> Result<()> {
        let mut statements = Vec::new();

        for input in self.get_inputs()? {
            statements.extend(self.read_input(&self.path.join(input))?);
        }

        Ok(self.statements = contexts::apply_stacked(
            &statements,
            &mut [
                Box::new(RuleContext::new()),
                Box::new(SavingsContext::new()),
            ],
        ))
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
