mod cli;

use kash::{
    format::{self, Result},
    statement::Statement,
};
use std::io;

fn main() -> Result<()> {
    for ln in io::stdin().lines() {
        let ln = ln.unwrap();
        let des = format::Deserializer::from_str(ln.as_str());
        let statement = des.deserialize().unwrap();

        if let Some(table) = match &statement {
            Statement::Fixed(s) => Some(cli::format_fixed(&s)),
            _ => None,
        } {
            print!("{}", table);
        }
    }
    Ok(())
}
