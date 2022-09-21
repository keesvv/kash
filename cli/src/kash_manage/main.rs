mod args;
mod config;

use self::args::{Args, Operation};
use self::config::Config;
use clap::Parser;
use kash_cli::output::OutputOptions;
use kash_repo::{fs::{FsRepo, FsOptions}, repo::RepoLike};
use std::io::ErrorKind;

fn main() {
    let args: Args = Args::parse();
    let config = match Config::parse() {
        Ok(c) => c,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => Ok(Default::default()),
            _ => Err(e),
        }
        .unwrap(),
    };

    let mut repo = FsRepo::new(
        FsOptions {
            path: args
                .repo_dir
                .unwrap_or(config.repo.unwrap_or_default().path.unwrap_or_default()),
            include: args.include,
        },
    );

    repo.reload_store().unwrap();

    match args.op {
        Operation::Show(args) => args.output.output_format.to_stdout(
            &repo.get_all().unwrap(),
            OutputOptions {
                discrete: args.output.discrete,
                currency_symbol: args.output.currency_symbol.unwrap_or(
                    config
                        .output
                        .unwrap_or_default()
                        .currency
                        .unwrap_or(OutputOptions::default().currency_symbol),
                ),
            },
        ),
    };
}
