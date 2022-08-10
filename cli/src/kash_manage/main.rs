mod args;

use self::args::Args;
use clap::Parser;

fn main() {
    Args::parse();
}
