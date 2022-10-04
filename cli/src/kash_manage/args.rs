use clap::{Parser, Subcommand};
use kash_cli::output::OutputArgs;
use std::path::PathBuf;

/// command-line management interface to kash
#[derive(Parser, Debug)]
#[clap(name = "kash-manage-cli", version, about)]
pub struct Args {
    /// Repository directory
    #[clap(short = 'r', long = "repo")]
    pub repo_dir: Option<PathBuf>,

    /// Additional include globs
    #[clap(short = 'I', long = "include")]
    pub include: Vec<String>,

    #[clap(subcommand)]
    pub op: Operation,
}

#[derive(Subcommand, Debug)]
pub enum Operation {
    /// Output all inputs from the repository
    Show {
        #[clap(flatten)]
        output: OutputArgs,
    },
    New {
        /// Path for the new repository
        path: PathBuf,
    },
}
