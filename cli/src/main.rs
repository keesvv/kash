use clap::Parser;
use kash_cli::args::{Args, InputFormat, OutputFormat};
use kash_cli::output::TableOutput;
use kash_convert::input::{ktf::KtfInput, Input};
use kash_convert::output::{json::JsonOutput, Output};
use std::io;

fn main() {
    let args: Args = Args::parse();

    let input = match args.input {
        InputFormat::Ktf => KtfInput::new().from_read(io::stdin()),
    }
    .unwrap();

    println!(
        "{}",
        match args.output {
            OutputFormat::Table => TableOutput::new(&input).to_string(),
            OutputFormat::Json => JsonOutput::new(&input).to_string(),
        }
    );
}
