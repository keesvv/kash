use clap::clap_derive::ArgEnum;
use clap::Parser;
use kash_cli::output::OutputArgs;

#[derive(Debug, Clone, ArgEnum)]
pub enum InputFormat {
    Toml,
    Json,
    Camt053,
}

/// command-line interface to kash
#[derive(Parser, Debug)]
#[clap(name = "kash-cli", version, about)]
pub struct Args {
    /// Input format (may be repeated for each input)
    #[clap(short = 'i', long = "input", arg_enum, default_value = "toml")]
    pub input_format: Vec<InputFormat>,

    /// Input file(s)
    #[clap(default_value = "-")]
    pub input: Vec<String>,

    #[clap(flatten)]
    pub output: OutputArgs,
}
