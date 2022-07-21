use clap::Parser;
use kash_cli::args::{Args, InputFormat, OutputFormat};
use kash_cli::output::TableOutput;
use kash_convert::input::{ktf::KtfInput, Input};
use kash_convert::output::{json::JsonOutput, Output};
use std::fs::File;
use std::io::{self, Read};

fn main() {
    let args: Args = Args::parse();
    let mut input = Vec::new();

    for (i, input_file) in args.input.iter().enumerate() {
        let format = args
            .input_format
            .iter()
            .nth(i)
            .unwrap_or(args.input_format.iter().last().unwrap());

        let reader: Box<dyn Read> = match input_file.to_string().as_str() {
            "-" => Box::new(io::stdin()),
            _ => Box::new(File::open(input_file).unwrap()),
        };

        input.extend(
            match format {
                InputFormat::Ktf => KtfInput::new().from_read(reader),
            }
            .unwrap(),
        );
    }

    println!(
        "{}",
        match args.output {
            OutputFormat::Table => TableOutput::new(&input).to_string(),
            OutputFormat::Json => JsonOutput::new(&input).to_string(),
        }
    );
}
