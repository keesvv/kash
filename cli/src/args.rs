use clap::clap_derive::ArgEnum;
use clap::Parser;

#[derive(Debug, Clone, ArgEnum)]
pub enum InputFormat {
    Ktf,
}

#[derive(Debug, Clone, ArgEnum)]
pub enum OutputFormat {
    Table,
    Json,
}

/// command-line interface to kash
#[derive(Parser, Debug)]
#[clap(name = "kash-cli", author, version, about)]
pub struct Args {
    /// Input format
    #[clap(short = 'f', long, arg_enum, default_value = "ktf")]
    pub input_format: Vec<InputFormat>,

    /// Input file
    #[clap(short, long, default_value = "-")]
    pub input: Vec<String>,

    /// Output format
    #[clap(short, long, arg_enum, default_value = "table")]
    pub output: OutputFormat,
}
