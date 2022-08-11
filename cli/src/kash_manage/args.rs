use clap::Parser;
use kash_cli::output::OutputFormat;
use std::path::PathBuf;

/// command-line management interface to kash
#[derive(Parser, Debug)]
#[clap(name = "kash-manage-cli", version, about)]
pub struct Args {
    /// Repository directory
    #[clap(default_value = ".")]
    pub repo_dir: PathBuf,

    /// Output format
    #[clap(short = 'o', long = "output", arg_enum, default_value = "table")]
    pub output_format: OutputFormat,
}
