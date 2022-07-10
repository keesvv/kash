use kash::format::{self, Result};
use std::io;

fn main() -> Result<()> {
    for ln in io::stdin().lines() {
        let ln = ln.unwrap();
        let d = format::Deserializer::from_str(ln.as_str());
        println!("{:#?}", d.deserialize()?);
    }
    Ok(())
}
