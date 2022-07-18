mod cli;

use cli::display::{StatementsJson, StatementsTable};
use cli::format::{Deserializer, Statement};
use std::env;
use std::io;

fn main() {
    let statements = io::stdin()
        .lines()
        .map(|ln| Deserializer::from_str(&ln.unwrap()).deserialize().unwrap());
    let collect_st = || statements.collect::<Vec<Statement>>();

    // TODO: use clap
    match env::args().nth(1) {
        Some(e) => match e.as_str() {
            "-j" => println!("{}", StatementsJson::new(&collect_st()).to_string()),
            "-h" | _ => eprintln!("usage: {} [-h|-j]", env::args().nth(0).unwrap()),
        },
        _ => println!("{}", StatementsTable::new(&collect_st()).to_string()),
    }
}
