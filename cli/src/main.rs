use kash_cli::output::TableOutput;
use kash_convert::input::{ktf::KtfInput, Input};
use kash_convert::output::{json::JsonOutput, Output};
use std::env;
use std::io;

fn main() {
    let input = KtfInput::new();
    let collect_st = || input.from_read(io::stdin()).unwrap();

    // TODO: use clap
    match env::args().nth(1) {
        Some(e) => match e.as_str() {
            "-j" => println!("{}", JsonOutput::new(&collect_st()).to_string()),
            "-h" | _ => eprintln!("usage: {} [-h|-j]", env::args().nth(0).unwrap()),
        },
        _ => println!("{}", TableOutput::new(&collect_st()).to_string()),
    }
}
