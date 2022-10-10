use clap::Parser;
use std::path::PathBuf;

/// HTTP reference implementation for kash
#[derive(Parser, Debug)]
#[clap(name = "kash-http", version, about)]
pub struct Args {
    /// Repository to interact with
    #[clap(short = 'r', long = "repo")]
    pub repo: Option<PathBuf>,

    /// Port to listen on
    #[clap(short = 'p', long = "port", default_value = "8080")]
    pub port: u16,
}
