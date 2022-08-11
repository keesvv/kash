use clap::clap_derive::ArgEnum;
use clap::Parser;
use kash_cli::output::OutputFormat;

#[derive(Debug, Clone, ArgEnum)]
pub enum InputFormat {
    Ktf,
    Json,
    Camt053,
}

/// command-line interface to kash
#[derive(Parser, Debug)]
#[clap(name = "kash-cli", version, about)]
pub struct Args {
    /// Input format (may be repeated for each input)
    #[clap(short = 'i', long = "input", arg_enum, default_value = "ktf")]
    pub input_format: Vec<InputFormat>,

    /// Input file(s)
    #[clap(default_value = "-")]
    pub input: Vec<String>,

    /// Output format
    #[clap(short = 'o', long = "output", arg_enum, default_value = "table")]
    pub output_format: OutputFormat,
}
