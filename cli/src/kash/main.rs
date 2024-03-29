mod args;

use self::args::{Args, InputFormat};
use clap::Parser;
use kash_cli::output::OutputOptions;
use kash_convert::input::toml::TomlInput;
use kash_convert::input::{camt053::Camt053Input, json::JsonInput, Input};
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
                InputFormat::Json => JsonInput::new().from_read(reader),
                InputFormat::Toml => TomlInput::new().from_read(reader),
                InputFormat::Camt053 => Camt053Input::new().from_read(reader),
            }
            .unwrap(),
        );
    }

    args.output
        .output_format
        .to_stdout(&input, OutputOptions::from(args.output));
}
