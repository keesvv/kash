use super::repo::{Error, RepoLike, Result};
use chrono::Utc;
use kash::{
    contexts,
    statements::{
        rule::context::RuleContext, savings::context::SavingsContext, Statement,
    },
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
    opts: FsOptions,
    statements: Vec<Statement>,
}

#[derive(Clone)]
pub struct FsOptions {
    pub path: PathBuf,
    pub include: Vec<String>,
}

impl FsRepo {
    pub fn new(opts: FsOptions) -> Self {
        Self {
            opts,
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
            .opts.path
            .join(path)
            .to_str()
            .ok_or(Error::Message("path is not valid unicode".into()))?
            .into())
    }

    fn get_inputs(&self) -> Result<Vec<PathBuf>> {
        let include_str = fs::read_to_string(&self.opts.path.join(".kash").join("include"))
            .map_err(Error::IO)?;

        Ok(include_str
            .lines()
            .chain(self.opts.include.iter().map(String::as_str))
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
            statements.extend(self.read_input(&self.opts.path.join(input))?);
        }

        Ok(self.statements = contexts::apply_stacked(
            &statements,
            &mut [
                Box::new(RuleContext::new()),
                Box::new(SavingsContext::new(Utc::now().date_naive())),
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
