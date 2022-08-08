use clap::Parser;
use kash_cli::args::{Args, InputFormat, OutputFormat};
#[cfg(feature = "output-pie")]
use kash_cli::output::pie::PieOutput;
use kash_cli::output::table::TableOutput;
use kash_convert::input::{camt053::Camt053Input, json::JsonInput, ktf::KtfInput, Input};
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
            file => Box::new(File::open(file).unwrap()),
        };

        input.extend(
            match format {
                InputFormat::Ktf => KtfInput::new().from_read(reader),
                InputFormat::Json => JsonInput::new().from_read(reader),
                InputFormat::Camt053 => Camt053Input::new().from_read(reader),
            }
            .unwrap(),
        );
    }

    println!(
        "{}",
        match args.output_format {
            OutputFormat::Table => TableOutput::new(&input).to_string(),
            #[cfg(feature = "output-pie")]
            OutputFormat::Pie => PieOutput::new(&input).to_string(),
            OutputFormat::Json => JsonOutput::new(&input).to_string(),
        }
    );
}
