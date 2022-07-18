mod cli;

use cli::display::StatementsTable;
use cli::format::{Deserializer, Statement};
use std::io;

fn main() {
    let statements = io::stdin()
        .lines()
        .map(|ln| Deserializer::from_str(&ln.unwrap()).deserialize().unwrap())
        .collect::<Vec<Statement>>();

    print!("{}", StatementsTable::new(&statements));
}
