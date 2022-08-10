use clap::Parser;

/// command-line management interface to kash
#[derive(Parser, Debug)]
#[clap(name = "kash-manage-cli", version, about)]
pub struct Args {}
