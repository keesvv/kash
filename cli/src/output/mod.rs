#[cfg(feature = "output-pie")]
pub mod pie;
pub mod table;

#[cfg(feature = "output-pie")]
use self::pie::PieOutput;
use self::table::TableOutput;
use clap::clap_derive::ArgEnum;
use kash::statements::Statement;
use kash_convert::output::{json::JsonOutput, Output};

#[derive(Debug, Clone, ArgEnum)]
pub enum OutputFormat {
    Table,
    #[cfg(feature = "output-pie")]
    Pie,
    Json,
}

#[derive(Clone, Copy)]
pub struct OutputOptions {
    pub discrete: bool,
}

impl Default for OutputOptions {
    fn default() -> Self {
        Self { discrete: false }
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
