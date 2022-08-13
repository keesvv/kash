use clap::{Parser, Subcommand};
use kash_cli::output::OutputFormat;
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

    /// Output format
    #[clap(short = 'o', long = "output", arg_enum, default_value = "table")]
    pub output_format: OutputFormat,

    /// Hide sensitive information
    #[clap(short = 'd', long = "discrete")]
    pub discrete: bool,
}

#[derive(Subcommand, Debug)]
pub enum Operation {
    /// Output all inputs from the repository
    Show,
}
