#[cfg(feature = "output-pie")]
pub mod pie;
pub mod table;

#[cfg(feature = "output-pie")]
use self::pie::PieOutput;
use self::table::TableOutput;
use clap::{clap_derive::ArgEnum, Parser};
use kash::statements::Statement;
use kash_convert::output::{json::JsonOutput, Output};
use std::iter;

#[derive(Debug, Clone, Copy, ArgEnum)]
pub enum OutputFormat {
    Table,
    #[cfg(feature = "output-pie")]
    Pie,
    Json,
}

#[derive(Clone, Copy)]
pub struct OutputOptions {
    pub discrete: bool,
    pub currency_symbol: char,
}

impl From<OutputArgs> for OutputOptions {
    fn from(args: OutputArgs) -> Self {
        Self {
            discrete: args.discrete,
            currency_symbol: args
                .currency_symbol
                .unwrap_or(Self::default().currency_symbol),
        }
    }
}

impl Default for OutputOptions {
    fn default() -> Self {
        Self {
            discrete: false,
            currency_symbol: CUR_SYM,
        }
    }
}

impl OutputFormat {
    pub fn to_stdout(&self, input: &[Statement], opts: OutputOptions) {
        println!(
            "{}",
            match self {
                OutputFormat::Table => TableOutput::new(&input, opts).to_string(),
                #[cfg(feature = "output-pie")]
                OutputFormat::Pie => PieOutput::new(&input).to_string(),
                OutputFormat::Json => JsonOutput::new(&input).to_string(),
            }
        )
    }
}

#[derive(Parser, Debug, Clone, Copy)]
pub struct OutputArgs {
    /// Output format
    #[clap(short = 'o', long = "output", arg_enum, default_value = "table")]
    pub output_format: OutputFormat,

    /// Currency symbol
    #[clap(short = 'c', long = "currency-symbol")]
    pub currency_symbol: Option<char>,

    /// Hide sensitive information
    #[clap(short = 'd', long = "discrete")]
    pub discrete: bool,
}

pub const CUR_SYM: char = '¤';
pub const MASK_CHAR: char = 'x';

pub fn generate_mask(len: usize) -> String {
    iter::repeat(MASK_CHAR).take(len).collect()
}
