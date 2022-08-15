use directories::ProjectDirs;
use serde::Deserialize;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Deserialize, Debug, Default)]
pub struct Config {
    pub repo: RepoConfig,
}

#[derive(Deserialize, Debug, Default)]
pub struct RepoConfig {
    pub path: Option<PathBuf>,
}

impl Config {
    pub fn parse() -> io::Result<Self> {
        let path = ProjectDirs::from("", "", "kash")
            .unwrap()
            .config_dir()
            .join("config.toml");

        let mut buf = Vec::new();
        let mut file = File::open(path)?;
        file.read_to_end(&mut buf)?;

        Ok(toml::from_slice(&buf)?)
    }
}
