use kash_cli::display::{StatementsJson, StatementsTable};
use kash_cli::input::{csv::CsvInput, Input};
use std::env;
use std::io;

fn main() {
    let input = CsvInput::new(b',');
    let collect_st = || input.from_read(io::stdin()).unwrap();

    // TODO: use clap
    match env::args().nth(1) {
        Some(e) => match e.as_str() {
            "-j" => println!("{}", StatementsJson::new(&collect_st()).to_string()),
            "-h" | _ => eprintln!("usage: {} [-h|-j]", env::args().nth(0).unwrap()),
        },
        _ => println!("{}", StatementsTable::new(&collect_st()).to_string()),
    }
}
