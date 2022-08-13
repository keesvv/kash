use clap::{Parser, Subcommand};
use kash_cli::output::OutputArgs;
use std::path::PathBuf;

/// command-line management interface to kash
#[derive(Parser, Debug)]
#[clap(name = "kash-manage-cli", version, about)]
pub struct Args {
    /// Repository directory
    #[clap(default_value = ".")]
    pub repo_dir: PathBuf,

    #[clap(subcommand)]
    pub op: Operation,
}

#[derive(Subcommand, Debug)]
pub enum Operation {
    /// Output all inputs from the repository
    Show(ShowArgs),
}

#[derive(Parser, Debug)]
pub struct ShowArgs {
    #[clap(flatten)]
    pub output: OutputArgs,
}
